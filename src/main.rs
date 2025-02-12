// src/main.rs
use log::{info, error};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod platform;
mod compositor;
mod graphics;
mod window;
mod input;
mod scene;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    info!("Starting GL Engine");

    // Create event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("GL Engine")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)?;

    // Initialize graphics context
    let graphics = graphics::GraphicsContext::new(&window)?;

    // Main event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    info!("Window close requested");
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    info!("Window resized to: {:?}", size);
                    graphics.resize(size.width, size.height);
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                if let Err(e) = graphics.render() {
                    error!("Render error: {}", e);
                }
            }
            _ => {}
        }
    });
}