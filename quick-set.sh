#!/bin/bash
# quick-set.sh -- Rapid Android settings tweaker
# Usage: ./quick-set.sh brightness 128
#        ./quick-set.sh wifi off
#        ./quick-set.sh list

SETTINGS=(
  "brightness:system:screen_brightness:0-255"
  "wifi:global:wifi_on:0-1"
  "bluetooth:global:bluetooth_on:0-1"
  "location:secure:location_mode:0-3"
  "autobright:system:screen_brightness_mode:0-1"
  "doze:global:low_power:0-1"
  "animations:global:animator_duration_scale:0-10"
  "font:system:font_scale:0.85-1.15"
  "rotation:system:accelerometer_rotation:0-1"
  "adb:global:adb_enabled:0-1"
  "developer:global:development_settings_enabled:0-1"
  "backup:global:backup_auto_restore:0-1"
)

if [[ -z "$1" ]] || [[ "$1" == "list" ]]; then
  echo "⚙️  Android Quick Settings"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  for setting in "${SETTINGS[@]}"; do
    IFS=":" read -r name ns key range <<< "$setting"
    val=$(adb shell settings get $ns $key 2>/dev/null)
    echo "  $name: $val ($range)"
  done
  exit 0
fi

# Parse arguments
NAME="$1"
VALUE="$2"

# Find setting
for setting in "${SETTINGS[@]}"; do
  IFS=":" read -r sname ns key range <<< "$setting"
  if [[ "$sname" == "$NAME" ]]; then
    adb shell settings put $ns $key $VALUE
    echo "✓ $sname set to $VALUE"
    exit 0
  fi
done

echo "Unknown setting: $NAME"
echo "Run: $0 list"
