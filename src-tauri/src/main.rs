#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    WindowEvent,
};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu_hide = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu_hide);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();

                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.hide().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();

                    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
                    let show = CustomMenuItem::new("show".to_string(), "Show");

                    let tray_menu_show = SystemTrayMenu::new()
                        .add_item(quit)
                        .add_native_item(SystemTrayMenuItem::Separator)
                        .add_item(show);

                    app.tray_handle().set_menu(tray_menu_show).unwrap();
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();

                    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
                    let show = CustomMenuItem::new("hide".to_string(), "Hide");

                    let tray_menu_hide = SystemTrayMenu::new()
                        .add_item(quit)
                        .add_native_item(SystemTrayMenuItem::Separator)
                        .add_item(show);

                    app.tray_handle().set_menu(tray_menu_hide).unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .enable_macos_default_menu(true)
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::WindowEvent {
                label: _, event, ..
            } => match event {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();

                    let win = _app_handle.get_window("main").unwrap();

                    win.hide().unwrap();
                }
                _ => {}
            },
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
