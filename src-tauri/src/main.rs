#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod state;
mod clipboard;

use tauri::{Manager, Emitter};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, Modifiers, Code, ShortcutState};
use enigo::{Enigo, Key, KeyboardControllable};
use std::sync::Mutex;
use crate::state::AppState;

#[tauri::command]
async fn get_history(state: tauri::State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    let db = state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT id, content, pinned FROM history ORDER BY timestamp DESC").unwrap();
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
async fn select_item(handle: tauri::AppHandle, content: String) -> Result<(), String> {
    let window = handle.get_webview_window("main").unwrap();
    window.hide().unwrap();

    clipboard::set_content(content).map_err(|e| e.to_string())?;

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));
        
        let mut enigo = Enigo::new();
        enigo.key_down(Key::Control);
        enigo.key_click(Key::Layout('v'));
        enigo.key_up(Key::Control);
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
    let conn = db::init_db().expect("Failed to init database");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        let ctrl_shift_v = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);
                        if shortcut == &ctrl_shift_v {
                            if let Some(window) = app.get_webview_window("main") {
                                window.show().unwrap();
                                window.set_focus().unwrap();
                            }
                        }
                    }
                })
                .build(),
        )
        .manage(AppState { db: Mutex::new(conn) })
        .setup(|app| {
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
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                    _ => {}
                })
                .on_tray_icon_event(|_tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
                        // Optional: Handle tray click
                    }
                })
                .build(app).unwrap();

            // Clipboard Watcher
            let handle_clone = handle.clone();
            clipboard::start_watcher(move |content| {
                let state = handle_clone.state::<AppState>();
                let db = state.db.lock().unwrap();
                let _ = db::insert_item(&db, &content);
                handle_clone.emit("db-updated", ()).unwrap();
            });

            // Register Global Shortcut
            let ctrl_shift_v = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);
            app.global_shortcut().register(ctrl_shift_v).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_history, select_item, delete_item, toggle_pin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}