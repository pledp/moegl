use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::context::{Context, ContextBuilder, GameState};
use crate::graphics::GraphicsContext;
use crate::App;
use crate::MoeglError;

pub(crate) struct Window {
    pub title: String,
    pub fps: u32,
    pub width: u32,
    pub height: u32,
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

pub fn run<A>(ctx: &mut Context, app: &A, mut event_loop: EventLoop<()>, mut graphics_context: GraphicsContext) -> Result<(), MoeglError>
where
    A: App,
{
    let event_result = event_loop.run(move |event, control_flow| {
        match ctx.state {
            GameState::QuitRequested => {
                control_flow.exit();
            }

            _ => {}
        }

        match event {
            Event::WindowEvent { ref event, window_id: _, } => match event {
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

                WindowEvent::Resized(physical_size) => {
                    graphics_context.resize(*physical_size);
                }
                
                /*
                WindowEvent::CursorMoved { device_id, position } => {
                    todo!();
                }
                */

                // Main loop, run draw, update, etc
                WindowEvent::RedrawRequested => {
                    graphics_context.window().request_redraw();
                    ctx.frame_loop(app, &mut graphics_context);
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
