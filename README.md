# ⚙️ Android Quick Settings

Rust CLI tool to quickly toggle Android settings via ADB. Fast, no-dependency binary.

```bash
cargo build --release
./target/release/aqs brightness 180
./target/release/aqs wifi on
./target/release/aqs location off
./target/release/aqs airplane-mode toggle
./target/release/aqs settings list
```

## Supported toggles
- brightness [0-255]
- wifi [on|off|toggle]
- bluetooth [on|off|toggle]
- location [on|off|toggle]
- airplane-mode [on|off|toggle]
- doze [on|off]
- animations [on|off|toggle]
