use tao::{event_loop::EventLoop, window::WindowBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Shoclaw Browser - Native Core Initialized");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Shoclaw")
        .build(&event_loop)?;

    println!("Created window handle: {:?}", window.id());
    Ok(())
}
