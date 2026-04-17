# ⚙️ Android Quick Settings

Fast Rust CLI to toggle Android settings via ADB — airplane mode, WiFi, Bluetooth, brightness, location, timeouts.

## Install

```bash
cargo install --path .
# Or: cargo build --release
```

## Usage

```bash
aqs airplane              # toggle airplane mode
aqs wifi on               # turn WiFi on
aqs bluetooth off         # turn Bluetooth off
aqs brightness 200        # set brightness (0-255)
aqs timeout 5             # screen timeout 5 minutes
aqs location on           # enable location
aqs status                # show all settings
```

All operations via ADB to connected device. No root needed.
