// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent, TrayIcon};
use tauri::tray::{SystemTrayBuilder, TrayIconBuilder};
use tauri::menu::{MenuBuilder, MenuItemBuilder};

pub fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let toggle = MenuItemBuilder::with_id("toggle", "Toggle").build(app)?;
            let menu = MenuBuilder::new(app).items(&[&toggle]).build()?;
            let tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(move |_app, event| match event.id().as_ref() {
                    "toggle" => {
                        println!("toggle clicked");
                    }
                    _ => (),
                })
                .on_tray_icon_event(|tray, event| {
                    if event.click_type == tauri::tray::ClickType::Left {
                        let app = tray.app_handle();

                        #[cfg(not(target_os = "macos"))] {
                            if let Some(webview_window) = app.get_webview_window("main") {
                                let _ = webview_window.show();
                                let _ = webview_window.set_focus();
                            }
                        }

                        #[cfg(target_os = "macos")] {
                            tauri::AppHandle::show(&app.app_handle()).unwrap();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                #[cfg(not(target_os = "macos"))] {
                    window.hide().unwrap();
                }

                #[cfg(target_os = "macos")] {
                    tauri::AppHandle::hide(&window.app_handle()).unwrap();
                }
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
