use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
};

use crate::context::ContextBuilder;
use crate::MoeglError;

pub(crate) struct Window {
    title: String,
}

impl Window {
    pub fn new(settings: &ContextBuilder) -> Self {
        Self {
            title: settings.title.to_owned(),
        }
    }

    /// Run event loop
    pub fn run(&self) -> Result<(), MoeglError>  {
        let event_loop = EventLoop::new().unwrap();
        let window = winit::window::WindowBuilder::new()
            .with_title(&self.title)
            .build(&event_loop).unwrap();

        let event_result = event_loop.run(move |event, control_flow| match event {
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
                _ => {}
            },
            _ => {}
        });

        match event_result {
            Ok(_) => Ok(()),
            Err(_) => Err(MoeglError::WinitError),
        }
    }
}

