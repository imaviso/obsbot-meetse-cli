# Obsbot Meet SE CLI

A command-line interface for controlling the **Obsbot Meet SE** webcam on Linux. This tool allows you to adjust image settings (brightness, contrast, etc.), camera controls (zoom, focus), and AI features (auto-framing, background blur) via the Obsbot SDK.

## Features

-   **Device Info**: List connected devices and retrieve serial numbers/firmware versions.
-   **Image Controls**: Adjust Brightness, Contrast, Saturation, Hue, Sharpness.
-   **White Balance**: Toggle Auto White Balance or set Manual Color Temperature (2000K-8000K).
-   **Camera Controls**: Digital Zoom (1x-4x), Auto/Manual Focus.
-   **AI Features**: Control Auto-Framing (Group/Single), Media Modes (Background/Normal).
-   **System**: Anti-Flicker settings and Factory Reset.
-   **Reliability**: Includes robust retry logic to handle SDK device detection delays.

## Prerequisites

-   **Linux** (Tested on NixOS)
-   **Nix** (with Flakes enabled)
-   **Obsbot SDK** (Expected in `sdk/` directory, already included in this repo structure)

## Build & Run

This project uses **Nix Flakes** to provide a reproducible development environment with all necessary dependencies (`clang`, `libdev.so`, etc.).

1.  **Enter the Development Shell**:
    ```bash
    nix develop
    ```

2.  **Build the Project**:
    ```bash
    cargo build
    ```

3.  **Run the CLI**:
    ```bash
    ./target/debug/obsbot-cli --help
    ```

> **Note**: `nix build` (standalone build) currently faces linking issues with the precompiled SDK in the sandbox. Please use `nix develop` + `cargo build` for now.

## Usage

### List Devices
```bash
./target/debug/obsbot-cli list
```

### Image Controls
```bash
# Set Saturation to 0 (B&W)
./target/debug/obsbot-cli image --saturation 0

# Set Brightness (0-100)
./target/debug/obsbot-cli image --brightness 60

# Manual White Balance (4000K)
./target/debug/obsbot-cli image --wb-auto false --wb-temp 4000

# Background Blur (Requires 'Background' mode)
./target/debug/obsbot-cli mode background
./target/debug/obsbot-cli image --blur 100
```

### Camera Controls
```bash
# Zoom In (2.0x)
./target/debug/obsbot-cli camera --zoom 2.0

# Manual Focus
./target/debug/obsbot-cli camera --focus-auto false --focus 80

# Anti-Flicker (50Hz)
./target/debug/obsbot-cli camera --anti-flicker 1
```

### System
```bash
# Factory Reset
./target/debug/obsbot-cli reset
```

## Troubleshooting

-   **"No device found"**:
    -   The CLI will retry detection for up to 10 seconds.
    -   Ensure the camera is plugged in and recognized by `lsusb`.
    -   Ensure you have permissions to access video devices (e.g., usually part of the `video` group).

## Project Structure

-   `src/main.rs`: Rust CLI entry point and command parsing.
-   `wrapper/`: C++ wrapper around the official Obsbot SDK (`libdev.so`).
-   `sdk/`: Official Obsbot SDK headers and shared libraries.
-   `flake.nix`: Nix development environment configuration.

## License

The code for `obsbot-cli` is released under the **MIT License**. See [LICENSE](LICENSE) for details.

> **Important**: This project depends on the **Obsbot SDK**, which is proprietary software owned by OBSBOT. The SDK files included in or linked by this project are subject to their own proprietary license terms and are **not** covered by the MIT License of this CLI tool.
