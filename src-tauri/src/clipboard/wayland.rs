use super::ClipboardWatcher;
use anyhow::{Result, anyhow};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub struct WaylandWatcher;

impl ClipboardWatcher for WaylandWatcher {
    fn watch(&self, on_change: Box<dyn Fn(String) + Send + Sync>) -> Result<()> {
        let mut last_content = String::new();

        // Try to use wl-paste --watch
        let mut child = Command::new("wl-paste")
            .arg("--watch")
            .arg("sh")
            .arg("-c")
            .arg("echo _CHANGE_")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                eprintln!("CRITICAL ERROR: 'wl-paste' not found. Please install 'wl-clipboard'. Error: {}", e);
                anyhow!("wl-clipboard not installed?")
            })?;

        // Give it a moment to see if it crashes (common on GNOME)
        std::thread::sleep(std::time::Duration::from_millis(200));

        let mut fallback_polling = false;
        if let Ok(Some(status)) = child.try_wait() {
            eprintln!("wl-paste --watch exited with status {}. Falling back to polling mode.", status);
            fallback_polling = true;
        }

        if fallback_polling {
            loop {
                let output = Command::new("wl-paste")
                    .arg("-n")
                    .output();
                
                if let Ok(out) = output {
                    let current = String::from_utf8_lossy(&out.stdout).to_string();
                    if !current.is_empty() && current != last_content {
                        on_change(current.clone());
                        last_content = current;
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        } else {
            let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to open stdout"))?;
            let reader = BufReader::new(stdout);

            for line in reader.lines() {
                if let Ok(_) = line {
                    let output = Command::new("wl-paste")
                        .arg("-n")
                        .output();
                    
                    if let Ok(out) = output {
                        let content = String::from_utf8_lossy(&out.stdout).to_string();
                        if !content.trim().is_empty() {
                            on_change(content);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn set_content(&self, content: String) -> Result<()> {
        use std::io::Write;
        let mut child = Command::new("wl-copy")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|_| anyhow!("wl-copy not installed?"))?;

        let mut stdin = child.stdin.take().ok_or_else(|| anyhow!("Failed to open stdin"))?;
        stdin.write_all(content.as_bytes())?;
        drop(stdin);
        child.wait()?;
        Ok(())
    }
}