use std::sync::Mutex;
use std::time::Instant;
use rusqlite::Connection;

pub struct AppState {
    pub db: Mutex<Connection>,
    pub last_paste_at: Mutex<Instant>,
}