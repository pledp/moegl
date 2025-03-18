use std::any::Any;

use nalgebra_glm as glm;

use winit::{
    event::*,
    keyboard::{KeyCode, PhysicalKey},
    event_loop:: {EventLoop, ControlFlow},
    event_loop::EventLoopBuilder,
};

use crate::app::{Context, ContextBuilder, GameState, Plugin};
use crate::MoeglError;
use crate::graphics::GraphicsContext;


#[derive(Default)]
pub struct WinitPlugin {
    pub window: Window    
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
    fn build(&mut self, settings: &ContextBuilder) {
        self.window = Window::new(settings);
    }

    fn init(&mut self, ctx: &mut Context) {
        ctx.set_runner(run);
    } 
}

#[derive(Default)]
pub struct Window {
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
    let mut event_loop_builder = EventLoopBuilder::new();
    
    /// TODO: Create winit plugin
    #[cfg(target_os= "windows")]
    {
        use winit::platform::windows::EventLoopBuilderExtWindows;
        event_loop_builder.with_any_thread(true);
    }

    let event_loop = event_loop_builder.build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let window = ctx.get_plugin::<WinitPlugin>().unwrap();
    println!("{}", window.window.title);

    let winit_window = winit::window::WindowBuilder::new()
        .with_title(window.window.title.to_owned())
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
                    ctx.frame_loop(&mut graphics_context);
                    
                    // TODO: Make idiomatic
                    let mut render_data = graphics_context.start_draw().unwrap();
                    graphics_context.clear(&mut render_data, glm::vec3(1.0, 1.0, 1.0));
                    graphics_context.render(&mut render_data);
                    graphics_context.end_draw(render_data);
                }
                _ => {}
            },
            Event::AboutToWait => {
                graphics_context.window().request_redraw();                
            }

            _ => {}
        }
    });

    match event_result {
        Ok(_) => Ok(()),
        Err(_) => Err(MoeglError::WinitError),
    }
}
