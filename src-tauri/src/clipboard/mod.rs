use anyhow::Result;
use std::env;

pub mod wayland;
pub mod x11;

pub trait ClipboardWatcher {
    fn watch(&self, on_change: Box<dyn Fn(String) + Send + Sync>) -> Result<()>;
    fn set_content(&self, content: String) -> Result<()>;
}

pub fn set_content(content: String) -> Result<()> {
    let session_type = env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if session_type == "wayland" {
        wayland::WaylandWatcher.set_content(content)
    } else {
        x11::X11Watcher.set_content(content)
    }
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