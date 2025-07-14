use winit::{
    application::ApplicationHandler,
    event::{WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow},
    window::{Window, WindowId},
};
use pixels::{Pixels, SurfaceTexture};
use log::info;

#[derive(Default)]
pub struct App {
    window: Option<Window>,

}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();
        let size = window.inner_size();
        println!("Window created: {}x{}", size.width, size.height);
        self.window = Some(window);
        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();
                let size = window.inner_size();

                let surface = SurfaceTexture::new(size.width, size.height, window);
                let mut pixels = Pixels::new(size.width, size.height, surface).unwrap();

                let frame: &mut [u8] = pixels.frame_mut();
                for pixel  in frame.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&[0x00, 0x80, 0xFF, 0xFF]); // Light blue
                }

                pixels.render().unwrap();
            }

            WindowEvent::CloseRequested => {
                println!("Window closed");
                event_loop.exit();
            }

            _ => {}
        }
    }
}