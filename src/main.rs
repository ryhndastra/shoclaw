pub mod browser;
pub mod config;
pub mod tab;

use browser::BrowserWindow;
use config::Config;
use tab::TabManager;
use tao::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, ModifiersState},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
        if std::env::var_os("GIO_MODULE_DIR").is_none() {
            if let Ok(entries) = std::fs::read_dir("/nix/store") {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    if name_str.contains("glib-networking")
                        && !name_str.ends_with(".drv")
                        && !name_str.ends_with(".tar.xz")
                    {
                        let candidate = entry.path().join("lib/gio/modules");
                        if candidate.join("libgiognutls.so").exists() {
                            std::env::set_var("GIO_MODULE_DIR", &candidate);
                            println!("Configured GIO_MODULE_DIR: {:?}", candidate);
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("=== Shoclaw Browser Initializing ===");

    let config = Config::load();
    let mut tab_manager = TabManager::new(config.performance.max_active_tabs);
    let initial_tab_id = tab_manager.create_tab(&config.general.homepage);
    println!("Initial Tab ID: {}, URL: {}", initial_tab_id, config.general.homepage);

    let event_loop = EventLoop::new();
    let browser_window = BrowserWindow::new(&event_loop, &config.general.homepage)?;

    println!(
        "WebKit Viewport Attached successfully to Window {:?}",
        browser_window.window.id()
    );

    let mut modifiers = ModifiersState::empty();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Shoclaw: Close requested, exiting cleanly.");
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(physical_size) => {
                    let scale = browser_window.window.scale_factor();
                    let logical_size = physical_size.to_logical::<f64>(scale);
                    let _ = browser_window.handle_resize(logical_size);
                }
                WindowEvent::ModifiersChanged(new_modifiers) => {
                    modifiers = new_modifiers;
                }
                WindowEvent::KeyboardInput {
                    event: key_event,
                    ..
                } => {
                    if key_event.state == ElementState::Pressed {
                        // Ctrl + R: Reload
                        if modifiers.control_key() && key_event.physical_key == KeyCode::KeyR {
                            println!("Shortcut: Reloading active page");
                            let _ = browser_window.reload();
                        }
                        // Alt + Left: Back
                        else if modifiers.alt_key() && key_event.physical_key == KeyCode::ArrowLeft {
                            println!("Shortcut: Navigating back");
                            let _ = browser_window.back();
                        }
                        // Alt + Right: Forward
                        else if modifiers.alt_key() && key_event.physical_key == KeyCode::ArrowRight {
                            println!("Shortcut: Navigating forward");
                            let _ = browser_window.forward();
                        }
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // Synchronize window title if the page title changed
                if let Ok(title_lock) = browser_window.current_title.lock() {
                    let formatted_title = format!("{} - Shoclaw", *title_lock);
                    if browser_window.window.title() != formatted_title {
                        browser_window.window.set_title(&formatted_title);
                    }
                }
            }
            _ => {}
        }
    });
}
