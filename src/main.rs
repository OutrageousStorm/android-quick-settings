use clap::{Parser, Subcommand};
use colored::*;
use regex::Regex;
use std::process::{Command, Stdio};

#[derive(Parser)]
#[command(name = "aqs", about = "Android Quick Settings Manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Toggle airplane mode
    Airplane {
        #[arg(value_parser = ["on", "off"])]
        state: Option<String>,
    },
    /// Toggle WiFi
    Wifi {
        #[arg(value_parser = ["on", "off"])]
        state: Option<String>,
    },
    /// Toggle Bluetooth
    Bluetooth {
        #[arg(value_parser = ["on", "off"])]
        state: Option<String>,
    },
    /// Set brightness (0-255)
    Brightness { level: u8 },
    /// Set screen timeout (minutes)
    Timeout { minutes: u32 },
    /// Enable/disable location
    Location {
        #[arg(value_parser = ["on", "off"])]
        state: Option<String>,
    },
    /// Show current device settings status
    Status,
    /// Batch apply settings from JSON
    Batch { file: String },
}

fn adb_shell(cmd: &str) -> String {
    let output = Command::new("adb")
        .args(&["shell", cmd])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to run adb");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn get_setting(namespace: &str, key: &str) -> String {
    adb_shell(&format!("settings get {} {}", namespace, key))
        .trim()
        .to_string()
}

fn put_setting(namespace: &str, key: &str, value: &str) {
    adb_shell(&format!("settings put {} {} {}", namespace, key, value));
}

fn get_status() {
    println!("{}", "Device Settings Status".bold().underline());
    
    let settings = [
        ("Airplane mode", "global", "airplane_mode_on"),
        ("WiFi", "global", "wifi_on"),
        ("Bluetooth", "global", "bluetooth_on"),
        ("Location", "secure", "location_mode"),
        ("Brightness", "system", "screen_brightness"),
        ("Screen timeout", "system", "screen_off_timeout"),
    ];

    for (name, ns, key) in &settings {
        let val = get_setting(ns, key);
        let status = match val.as_str() {
            "0" | "" => "OFF".red(),
            "1" => "ON".green(),
            _ => val.normal(),
        };
        println!("  {:<18} {}", format!("{}:", name), status);
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Airplane { state } => {
            let val = match state.as_deref() {
                Some("on") => "1",
                Some("off") => "0",
                None => {
                    let cur = get_setting("global", "airplane_mode_on");
                    if cur == "1" { "0" } else { "1" }
                }
                _ => "0",
            };
            put_setting("global", "airplane_mode_on", val);
            println!("{} Airplane mode: {}", "✓".green(), if val == "1" { "ON" } else { "OFF" });
        }

        Commands::Wifi { state } => {
            let val = match state.as_deref() {
                Some("on") => "1",
                Some("off") => "0",
                None => {
                    let cur = get_setting("global", "wifi_on");
                    if cur == "1" { "0" } else { "1" }
                }
                _ => "0",
            };
            put_setting("global", "wifi_on", val);
            println!("{} WiFi: {}", "✓".green(), if val == "1" { "ON" } else { "OFF" });
        }

        Commands::Bluetooth { state } => {
            let val = match state.as_deref() {
                Some("on") => "1",
                Some("off") => "0",
                None => {
                    let cur = get_setting("global", "bluetooth_on");
                    if cur == "1" { "0" } else { "1" }
                }
                _ => "0",
            };
            put_setting("global", "bluetooth_on", val);
            println!("{} Bluetooth: {}", "✓".green(), if val == "1" { "ON" } else { "OFF" });
        }

        Commands::Brightness { level } => {
            put_setting("system", "screen_brightness_mode", "0"); // manual
            put_setting("system", "screen_brightness", &level.to_string());
            println!("{} Brightness set to {}", "✓".green(), level);
        }

        Commands::Timeout { minutes } => {
            let ms = minutes as u64 * 60 * 1000;
            put_setting("system", "screen_off_timeout", &ms.to_string());
            println!("{} Screen timeout: {} min", "✓".green(), minutes);
        }

        Commands::Location { state } => {
            let val = match state.as_deref() {
                Some("on") => "3",  // high accuracy
                Some("off") => "0",
                None => {
                    let cur = get_setting("secure", "location_mode");
                    if cur == "0" { "3" } else { "0" }
                }
                _ => "0",
            };
            put_setting("secure", "location_mode", val);
            println!("{} Location: {}", "✓".green(), if val != "0" { "ON" } else { "OFF" });
        }

        Commands::Status => {
            get_status();
        }

        Commands::Batch { file } => {
            println!("Batch mode not yet implemented");
        }
    }
}
