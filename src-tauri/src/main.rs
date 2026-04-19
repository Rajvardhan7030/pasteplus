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
    if session_type == "wayland" {
        let wl_paste = std::process::Command::new("which").arg("wl-paste").output().map(|o| o.status.success()).unwrap_or(false);
        let wtype = std::process::Command::new("which").arg("wtype").output().map(|o| o.status.success()).unwrap_or(false);
        if !wl_paste { eprintln!("⚠️  DEPENDENCY MISSING: 'wl-clipboard' not found."); }
        if !wtype { eprintln!("⚠️  DEPENDENCY MISSING: 'wtype' not found."); }
    }
}

#[tauri::command]
async fn get_history(state: tauri::State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT id, content, pinned FROM history ORDER BY timestamp DESC LIMIT 100").unwrap();
    let rows = stmt.query_map([], |row| {
        Ok(serde_json::json!({
            "id": row.get::<_, i32>(0)?,
            "content": row.get::<_, String>(1)?,
            "pinned": row.get::<_, i32>(2)? == 1
        }))
    }).unwrap();

    let mut results = Vec::new();
    for row in rows {
        results.push(row.unwrap());
    }
    Ok(results)
}

#[tauri::command]
async fn select_item(state: tauri::State<'_, AppState>, handle: tauri::AppHandle, content: String) -> Result<(), String> {
    let window = handle.get_webview_window("main").unwrap();
    
    {
        let mut last_paste = state.last_paste_at.lock().unwrap();
        *last_paste = Instant::now();
    }

    println!("App: Selection made. Hiding window...");
    window.hide().unwrap();

    let trimmed_content = content.trim().to_string();
    let _ = clipboard::set_content(trimmed_content);

    std::thread::spawn(move || {
        // Safe 1s delay for window focus transition
        std::thread::sleep(Duration::from_millis(1000));
        
        let session_type = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
        if session_type == "wayland" {
            println!("App: Performing Wayland paste via wtype");
            let _ = std::process::Command::new("wtype")
                .arg("-M").arg("ctrl")
                .arg("v")
                .arg("-m").arg("ctrl")
                .spawn()
                .map(|mut child| child.wait());
        } else {
            println!("App: Performing X11 paste via xdotool/enigo fallback");
            let _ = std::process::Command::new("xdotool")
                .arg("key")
                .arg("ctrl+v")
                .spawn();
        }
    });

    Ok(())
}

#[tauri::command]
async fn delete_item(state: tauri::State<'_, AppState>, id: i32) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db::delete_item(&db, id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn toggle_pin(state: tauri::State<'_, AppState>, id: i32) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db::toggle_pin(&db, id).map_err(|e| e.to_string())
}

fn main() {
    check_dependencies();
    let conn = db::init_db().expect("Failed to init database");
    
    // Simplest possible shortcut: F9
    let f9_shortcut = Shortcut::new(None, Code::F9);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state() == ShortcutState::Pressed && shortcut == &f9_shortcut {
                        // Strict Toggle Lock
                        if TOGGLING.load(Ordering::SeqCst) {
                            return;
                        }
                        TOGGLING.store(true, Ordering::SeqCst);
                        
                        let state = app.state::<AppState>();
                        
                        // Paste Cooldown
                        {
                            let last_paste = state.last_paste_at.lock().unwrap();
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
            let _ = app.global_shortcut().register(f9_shortcut);
            
            let handle = app.handle().clone();

            // System Tray
            let open_i = MenuItem::with_id(&handle, "open", "Open", true, None::<&str>).unwrap();
            let quit_i = MenuItem::with_id(&handle, "quit", "Quit", true, None::<&str>).unwrap();
            let menu = Menu::with_items(&handle, &[&open_i, &quit_i]).unwrap();

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|handle, event| match event.id.as_ref() {
                    "quit" => std::process::exit(0),
                    "open" => {
                        let window = handle.get_webview_window("main").unwrap();
                        let _ = window.show();
                    }
                    _ => {}
                })
                .build(app).unwrap();

            // Clipboard Watcher
            let handle_clone = handle.clone();
            clipboard::start_watcher(move |content| {
                let trimmed = content.trim().to_string();
                if trimmed.is_empty() { return; }

                let state = handle_clone.state::<AppState>();
                let db = state.db.lock().unwrap();
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