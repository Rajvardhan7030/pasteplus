#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod state;
mod clipboard;

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu, SystemTrayEvent, GlobalShortcutManager};
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
    let window = handle.get_window("main").unwrap();
    window.hide().unwrap();

    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));
        
        let mut enigo = Enigo::new();
        enigo.key_down(Key::Control);
        enigo.key_click(Key::Layout('v'));
        enigo.key_up(Key::Control);
    });

    Ok(())
}

fn main() {
    let conn = db::init_db().expect("Failed to init database");
    
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("open", "Open"))
        .add_item(CustomMenuItem::new("quit", "Quit"));

    tauri::Builder::default()
        .manage(AppState { db: Mutex::new(conn) })
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => std::process::exit(0),
                "open" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            let handle = app.handle();
            let state = handle.state::<AppState>();

            clipboard::start_watcher(move |content| {
                let db = state.db.lock().unwrap();
                let _ = db::insert_item(&db, &content);
                handle.emit_all("db-updated", ()).unwrap();
            });

            let app_handle = app.handle();
            app.global_shortcut_manager().register("Ctrl+Shift+V", move || {
                let window = app_handle.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_history, select_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}