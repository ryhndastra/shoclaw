pub mod browser;
pub mod config;
pub mod tab;

use browser::BrowserWindow;
use config::Config;
use tao::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, ModifiersState},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    {
        // 1. Prevents WebKitGTK blank/black screen on Wayland (NVIDIA / Hybrid GPU).
        if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }

        // 2. Resolves NixOS TLS module path for HTTPS connections.
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
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("=== Shoclaw Browser Initializing ===");

    let config = Config::load();
    let event_loop = EventLoop::new();

    let mut browser_window = BrowserWindow::new(
        &event_loop,
        &config.general.homepage,
        config.performance.max_active_tabs,
    )?;

    println!(
        "Shoclaw running with max_active_tabs = {}",
        config.performance.max_active_tabs
    );

    let mut modifiers = ModifiersState::empty();
    let homepage = config.general.homepage.clone();

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
                        // Ctrl + T: Open new tab
                        if modifiers.control_key() && key_event.physical_key == KeyCode::KeyT {
                            println!("[Action] Opening new tab");
                            let _ = browser_window.open_tab(&homepage);
                        }
                        // Ctrl + W: Close active tab
                        else if modifiers.control_key() && key_event.physical_key == KeyCode::KeyW {
                            println!("[Action] Closing active tab");
                            if let Ok(Some(_)) = browser_window.close_active_tab() {
                                // Tab closed and neighbor activated
                            } else {
                                println!("No tabs left, exiting application.");
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                        // Ctrl + Tab: Switch to next tab
                        else if modifiers.control_key() && !modifiers.shift_key() && key_event.physical_key == KeyCode::Tab {
                            println!("[Action] Next tab");
                            let _ = browser_window.next_tab();
                        }
                        // Ctrl + Shift + Tab: Switch to previous tab
                        else if modifiers.control_key() && modifiers.shift_key() && key_event.physical_key == KeyCode::Tab {
                            println!("[Action] Previous tab");
                            let _ = browser_window.prev_tab();
                        }
                        // Ctrl + R: Reload active tab
                        else if modifiers.control_key() && key_event.physical_key == KeyCode::KeyR {
                            println!("[Action] Reloading active page");
                            let _ = browser_window.reload_active();
                        }
                        // Alt + Left: Back
                        else if modifiers.alt_key() && key_event.physical_key == KeyCode::ArrowLeft {
                            println!("[Action] Navigating back");
                            let _ = browser_window.back_active();
                        }
                        // Alt + Right: Forward
                        else if modifiers.alt_key() && key_event.physical_key == KeyCode::ArrowRight {
                            println!("[Action] Navigating forward");
                            let _ = browser_window.forward_active();
                        }
                        // Ctrl + H: Print memory & hibernation status
                        else if modifiers.control_key() && key_event.physical_key == KeyCode::KeyH {
                            browser_window.dump_memory_summary();
                        }
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // Synchronize window title
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
