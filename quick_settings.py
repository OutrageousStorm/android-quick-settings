#!/usr/bin/env python3
"""quick_settings.py - Rapid Android settings bulk modifier"""
import subprocess, sys, argparse

PRESETS = {
    "gaming": [
        ("global", "low_power", "0"),
        ("global", "airplane_mode_on", "0"),
        ("secure", "location_mode", "1"),
        ("system", "screen_brightness", "255"),
        ("system", "screen_brightness_mode", "0"),
        ("global", "wifi_on", "1"),
    ],
    "battery": [
        ("global", "low_power", "1"),
        ("global", "wifi_scan_always_enabled", "0"),
        ("global", "ble_scan_always_enabled", "0"),
        ("system", "screen_brightness", "100"),
        ("global", "airplane_mode_on", "0"),
    ],
    "sleep": [
        ("global", "low_power", "1"),
        ("global", "airplane_mode_on", "1"),
        ("secure", "location_mode", "0"),
        ("system", "screen_off_timeout", "15000"),
    ],
    "meeting": [
        ("global", "airplane_mode_on", "1"),
        ("system", "notification_sound", ""),
        ("system", "vibrate_on_notification", "0"),
    ],
    "privacy": [
        ("global", "limit_ad_tracking", "1"),
        ("secure", "location_mode", "0"),
        ("global", "wifi_scan_always_enabled", "0"),
    ],
}

def adb(cmd):
    subprocess.run(f"adb shell {cmd}", shell=True, capture_output=True)

def apply_preset(name):
    if name not in PRESETS:
        print(f"Unknown preset: {name}")
        print(f"Available: {', '.join(PRESETS.keys())}")
        return
    
    print(f"📱 Applying preset: {name}\n")
    for ns, key, val in PRESETS[name]:
        adb(f"settings put {ns} {key} {val}")
        print(f"  ✓ {key} = {val}")
    print("\n✅ Done")

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("preset", nargs='?', help="gaming|battery|sleep|meeting|privacy")
    parser.add_argument("--list", action="store_true", help="List all presets")
    args = parser.parse_args()
    
    if args.list:
        print("Available presets:")
        for name in PRESETS:
            print(f"  {name}")
        return
    
    if args.preset:
        apply_preset(args.preset)
    else:
        parser.print_help()

if __name__ == "__main__":
    main()
