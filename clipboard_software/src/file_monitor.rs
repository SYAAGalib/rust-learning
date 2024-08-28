use notify::{watcher, RecursiveMode, Watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::error::Error;

/// Monitors file system changes in a specified directory.
///
/// # Arguments
/// * `path` - The path of the directory to monitor.
///
/// # Returns
/// * `Ok(())` if the directory is successfully monitored.
/// * `Err(String)` with an error message if the operation fails.
pub fn monitor_files(path: &str) -> Result<(), String> {
    // Create a channel to receive file system events
    let (tx, rx) = channel();

    // Create a watcher and configure it
    let mut watcher = watcher(tx, Duration::from_secs(10)).map_err(|e| e.to_string())?;
    
    // Start watching the specified path
    watcher.watch(path, RecursiveMode::Recursive).map_err(|e| e.to_string())?;

    // Handle events
    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Create(path) => println!("File created: {:?}", path),
                DebouncedEvent::Write(path) => println!("File modified: {:?}", path),
                DebouncedEvent::Remove(path) => println!("File removed: {:?}", path),
                DebouncedEvent::Rename(old_path, new_path) => println!("File renamed from {:?} to {:?}", old_path, new_path),
                _ => (),
            },
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}
