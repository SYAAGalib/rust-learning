use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TrayIcon, Menu, MenuItem};

pub fn create_gui(app: &Application) -> ApplicationWindow {
    // Create the main application window (not visible in the tray, just for setup)
    let window = ApplicationWindow::new(app);
    window.set_title("Clipboard Software");
    window.set_default_size(300, 200);
    window.show_all();
    window
}

pub fn create_tray_icon(app: &Application) -> TrayIcon {
    // Create the system tray icon
    let tray_icon = TrayIcon::new("Clipboard Software", "").expect("Failed to create tray icon");
    
    // Create a menu for the tray icon
    let menu = Menu::new();
    
    // Add "Copy Text" item to the menu
    let copy_item = MenuItem::with_label("Copy Text");
    copy_item.connect_activate(move |_| {
        // Placeholder for copy text functionality
        println!("Copy Text clicked");
    });
    menu.add(&copy_item);

    // Add "Take Screenshot" item to the menu
    let screenshot_item = MenuItem::with_label("Take Screenshot");
    screenshot_item.connect_activate(move |_| {
        // Call screenshot function
        crate::screenshot::take_screenshot();
    });
    menu.add(&screenshot_item);

    // Add "Quit" item to the menu
    let quit_item = MenuItem::with_label("Quit");
    quit_item.connect_activate(|_| {
        gtk::main_quit();
    });
    menu.add(&quit_item);
    
    tray_icon.set_menu(&menu);
    tray_icon
}
