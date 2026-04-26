#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod state;
mod clipboard;

use tauri::{Manager, Emitter};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, Modifiers, Code, ShortcutState};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Instant, Duration};
use crate::state::AppState;

// Strict toggle lock to prevent "shuttering" loops
static TOGGLING: AtomicBool = AtomicBool::new(false);

fn check_dependencies() {
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if session_type != "wayland" {
        let xclip = std::process::Command::new("which").arg("xclip").output().map(|o| o.status.success()).unwrap_or(false);
        let xdotool = std::process::Command::new("which").arg("xdotool").output().map(|o| o.status.success()).unwrap_or(false);
        if !xclip { eprintln!("⚠️  DEPENDENCY MISSING: 'xclip' not found."); }
        if !xdotool { eprintln!("⚠️  DEPENDENCY MISSING: 'xdotool' not found."); }
    }
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if session_type == "wayland" {
        let wl_paste = std::process::Command::new("which").arg("wl-paste").output().map(|o| o.status.success()).unwrap_or(false);
        let wtype = std::process::Command::new("which").arg("wtype").output().map(|o| o.status.success()).unwrap_or(false);
        if !wl_paste { eprintln!("⚠️  DEPENDENCY MISSING: 'wl-clipboard' not found."); }
        if !wtype { eprintln!("⚠️  DEPENDENCY MISSING: 'wtype' not found."); }
    }
}

#[tauri::command]
async fn get_history(state: tauri::State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    let mut stmt = db.prepare("SELECT id, content, pinned FROM history ORDER BY pinned DESC, timestamp DESC LIMIT 100").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i32>(0)?,
            "content": row.get::<_, String>(1)?,
            "pinned": row.get::<_, i32>(2)? == 1
        }))
    }).map_err(|e| e.to_string())?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row.map_err(|e| e.to_string())?);
    }
    Ok(results)
}

#[tauri::command]
async fn select_item(state: tauri::State<'_, AppState>, handle: tauri::AppHandle, content: String) -> Result<(), String> {
    let window = handle.get_webview_window("main").ok_or("Main window not found")?;
    
    {
        let mut last_paste = state.last_paste_at.lock().map_err(|e| e.to_string())?;
        *last_paste = Instant::now();
    }

    println!("App: Selection made. Hiding window...");
    window.hide().map_err(|e| e.to_string())?;

    clipboard::set_content(content).map_err(|e| e.to_string())?;

    // Safe 1s delay for window focus transition
    tokio::time::sleep(Duration::from_millis(1000)).await;
    
    let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if session_type == "wayland" {
        println!("App: Performing Wayland paste via wtype");
        let status = std::process::Command::new("wtype")
            .arg("-M").arg("ctrl")
            .arg("v")
            .arg("-m").arg("ctrl")
            .status()
            .map_err(|e| format!("Failed to run wtype: {}", e))?;
            
        if !status.success() {
            return Err("wtype failed to perform paste".to_string());
        }
    } else {
        println!("App: Performing X11 paste via xdotool/enigo fallback");
        let status = std::process::Command::new("xdotool")
            .arg("key")
            .arg("ctrl+v")
            .status();

        match status {
            Ok(s) if s.success() => {},
            _ => {
                println!("App: xdotool failed, trying enigo fallback");
                use enigo::{Enigo, Key, KeyboardControllable};
                let mut enigo = Enigo::new();
                enigo.key_down(Key::Control);
                enigo.key_click(Key::Layout('v'));
                enigo.key_up(Key::Control);
            }
        }
    }

    Ok(())
}

#[tauri::command]
async fn delete_item(state: tauri::State<'_, AppState>, id: i32) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db::delete_item(&db, id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn toggle_pin(state: tauri::State<'_, AppState>, id: i32) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db::toggle_pin(&db, id).map_err(|e| e.to_string())
}

fn main() {
    check_dependencies();
    let conn = db::init_db().expect("Failed to init database");
    
    // Simplest possible shortcut: F9
    let global_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state() == ShortcutState::Pressed && shortcut == &global_shortcut {
                        // Strict Toggle Lock
                        if TOGGLING.load(Ordering::SeqCst) {
                            return;
                        }
                        TOGGLING.store(true, Ordering::SeqCst);
                        
                        let state = app.state::<AppState>();
                        
                        // Paste Cooldown
                        {
                            let last_paste = match state.last_paste_at.lock() { Ok(l) => l, Err(_) => { TOGGLING.store(false, Ordering::SeqCst); return; } };
                            if last_paste.elapsed() < Duration::from_millis(1500) {
                                TOGGLING.store(false, Ordering::SeqCst);
                                return;
                            }
                        }

                        if let Some(window) = app.get_webview_window("main") {
                            let is_visible = window.is_visible().unwrap_or(false);
                            if is_visible {
                                println!("Shortcut: Hiding");
                                let _ = window.hide();
                            } else {
                                println!("Shortcut: Showing");
                                let _ = window.show();
                                let _ = app.emit("db-updated", ());
                            }
                        }

                        // Unlock toggle after 500ms
                        let _ = std::thread::spawn(|| {
                            std::thread::sleep(Duration::from_millis(500));
                            TOGGLING.store(false, Ordering::SeqCst);
                        });
                    }
                })
                .build(),
        )
        .manage(AppState { 
            db: Mutex::new(conn),
            last_paste_at: Mutex::new(Instant::now() - Duration::from_secs(10)),
        })
        .setup(move |app| {
            let _ = app.global_shortcut().register(global_shortcut);
            
            let handle = app.handle().clone();

            // System Tray
            let open_i = MenuItem::with_id(&handle, "open", "Open", true, None::<&str>).expect("UI element creation failed");
            let quit_i = MenuItem::with_id(&handle, "quit", "Quit", true, None::<&str>).expect("UI element creation failed");
            let menu = Menu::with_items(&handle, &[&open_i, &quit_i]).expect("UI element creation failed");

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().expect("UI element creation failed").clone())
                .menu(&menu)
                .on_menu_event(|handle, event| match event.id.as_ref() {
                    "quit" => std::process::exit(0),
                    "open" => {
                        if let Some(window) = handle.get_webview_window("main") {
                            let _ = window.show();
                        }
                    }
                    _ => {}
                })
                .build(app).map_err(|e| e.to_string())?;

            // Clipboard Watcher
            let handle_clone = handle.clone();
            clipboard::start_watcher(move |content| {
                let trimmed = content.clone();
                if trimmed.is_empty() { return; }

                let state = handle_clone.state::<AppState>();
                let db = match state.db.lock() {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("Failed to lock db in watcher: {}", e);
                        return;
                    }
                };
                if let Ok(_) = db::insert_item(&db, &trimmed) {
                    let _ = handle_clone.emit("db-updated", ());
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_history, select_item, delete_item, toggle_pin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}