use std::any::{Any};

use winit::{
    event::*,
    keyboard::{KeyCode, PhysicalKey},
    event_loop::EventLoop,
    event_loop::EventLoopBuilder,
};

use crate::app::{Context, ContextBuilder, GameState, Plugin};
use crate::MoeglError;
use crate::graphics::GraphicsContext;


#[derive(Default)]
pub struct WinitPlugin {
    window: Window    
}

impl WinitPlugin {
    fn new(settings: &ContextBuilder) -> Self {
        let window = Window::new(settings);

        WinitPlugin {
            window
        }
    }
}

impl Plugin for WinitPlugin {
    fn init(&mut self, ctx: &mut Context) {
        ctx.set_runner(run);
    } 
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Default)]
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

pub fn run(mut ctx: Context) -> Result<(), MoeglError> {
    println!("test");

    let mut event_loop_builder = EventLoopBuilder::new();
    
    /// TODO: Create winit plugin
    #[cfg(target_os= "windows")]
    {
        use winit::platform::windows::EventLoopBuilderExtWindows;
        event_loop_builder.with_any_thread(true);
    }

    let event_loop = event_loop_builder.build().unwrap();

    let window = ctx.get_plugin::<WinitPlugin>().unwrap();

    let winit_window = winit::window::WindowBuilder::new()
        .with_title("test")
        .with_inner_size(winit::dpi::LogicalSize::new(
            50,
            50,
        ))
        .build(&event_loop)
        .unwrap();

    let mut graphics_context = pollster::block_on(GraphicsContext::new(winit_window));

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
                window_id: _,
            } => match event {
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

                WindowEvent::KeyboardInput { event, .. } => {
                    if let PhysicalKey::Code(code) = event.physical_key {
                    }
                }

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
                    ctx.frame_loop();
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
