use std::process::Command;
use std::env;

fn adb(cmd: &str) -> String {
    let output = Command::new("adb")
        .args(&["shell", cmd])
        .output()
        .expect("Failed to execute adb");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn set_brightness(level: u8) {
    adb(&format!("settings put system screen_brightness {}", level.min(255)));
    println!("✓ Brightness set to {}", level);
}

fn toggle_wifi(state: &str) {
    let val = match state {
        "on" => "1",
        "off" => "0",
        "toggle" => {
            let status = adb("settings get global wifi_on");
            if status.contains("1") { "0" } else { "1" }
        },
        _ => return println!("Usage: aqs wifi [on|off|toggle]"),
    };
    adb(&format!("cmd wifi set-wifi-enabled {}", val == "1"));
    println!("✓ WiFi {}", if val == "1" { "ON" } else { "OFF" });
}

fn toggle_bluetooth(state: &str) {
    let val = match state {
        "on" => "1",
        "off" => "0",
        "toggle" => {
            let status = adb("settings get global bluetooth_on");
            if status.contains("1") { "0" } else { "1" }
        },
        _ => return println!("Usage: aqs bluetooth [on|off|toggle]"),
    };
    adb(&format!("svc bluetooth {'{}' if val == \"1\" else '{}'}"));
    println!("✓ Bluetooth {}", if val == "1" { "ON" } else { "OFF" });
}

fn toggle_location(state: &str) {
    let val = match state {
        "on" => "1",
        "off" => "0",
        _ => return println!("Usage: aqs location [on|off]"),
    };
    adb(&format!("settings put secure location_mode {}", val));
    println!("✓ Location {}", if val == "1" { "ON" } else { "OFF" });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Android Quick Settings CLI");
        println!("Usage: aqs <command> [value]");
        println!("\nCommands:");
        println!("  brightness <0-255>  Set screen brightness");
        println!("  wifi [on|off|toggle]");
        println!("  bluetooth [on|off|toggle]");
        println!("  location [on|off]");
        println!("  airplane-mode [on|off|toggle]");
        println!("  doze [on|off]");
        return;
    }

    match args[1].as_str() {
        "brightness" => {
            if let Ok(level) = args.get(2).unwrap_or(&"0".to_string()).parse::<u8>() {
                set_brightness(level);
            }
        },
        "wifi" => toggle_wifi(args.get(2).map(|s| s.as_str()).unwrap_or("toggle")),
        "bluetooth" => toggle_bluetooth(args.get(2).map(|s| s.as_str()).unwrap_or("toggle")),
        "location" => toggle_location(args.get(2).map(|s| s.as_str()).unwrap_or("on")),
        "airplane-mode" => {
            let val = match args.get(2).map(|s| s.as_str()) {
                Some("on") => "1",
                Some("off") => "0",
                Some("toggle") => {
                    let status = adb("settings get global airplane_mode_on");
                    if status.contains("1") { "0" } else { "1" }
                },
                _ => return println!("Usage: aqs airplane-mode [on|off|toggle]"),
            };
            adb(&format!("settings put global airplane_mode_on {}", val));
            println!("✓ Airplane mode {}", if val == "1" { "ON" } else { "OFF" });
        },
        "doze" => {
            let val = args.get(2).map(|s| s.as_str()).unwrap_or("on");
            if val == "on" {
                adb("dumpsys deviceidle enable deep");
                println!("✓ Doze enabled");
            } else {
                adb("dumpsys deviceidle disable deep");
                println!("✓ Doze disabled");
            }
        },
        _ => println!("Unknown command: {}", args[1]),
    }
}
