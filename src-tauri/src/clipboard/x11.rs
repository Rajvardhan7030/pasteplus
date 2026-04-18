use super::ClipboardWatcher;
use anyhow::Result;
use std::process::Command;
use std::thread;
use std::time::Duration;

pub struct X11Watcher;

impl ClipboardWatcher for X11Watcher {
    fn watch(&self, on_change: Box<dyn Fn(String) + Send + Sync>) -> Result<()> {
        let mut last_content = String::new();

        loop {
            let output = Command::new("xclip")
                .arg("-selection")
                .arg("clipboard")
                .arg("-o")
                .output();

            if let Ok(out) = output {
                let current = String::from_utf8_lossy(&out.stdout).to_string();
                if !current.is_empty() && current != last_content {
                    on_change(current.clone());
                    last_content = current;
                }
            }
            
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn set_content(&self, content: String) -> Result<()> {
        use std::io::Write;
        use std::process::Stdio;
        let mut child = Command::new("xclip")
            .arg("-selection")
            .arg("clipboard")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|_| anyhow::anyhow!("xclip not installed?"))?;

        let mut stdin = child.stdin.take().ok_or_else(|| anyhow::anyhow!("Failed to open stdin"))?;
        stdin.write_all(content.as_bytes())?;
        drop(stdin);
        child.wait()?;
        Ok(())
    }
}