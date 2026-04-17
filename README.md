# ⚙️ Android Quick Settings

Blazing-fast Bash CLI to toggle Android settings without the slow UI.

## Install
```bash
curl https://raw.githubusercontent.com/OutrageousStorm/android-quick-settings/main/quick-settings -o /usr/local/bin/quick-settings
chmod +x /usr/local/bin/quick-settings
```

## Usage
```bash
quick-settings brightness 200      # Set brightness
quick-settings airplane on         # Enable airplane mode
quick-settings wifi off            # Disable WiFi
quick-settings doze                # Force deep doze
quick-settings list                # Show all commands
```

## Features
- Single bash script, instant execution
- No Python, no dependencies
- Works on any Linux/Mac with ADB
