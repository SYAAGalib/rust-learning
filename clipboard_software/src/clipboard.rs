use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;

/// Retrieves the current text from the clipboard.
///
/// # Returns
/// * `Ok(String)` containing the clipboard text if successful.
/// * `Err(String)` with an error message if the operation fails.
pub fn get_clipboard_text() -> Result<String, String> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|e| e.to_string())?;
    ctx.get_contents().map_err(|e| e.to_string())
}

/// Sets the given text to the clipboard.
///
/// # Arguments
/// * `text` - A string slice that holds the text to be set to the clipboard.
///
/// # Returns
/// * `Ok(())` if the operation is successful.
/// * `Err(String)` with an error message if the operation fails.
pub fn set_clipboard_text(text: &str) -> Result<(), String> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().map_err(|e| e.to_string())?;
    ctx.set_contents(text.to_string()).map_err(|e| e.to_string())
}
