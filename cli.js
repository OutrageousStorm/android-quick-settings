#!/usr/bin/env node
const { execSync } = require("child_process");

const SETTINGS = {
  "brightness": { ns: "system", key: "screen_brightness", range: "0-255" },
  "volume": { ns: "global", key: "volume_music", range: "0-15" },
  "airplane": { ns: "global", key: "airplane_mode_on", values: ["0", "1"] },
  "wifi": { ns: "global", key: "wifi_on", values: ["0", "1"] },
  "bluetooth": { ns: "global", key: "bluetooth_on", values: ["0", "1"] },
  "nfc": { ns: "global", key: "nfc_on", values: ["0", "1"] },
  "location": { ns: "secure", key: "location_mode", range: "0-3" },
  "doze": { ns: "global", key: "low_power", values: ["0", "1"] },
};

function adb(cmd) {
  try {
    return execSync(`adb shell ${cmd}`, { encoding: "utf8" }).trim();
  } catch (e) {
    return "";
  }
}

function set(setting, value) {
  const cfg = SETTINGS[setting];
  if (!cfg) {
    console.error(`Unknown setting: ${setting}`);
    console.log(`Available: ${Object.keys(SETTINGS).join(", ")}`);
    process.exit(1);
  }

  adb(`settings put ${cfg.ns} ${cfg.key} ${value}`);
  console.log(`✓ ${setting} = ${value}`);
}

function get(setting) {
  const cfg = SETTINGS[setting];
  if (!cfg) {
    console.error(`Unknown setting: ${setting}`);
    process.exit(1);
  }
  const val = adb(`settings get ${cfg.ns} ${cfg.key}`);
  console.log(`${setting}: ${val}`);
}

function list() {
  console.log("\n⚙️  Quick Settings List");
  console.log("━".repeat(40));
  Object.entries(SETTINGS).forEach(([name, cfg]) => {
    const val = adb(`settings get ${cfg.ns} ${cfg.key}`);
    console.log(`  ${name.padEnd(15)} = ${val}`);
  });
}

const args = process.argv.slice(2);
if (args.length === 0 || args[0] === "list") {
  list();
} else if (args.length === 2) {
  set(args[0], args[1]);
} else if (args.length === 1) {
  get(args[0]);
}
