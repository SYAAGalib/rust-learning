use screenshot::{get_screenshot, CapturedImage};
use std::fs::File;
use std::path::Path;
use image::ImageFormat;

/// Takes a screenshot of the entire screen and saves it to a file.
///
/// # Arguments
/// * `file_path` - The path where the screenshot will be saved.
///
/// # Returns
/// * `Ok(())` if the screenshot is successfully saved.
/// * `Err(String)` with an error message if the operation fails.
pub fn take_screenshot(file_path: &str) -> Result<(), String> {
    // Capture the screen
    let screenshot: CapturedImage = get_screenshot().map_err(|e| e.to_string())?;
    
    // Convert CapturedImage to image::DynamicImage
    let image = image::DynamicImage::ImageRgb8(screenshot.to_image().map_err(|e| e.to_string())?);

    // Save the image to the specified file path
    let file = File::create(Path::new(file_path)).map_err(|e| e.to_string())?;
    image.save_with_format(file, ImageFormat::Png).map_err(|e| e.to_string())?;

    Ok(())
}
