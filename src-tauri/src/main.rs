// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::time::Duration;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::thread;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, Manager, AppHandle};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit);
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);

    thread::spawn(|| {
        listen();
    });

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
            SystemTrayEvent::DoubleClick {
                ..
            } => {}
            SystemTrayEvent::RightClick {
                ..
            } => {}
            _ => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![items, set_data, clear, toggle_window, hide])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

const MAX_ITEMS: usize = 50;
static CONTAINER: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn listen() {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut last_clipboard_contents = clipboard.get_contents().unwrap_or("".to_string());
    loop {
        let current_clipboard_contents = match clipboard.get_contents() {
            Ok(t) => t,
            Err(e) => {
                println!("get clipboard error: {:?}", e);
                "".to_string()
            }
        };
        if current_clipboard_contents.len() > 0 && current_clipboard_contents != last_clipboard_contents {
            last_clipboard_contents = current_clipboard_contents;
            set(last_clipboard_contents.clone());
        }
        thread::sleep(Duration::from_millis(1500));
    }
}

#[tauri::command]
fn items(s: String) -> Vec<String> {
    CONTAINER.lock().unwrap().iter().filter(|&x| s.len() == 0 || (*x).contains(&s)).cloned().rev().collect()
}

#[tauri::command]
fn set_data(data: String) -> String {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    match clipboard.set_contents(data) {
        Err(e) => format!("set clipboard error: {:?}", e),
        _ => format!("OK"),
    }
}

fn set(data: String) {
    let mut container = CONTAINER.lock().unwrap();

    container.push(data);

    if container.len() > MAX_ITEMS {
        container.remove(0);
    }
}

#[tauri::command]
fn clear() {
    CONTAINER.lock().unwrap().clear();
}

#[tauri::command]
fn toggle_window(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    if window.is_visible().unwrap() {
        window.hide().unwrap();
    } else {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

#[tauri::command]
fn hide(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    window.hide().unwrap();
}