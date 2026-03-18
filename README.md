# Steam Ticket Generator

This project provides an implementation of a encrypted app ticket generator for Steam. The generated ticket can then be used to run games that require a valid ticket to check for game ownership (ex. Denuvo protected games).

**Available in two versions:**
- **CLI** - Command-line interface (original version)
- **GUI** - Desktop application with graphical interface using egui/eframe

**Note for Denuvo games:**
 - Denuvo protected games will also require to have the correct steam account id in the steam emulator settings.
 - Each Steam account can achieve at most 5 activations a day.
 - An EncryptedAppTicket expires after 30 minutes and can be used multiple times in that time span, using the same ticket won't bypass the 5 daily activations limit.

## Quick Start

### GUI Version (Recommended for most users)

1. **Build the GUI:**
    ```sh
    cargo build --release --bin steam-ticket-gui
    ```

2. **Place steam_api64.dll:**
    Copy `steam_api64.dll` to the same directory as `steam-ticket-gui.exe` (in `target/release/`).

3. **Run the GUI:**
    ```sh
    .\target\release\steam-ticket-gui.exe
    ```
    Or simply double-click the executable.

4. **Generate a ticket:**
    - Make sure Steam is running and you're logged in
    - Enter the game's App ID in the input field
    - Click "Generate Ticket"
    - Wait for the success message
    - Click "Save to File" to create `configs.user.ini`

### CLI Version (Original)

1. **Build the CLI:**
    ```sh
    cargo build --release --bin steam-ticket-cli
    ```

2. **Place steam_api64.dll:**
    Copy `steam_api64.dll` to the same directory as `steam-ticket-cli.exe` (in `target/release/`).

3. **Run the CLI:**
    ```sh
    .\target\release\steam-ticket-cli.exe
    ```
    Open Steam on your computer, log in with the account you wish to use for the generation, then run the program.
    Input the game's AppID when prompted. The program will use the currently logged in account to generate the ticket.
    It will output both the user's SteamID and the generated ticket in base64 format.

## Using the Generated Ticket

The generated ticket can be used with [gbe_fork](https://github.com/Detanup01/gbe_fork).

**Option 1: Using GUI** - Click "Save to File" and the `configs.user.ini` file will be created automatically.

**Option 2: Manual creation** - Copy the generated SteamID and ticket to `configs.user.ini`:
```ini
[user::general]
account_steamid=YOUR_STEAM_ID
ticket=BASE64_ENCODED_TICKET
```

## GUI Features

The GUI version provides a user-friendly interface with:
- **Input validation** - Ensures App ID is valid before generating
- **Real-time status updates** - Shows progress and clear error messages
- **Background processing** - UI remains responsive during generation
- **One-click save** - Automatically creates the `configs.user.ini` file
- **Clean interface** - Minimal, easy-to-use design

**Error handling:**
- Validates Steam client is running
- Checks for valid App ID format
- Provides clear feedback for common issues

## Requirements

- **Steam Client** - Must be running and logged in with the account you want to use
- **steam_api64.dll** - Required to communicate with the Steam client (included in releases)
- **Game Ownership** - The logged-in account must own the game for the specified App ID
- **Rust** (for building from source) - Version 1.70 or later recommended

## Builds

Builds are available in the [releases](https://github.com/denuvosanctuary/steam-ticket-generator/releases) section of the repository. Nightly builds are also available in the [actions](https://github.com/denuvosanctuary/steam-ticket-generator/actions) section.

**Available binaries:**
- `steam-ticket-gui.exe` / `steam-ticket-gui` - GUI version (recommended)
- `steam-ticket-cli.exe` / `steam-ticket-cli` - CLI version (original)

The builds in the releases section will also include the `steam_api64.dll` file required to run the program. Otherwise you can download it from the [Steamworks SDK](https://partner.steamgames.com/doc/sdk). The minimum required version is 1.62.

**Note:** When running from releases, make sure `steam_api64.dll` (or `libsteam_api.so` on Linux) is in the same directory as the executable.

## Project Structure

```
steam-ticket-generator/
├── src/
│   ├── main.rs       # CLI version
│   └── gui_main.rs   # GUI version
├── Cargo.toml        # Build configuration
├── README.md         # This file
└── GUI_README.md     # Detailed GUI documentation
```

## Troubleshooting

### "Steam client is not running"
- Make sure Steam is open and you're logged in
- The Steam client must be running before generating a ticket

### "Failed to get encrypted app ticket"
- Verify that your account owns the game for the specified App ID
- Try restarting Steam
- Check that the App ID is correct

### "cargo: command not found"
- Install Rust from https://rustup.rs/
- Make sure cargo is in your system PATH

### GUI won't start
- Check that GPU drivers are up to date (egui requires GPU rendering)
- Ensure all dependencies compiled successfully

### Missing steam_api64.dll
- Download from [Steamworks SDK](https://partner.steamgames.com/doc/sdk) (version 1.62+)
- Place in the same directory as the executable

## Linux

Linux builds are also now available in both [releases](https://github.com/denuvosanctuary/steam-ticket-generator/releases) and [actions](https://github.com/denuvosanctuary/steam-ticket-generator/actions). The required `libsteam_api.so` is already included in the releases.

**Building on Linux:**
```sh
# GUI version
cargo build --release --bin steam-ticket-gui

# CLI version
cargo build --release --bin steam-ticket-cli
```

**Running on Linux:**
```sh
# Make sure libsteam_api.so is in the same directory
./target/release/steam-ticket-gui
# or
./target/release/steam-ticket-cli
```

## For More Information

- **GUI Documentation** - See [GUI_README.md](GUI_README.md) for detailed information about the GUI version, including architecture, customization, and advanced features
- **Issues & Support** - Report bugs or request features in the [GitHub Issues](https://github.com/denuvosanctuary/steam-ticket-generator/issues)

## Disclaimer

This project is for educational and research purposes only. Use responsibly and respect software licenses.
