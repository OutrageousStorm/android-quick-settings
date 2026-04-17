#!/bin/bash
# quick-settings.sh -- Android quick settings toggles via ADB
set -e

cmd="${1:-help}"
arg="${2}"

case "$cmd" in
  airplane-on)
    adb shell settings put global airplane_mode_on 1
    adb shell am broadcast -a android.intent.action.AIRPLANE_MODE
    echo "✈️  Airplane mode ON"
    ;;
  airplane-off)
    adb shell settings put global airplane_mode_on 0
    adb shell am broadcast -a android.intent.action.AIRPLANE_MODE
    echo "✈️  Airplane mode OFF"
    ;;
  wifi-on)
    adb shell cmd wifi set-wifi-enabled enabled
    echo "📶 WiFi ON"
    ;;
  wifi-off)
    adb shell cmd wifi set-wifi-enabled disabled
    echo "📶 WiFi OFF"
    ;;
  bluetooth-on)
    adb shell service call bluetooth_manager 6
    echo "🔵 Bluetooth ON"
    ;;
  bluetooth-off)
    adb shell service call bluetooth_manager 8
    echo "🔵 Bluetooth OFF"
    ;;
  dev-on)
    adb shell settings put global development_settings_enabled 1
    echo "🔧 Developer options ON"
    ;;
  dev-off)
    adb shell settings put global development_settings_enabled 0
    echo "🔧 Developer options OFF"
    ;;
  adb-on)
    adb shell settings put global adb_enabled 1
    echo "🐛 ADB debugging ON"
    ;;
  adb-off)
    adb shell settings put global adb_enabled 0
    echo "🐛 ADB debugging OFF"
    ;;
  screen-timeout)
    timeout=$((arg * 1000))
    adb shell settings put system screen_off_timeout $timeout
    echo "⏱️  Screen timeout: ${arg}s"
    ;;
  brightness)
    adb shell settings put system screen_brightness $arg
    adb shell settings put system screen_brightness_mode 0
    echo "☀️  Brightness: $arg/255"
    ;;
  animations-off)
    adb shell settings put global window_animation_scale 0
    adb shell settings put global transition_animation_scale 0
    adb shell settings put global animator_duration_scale 0
    echo "⚡ Animations OFF"
    ;;
  animations-on)
    adb shell settings put global window_animation_scale 1
    adb shell settings put global transition_animation_scale 1
    adb shell settings put global animator_duration_scale 1
    echo "⚡ Animations ON"
    ;;
  usb-tether-on)
    adb shell cmd connectivity tethering start usb
    echo "🔌 USB tethering ON"
    ;;
  usb-tether-off)
    adb shell cmd connectivity tethering stop usb
    echo "🔌 USB tethering OFF"
    ;;
  help)
    echo "Usage: quick-settings.sh <command>"
    echo "Commands: airplane-on/off, wifi-on/off, bluetooth-on/off, dev-on/off"
    echo "          adb-on/off, screen-timeout N, brightness N, animations-on/off"
    echo "          usb-tether-on/off"
    ;;
  *)
    echo "Unknown command: $cmd"
    exit 1
    ;;
esac
