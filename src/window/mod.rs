use std::time::{Duration, Instant};

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::context::{Context, ContextBuilder, GameState};
use crate::App;
use crate::MoeglError;

pub(crate) struct Window {
    title: String,
    fps: u32,
    width: u32,
    height: u32,
}

impl Window {
    pub fn new(settings: &ContextBuilder) -> Self {
        Self {
            title: settings.title.to_owned(),
            fps: settings.fps,
            width: settings.width,
            height: settings.height,
        }
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.fps = fps;
    }
}

pub fn run<A>(ctx: &mut Context, app: &A) -> Result<(), MoeglError>
where
    A: App,
{
    let event_loop = EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_title(&ctx.window.title)
        .with_inner_size(winit::dpi::LogicalSize::new(
            ctx.window.width,
            ctx.window.height,
        ))
        .build(&event_loop)
        .unwrap();

    let mut last_frame_time = Instant::now();

    let event_result = event_loop.run(move |event, control_flow| {
        match ctx.state {
            GameState::QuitRequested => {
                control_flow.exit();
            }

            _ => {}
        }

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

                // Main loop, run draw, update, etc
                WindowEvent::RedrawRequested => {
                    window.request_redraw();
                    let now = Instant::now();

                    if now - last_frame_time >= Duration::from_secs_f64(1.0 / ctx.window.fps as f64)
                    {
                        last_frame_time = now;

                        ctx.update(app);
                        ctx.draw(app);
                    }
                }
                _ => {}
            },

            _ => {}
        }
    });

    match event_result {
        Ok(_) => Ok(()),
        Err(_) => Err(MoeglError::WinitError),
    }
}
