use std::sync::{Arc, Mutex};
use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event_loop::EventLoopWindowTarget,
    window::{Window, WindowBuilder},
};
use wry::{Rect, WebView, WebViewBuilder};

#[cfg(target_os = "linux")]
use tao::platform::unix::WindowExtUnix;
#[cfg(target_os = "linux")]
use wry::WebViewBuilderExtUnix;

use super::navigation::{execute_navigation, normalize_url, NavigationAction};

pub struct BrowserWindow {
    pub window: Window,
    pub webview: WebView,
    pub current_title: Arc<Mutex<String>>,
}

impl BrowserWindow {
    pub fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        initial_url: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let window = WindowBuilder::new()
            .with_title("Shoclaw")
            .with_inner_size(LogicalSize::new(1280.0, 800.0))
            .with_min_inner_size(LogicalSize::new(640.0, 480.0))
            .build(event_loop)?;

        let current_title = Arc::new(Mutex::new("Shoclaw".to_string()));
        let title_clone = current_title.clone();

        let webview_builder = WebViewBuilder::new()
            .with_url(initial_url)
            .with_devtools(cfg!(debug_assertions))
            .with_document_title_changed_handler(move |title| {
                if let Ok(mut lock) = title_clone.lock() {
                    *lock = title;
                }
            });

        // Linux Wayland & X11 support via Tao's default vertical box container
        #[cfg(target_os = "linux")]
        let webview = {
            let vbox = window
                .default_vbox()
                .ok_or("Failed to retrieve default_vbox container from native Tao window")?;
            webview_builder.build_gtk(vbox)?
        };

        #[cfg(not(target_os = "linux"))]
        let webview = webview_builder.build(&window)?;

        Ok(Self {
            window,
            webview,
            current_title,
        })
    }

    pub fn navigate(&self, input: &str, search_template: &str) -> Result<(), Box<dyn std::error::Error>> {
        let target_url = normalize_url(input, search_template);
        execute_navigation(&self.webview, &NavigationAction::LoadUrl(target_url))
    }

    pub fn reload(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute_navigation(&self.webview, &NavigationAction::Reload)
    }

    pub fn back(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute_navigation(&self.webview, &NavigationAction::Back)
    }

    pub fn forward(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute_navigation(&self.webview, &NavigationAction::Forward)
    }

    pub fn handle_resize(&self, size: LogicalSize<f64>) -> Result<(), Box<dyn std::error::Error>> {
        self.webview.set_bounds(Rect {
            position: tao::dpi::Position::Logical(LogicalPosition::new(0.0, 0.0)),
            size: tao::dpi::Size::Logical(size),
        })?;
        Ok(())
    }
}
