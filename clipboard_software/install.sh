#!/bin/bash

# Variables
PROJECT_DIR="$HOME/code/Rust/Clipboard_Software"  # Adjusted path
BIN_DIR="/usr/local/bin"
DESKTOP_DIR="$HOME/.config/autostart"
SCRIPT_NAME="clipboard_software"

# Build the application
cd "$PROJECT_DIR" || exit
cargo build --release

# Install the binary
sudo cp "./target/release/$SCRIPT_NAME" "$BIN_DIR/$SCRIPT_NAME"

# Create autostart directory if it does not exist
mkdir -p "$DESKTOP_DIR"

# Create a .desktop file for autostart
AUTOSTART_FILE="$DESKTOP_DIR/$SCRIPT_NAME.desktop"
echo "[Desktop Entry]" | sudo tee "$AUTOSTART_FILE"
echo "Type=Application" | sudo tee -a "$AUTOSTART_FILE"
echo "Exec=$BIN_DIR/$SCRIPT_NAME" | sudo tee -a "$AUTOSTART_FILE"
echo "Hidden=false" | sudo tee -a "$AUTOSTART_FILE"
echo "NoDisplay=false" | sudo tee -a "$AUTOSTART_FILE"
echo "X-GNOME-Autostart-enabled=true" | sudo tee -a "$AUTOSTART_FILE"
echo "Name=Clipboard Software" | sudo tee -a "$AUTOSTART_FILE"

# Create a wrapper script to run the application
WRAPPER_SCRIPT="/usr/local/bin/run_clipboard.sh"
echo "#!/bin/bash" | sudo tee "$WRAPPER_SCRIPT"
echo "cd $PROJECT_DIR || exit" | sudo tee -a "$WRAPPER_SCRIPT"
echo "./target/release/$SCRIPT_NAME" | sudo tee -a "$WRAPPER_SCRIPT"

# Make the wrapper script executable
sudo chmod +x "$WRAPPER_SCRIPT"

echo "Installation complete!"
