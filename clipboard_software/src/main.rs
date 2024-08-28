use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Image};
use gdk_pixbuf::Pixbuf;
use crate::gui::{create_gui, create_tray_icon};
use crate::clipboard::get_clipboard_text;
use crate::screenshot::take_screenshot;
use crate::file_monitor::monitor_files;
use std::error::Error;
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize GTK application
    let app = Application::new(Some("com.example.clipboard_software"), Default::default())?;
    
    // Create the main GUI window (hidden)
    let window = create_gui(&app);

    // Load and set the icon for the main window
    let icon_data = include_bytes!("../src/assets/clipboard_software.png");
    let pixbuf = Pixbuf::from_bytes(
        icon_data,
        gdk_pixbuf::Colorspace::Rgb,
        false,
        8,
        48, // Adjust width if needed
        48, // Adjust height if needed
        48 * 3, // Rowstride
    )
    .expect("Failed to create pixbuf from icon data");
    window.set_icon(&pixbuf);

    // Create the system tray icon
    let tray_icon = create_tray_icon(&app);

    // Start file monitoring in a separate thread
    thread::spawn(|| {
        monitor_files();
    });
    
    // Add a callback to handle clipboard actions
    let clipboard_action = || {
        match get_clipboard_text() {
            Ok(text) => println!("Clipboard text: {}", text),
            Err(err) => println!("Error getting clipboard text: {}", err),
        }
    };

    // Set up a periodic task to check clipboard content
    thread::spawn(move || {
        loop {
            clipboard_action();
            thread::sleep(Duration::from_secs(5)); // Check every 5 seconds
        }
    });
    
    // Optionally, take a screenshot on startup
    // Uncomment if you want to take a screenshot on startup
    // take_screenshot(); 

    // Run the GTK main loop
    app.run();
    
    Ok(())
}
