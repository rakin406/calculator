#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use evalexpr::*;

/// A working calculator
#[tauri::command]
fn calculate(problem: &str) -> String {
    let calc = eval(problem);
    match calc {
        Ok(value) => {
            format!("{}", value.to_string())
        }
        Err(_error) => {
            format!("?")
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
