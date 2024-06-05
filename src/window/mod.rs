use std::time::{Duration, Instant};

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::context::{ContextBuilder, Context};
use crate::App;
use crate::MoeglError;

pub(crate) struct Window {
    title: String,
    fps: u32,
}

impl Window {
    pub fn new(settings: &ContextBuilder) -> Self {
        Self {
            title: settings.title.to_owned(),
            fps: settings.fps,
        }
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.fps = fps;
    } 
}

pub fn run<A>(mut context: Context, app: &A) -> Result<(), MoeglError> 
where
    A: App,
{
    let event_loop = EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_title(&context.window.title)
        .build(&event_loop).unwrap();

    let mut last_frame_time = Instant::now();

    let event_result = event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.exit(),
                
                /// Main loop, run draw, update, etc
                WindowEvent::RedrawRequested => {
                    window.request_redraw();
                    let now = Instant::now();

                    if now - last_frame_time >= Duration::from_secs_f64(1.0 / context.window.fps as f64) {
                        last_frame_time = now;

                        context.update(app);
                        context.draw(app);
                    }
                }
                _ => {}
            }

            _ => {}
        }
    });

    match event_result {
        Ok(_) => Ok(()),
        Err(_) => Err(MoeglError::WinitError),
    }
}
