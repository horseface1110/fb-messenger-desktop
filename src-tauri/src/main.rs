mod main_window;
mod settings;
mod shortcuts;
mod theme;
mod tray;

use tauri::{Manager, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            settings::get_settings,
            settings::save_settings
        ])
        .setup(|app| {
            let settings = settings::read_settings(app.handle())?;
            main_window::setup(app, &settings)?;
            tray::setup(app)?;
            shortcuts::setup(app)?;
            theme::apply_theme(app.handle(), &settings);

            if settings.start_minimized {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != "main" {
                return;
            }

            if let WindowEvent::CloseRequested { api, .. } = event {
                let close_to_tray = settings::read_settings(window.app_handle())
                    .map(|settings| settings.close_to_tray)
                    .unwrap_or(true);

                if close_to_tray {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Messenger Desktop");
}
