use winit::{error::EventLoopError, event_loop::{ControlFlow, EventLoop}};
use termio_ui::windowing::window::App;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    env_logger::init();
    let event_loop = EventLoop::new()?;

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Wait);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    /* event_loop.set_control_flow(ControlFlow::Wait); */

    event_loop.run_app(&mut App::default())?;
    Ok(())
}