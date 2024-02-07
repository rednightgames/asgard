#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    command, generate_context, generate_handler, Builder, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem
};

#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let name = CustomMenuItem::new("name".to_string(), "Rednight").disabled();
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit Rednight");

    let tray_menu = SystemTrayMenu::new()
        .add_item(name)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "hide" => {
                        let window = app.get_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                            item_handle.set_title("Show").unwrap();
                        } else {
                            window.show().unwrap();
                            item_handle.set_title("Hide").unwrap();
                        }
                    }
                    "quit" => {
                        let window = app.get_window("main").unwrap();
                        window.close().unwrap();
                    }
                    _ => {}
                }
            }
        })
        .invoke_handler(generate_handler![greet])
        .run(generate_context!())
        .expect("error while running tauri application")
}
