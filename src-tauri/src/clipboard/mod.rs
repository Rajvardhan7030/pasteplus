use anyhow::Result;
use std::env;

pub mod wayland;
pub mod x11;

pub trait ClipboardWatcher {
    fn watch(&self, on_change: Box<dyn Fn(String) + Send + Sync>) -> Result<()>;
}

pub fn start_watcher(on_change: impl Fn(String) + Send + Sync + 'static) {
    let session_type = env::var("XDG_SESSION_TYPE").unwrap_or_default();
    
    std::thread::spawn(move || {
        let callback = Box::new(on_change);
        if session_type == "wayland" {
            let _ = wayland::WaylandWatcher.watch(callback);
        } else {
            let _ = x11::X11Watcher.watch(callback);
        }
    });
}