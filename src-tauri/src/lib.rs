// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod command;
use serde_json::Error;
use tauri::{menu::*, utils::config::WindowConfig};

fn json_to_window_config(window_json: &str) -> Result<WindowConfig, Error> {
    serde_json::from_str(window_json)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .menu(|handle| {
            let menu = Menu::with_items(
                handle,
                &[
                    #[cfg(target_os = "macos")]
                    &Submenu::with_items(
                        handle,
                        "Edit",
                        true,
                        &[
                            &PredefinedMenuItem::undo(handle, None)?,
                            &PredefinedMenuItem::redo(handle, None)?,
                            &PredefinedMenuItem::cut(handle, None)?,
                            &PredefinedMenuItem::copy(handle, None)?,
                            &PredefinedMenuItem::paste(handle, None)?,
                            &PredefinedMenuItem::select_all(handle, None)?,
                        ],
                    )?,
                ],
            );
            menu
        })
        .setup(|app| {
            let app_handle = app.handle();
            let window_json = r#"{"label":"PurringCat","title":"PurringCat","url":"https://example.com","userAgent":"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36","width":600,"height":800,"theme":null,"resizable":true,"fullscreen":false,"maximized":false,"minWidth":400,"minHeight":300,"maxWidth":1920,"maxHeight":1080,"decorations":true,"transparent":false,"titleBarStyle":"Visible","visible":true,"focus":true,"closable":true,"minimizable":true,"maximizable":true,"alwaysOnTop":false,"alwaysOnBottom":false,"center":false,"skipTaskbar":false,"tabbingIdentifier":null,"parent":null,"dragDropEnabled":true,"browserExtensionsEnabled":false,"devtools":true,"contentProtected":false,"hiddenTitle":false,"incognito":false,"proxyUrl":null,"useHttpsScheme":false,"zoomHotkeysEnabled":false,"acceptFirstMouse":false}"#;
            match json_to_window_config(window_json) {
                Ok(config) => {
                    println!("Parsed WindowConfig: {:?}", config);
                    let _main_window =
                        tauri::WebviewWindowBuilder::from_config(app_handle, &config)
                            .unwrap()
                            .build()
                            .unwrap();
                }
                Err(err) => {
                    eprintln!("Failed to parse JSON: {}", err);
                }
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            command::pakeplus::open_window,
            command::pakeplus::preview_from_config,
            command::pakeplus::update_build_file,
            command::pakeplus::update_config_file,
            command::pakeplus::update_cargo_file,
            command::pakeplus::update_main_rust,
            command::pakeplus::update_custom_js,
            command::pakeplus::content_to_base64,
            command::pakeplus::update_config_json,
            command::pakeplus::rust_main_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
