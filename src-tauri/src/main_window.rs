use std::process::Command;

use tauri::{
    webview::NewWindowResponse, App, Url, WebviewUrl, WebviewWindowBuilder, Wry,
};

use crate::settings::Settings;

const DEFAULT_MESSENGER_URL: &str = "https://www.messenger.com";

pub fn setup(app: &mut App, settings: &Settings) -> tauri::Result<()> {
    let start_url = Url::parse(&settings.messenger_url)
        .unwrap_or_else(|_| Url::parse(DEFAULT_MESSENGER_URL).expect("valid Messenger URL"));

    WebviewWindowBuilder::new(app, "main", WebviewUrl::External(start_url))
        .title("Messenger")
        .inner_size(1200.0, 850.0)
        .min_inner_size(800.0, 600.0)
        .resizable(true)
        .fullscreen(false)
        .center()
        .on_navigation(|url| {
            if should_stay_in_app(url) {
                true
            } else {
                open_external_url(url);
                false
            }
        })
        .on_new_window(|url, _features| {
            open_external_url(&url);
            NewWindowResponse::<Wry>::Deny
        })
        .build()?;

    Ok(())
}

fn should_stay_in_app(url: &Url) -> bool {
    match url.scheme() {
        "http" | "https" => {
            matches!(
                url.host_str(),
                Some("messenger.com")
                    | Some("www.messenger.com")
                    | Some("facebook.com")
                    | Some("www.facebook.com")
                    | Some("m.facebook.com")
                    | Some("web.facebook.com")
            )
        }
        "about" | "data" | "blob" => true,
        _ => false,
    }
}

fn open_external_url(url: &Url) {
    let url = url.as_str();

    #[cfg(target_os = "windows")]
    let result = Command::new("rundll32")
        .args(["url.dll,FileProtocolHandler", url])
        .spawn();

    #[cfg(target_os = "macos")]
    let result = Command::new("open").arg(url).spawn();

    #[cfg(all(unix, not(target_os = "macos")))]
    let result = Command::new("xdg-open").arg(url).spawn();

    if let Err(error) = result {
        eprintln!("failed to open external URL: {error}");
    }
}
