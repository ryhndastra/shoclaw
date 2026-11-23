pub mod config;
pub mod tab;

use config::Config;
use tab::TabManager;
use tao::{event_loop::EventLoop, window::WindowBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Shoclaw Browser Initializing ===");

    // 1. Load user configuration
    let config = Config::load();
    println!(
        "Loaded Config: sidebar_mode={:?}, max_active_tabs={}, homepage={}",
        config.general.sidebar_mode,
        config.performance.max_active_tabs,
        config.general.homepage
    );

    // 2. Initialize TabManager with configured hibernation ceiling
    let mut tab_manager = TabManager::new(config.performance.max_active_tabs);
    let initial_tab_id = tab_manager.create_tab(&config.general.homepage);
    println!("Created initial tab ID: {}", initial_tab_id);

    // 3. Initialize Native Window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Shoclaw Browser")
        .build(&event_loop)?;

    println!("Created native window: {:?}", window.id());
    Ok(())
}
