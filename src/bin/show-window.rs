use std::num::NonZero;

use anyhow::Result;
use vulkrust_play::engine::VulkanEngine;
use winit::{event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::{Window, WindowAttributes}};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::window::{WindowId};
use softbuffer::{Context, Surface};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let w = event_loop.create_window(
            WindowAttributes::default().with_title("WSLg - first frame")
        ).unwrap();

        {
            let ctx  = Context::new(&w).unwrap();
            let mut surf = Surface::new(&ctx, &w).unwrap();
            let size = w.inner_size();
            surf.resize(NonZero::new(size.width).unwrap(), NonZero::new(size.height).unwrap()).ok();
            let mut buf = surf.buffer_mut().unwrap();
            for px in buf.iter_mut() { *px = 0xFF_202040; }
            buf.present().unwrap();
        } 

        self.window = Some(w);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        println!("{event:?}");
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            // WindowEvent::Resized(new_size) => {
            // //     if let Some(s) = &mut self.sb_surf {
            // //         // s.resize(new_size.width, new_size.height).ok();
            // //         // optional: redraw to fill new size
            // //         if let Ok(mut buf) = s.buffer_mut() {
            // //             for px in buf.iter_mut() { *px = 0xFF_202040; }
            // //             let _ = buf.present();
            // //         }
            // //     }
            // },
            // WindowEvent::RedrawRequested => {
            //      let window = self.window.as_ref().expect("redraw request without a window");

            //     // Notify that you're about to draw.
            //     window.pre_present_notify();

            //     // Draw.
            //     // fill::fill_window(window.as_ref());

            //     window.request_redraw();
            // }
            _ => (),
        }
    }
}

fn main() -> Result<()>{
    let mut engine = VulkanEngine::new("Window App", true)?;
    engine.create_device()?;

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    
    Ok(())
}
