# Steam Ticket Generator - GUI Version

## Overview

This is a desktop GUI application built with Rust and egui/eframe that wraps the Steam ticket generation functionality.

## Setup

### Option 1: Using the Updated Cargo.toml

1. Replace your `Cargo.toml` with `Cargo_GUI.toml`:
   ```bash
   cp Cargo_GUI.toml Cargo.toml
   ```

2. Install dependencies:
   ```bash
   cargo build --release
   ```

### Option 2: Adding GUI to Existing Setup

Add these dependencies to your existing `Cargo.toml`:

```toml
[dependencies]
eframe = "0.30"
egui = "0.30"

[[bin]]
name = "steam-ticket-gui"
path = "src/gui_main.rs"
```

## Building

### Build GUI version:
```bash
cargo build --release --bin steam-ticket-gui
```

### Build CLI version (original):
```bash
cargo build --release --bin steam-ticket-cli
```

## Running

### Run GUI:
```bash
cargo run --release --bin steam-ticket-gui
```

Or directly run the executable:
```bash
./target/release/steam-ticket-gui.exe  # Windows
./target/release/steam-ticket-gui      # Linux/Mac
```

### Run CLI (original):
```bash
cargo run --release --bin steam-ticket-cli
```

## GUI Features

- **App ID Input**: Enter the Steam App ID for the game
- **Generate Button**: Click to generate the encrypted app ticket
- **Status Log**: Real-time feedback and error messages
- **Save to File**: Save the generated ticket to `configs.user.ini`
- **Loading Indicator**: Shows when ticket generation is in progress

## Usage Instructions

1. **Start Steam**: Make sure the Steam client is running and you're logged in
2. **Enter App ID**: Input the App ID of the game (e.g., 480 for Spacewar)
3. **Click Generate**: The application will:
   - Initialize the Steam API
   - Request an encrypted app ticket
   - Display your Steam ID and the generated ticket
4. **Save to File**: Click "Save to File" to create `configs.user.ini` with the ticket data

## Architecture

The code is structured to keep GUI and logic separated:

### GUI Layer (`TicketGeneratorApp`)
- Handles user input and validation
- Manages UI state and rendering
- Spawns background threads for ticket generation
- Updates UI based on generation results

### Core Logic Layer
- `generate_ticket_core()`: Main function that interfaces with Steam API
- `run_callbacks()`: Processes Steam API callbacks
- `create_config_file()`: Saves ticket data to file

### Threading Model
- UI runs on the main thread (required by egui)
- Ticket generation runs in a background thread (prevents UI blocking)
- Shared state using `Arc<Mutex<>>` for thread-safe communication

## Error Handling

The GUI provides clear error messages for common issues:
- Invalid App ID format
- Steam client not running
- Failed to initialize Steam API
- Account doesn't own the game
- File save errors

## Requirements

- **Rust**: 1.70 or later
- **Steam Client**: Must be running and logged in
- **Account Ownership**: Must own the game for the specified App ID

## File Structure

```
src/
├── main.rs          # Original CLI version
└── gui_main.rs      # New GUI version
```

## Customization

### Changing Window Size
Edit the `NativeOptions` in `gui_main.rs`:
```rust
.with_inner_size([600.0, 500.0])  // width, height
.with_min_inner_size([500.0, 400.0])
```

### Adding an Application Icon
Replace the icon data in `gui_main.rs`:
```rust
.with_icon(
    eframe::icon_data::from_png_bytes(include_bytes!("../icon.png"))
        .expect("Failed to load icon"),
)
```

### Customizing the Theme
egui uses a default theme, but you can customize it in the `update()` method:
```rust
ctx.set_visuals(egui::Visuals::dark());  // or ::light()
```

## Troubleshooting

### "Steam client is not running"
- Make sure Steam is open and you're logged in

### "Failed to get encrypted app ticket"
- Verify you own the game
- Try restarting Steam
- Check the App ID is correct

### GUI doesn't start
- Ensure all dependencies are properly installed
- Check GPU drivers are up to date (egui uses GPU rendering)

## License

Same license as the original steam-ticket-generator project.
