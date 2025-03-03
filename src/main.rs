#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod actors;
mod renderer;
mod utility;
mod world;

use crate::utility::*;
use renderer::Renderer;
use std::{
    alloc::System,
    time::{Duration, SystemTime},
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};
use world::World;

const WORLD_RADIUS: f32 = 10.0;
const GRID_DIVISIONS: u32 = 15; // should be an odd number
const FISH_COUNT: u32 = 10;
const SHARK_COUNT: u32 = 0;

fn main() {
    let start = SystemTime::now();

    let mut state = State::new(World::new(
        WORLD_RADIUS,
        GRID_DIVISIONS,
        FISH_COUNT,
        SHARK_COUNT,
    ));
    pollster::block_on(state.run());

    let end = SystemTime::now();
    log::info!(
        "{} frames in {}ms ({} FPS)",
        state.frame(),
        SystemTime::now().duration_since(start).unwrap().as_millis(),
        (state.frame() as f32)
            / SystemTime::now()
                .duration_since(start)
                .unwrap()
                .as_secs_f32()
    );
}

struct State {
    world: World,
    frame: u32,
}

impl State {
    pub fn new(world: World) -> State {
        Self { world, frame: 0 }
    }

    pub fn frame(&self) -> u32 {
        self.frame
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
    pub async fn run(&mut self) {
        env_logger::init();

        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let mut renderer = Renderer::new(&window, &self.world).await;

        let mut last_frame = SystemTime::now();
        let _ = event_loop.run(move |event, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == renderer.window().id() => {
                    if !renderer.input(event) {
                        match event {
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
                                renderer.resize(*physical_size);
                            }

                            WindowEvent::RedrawRequested => 'RedrawLabel: {
                                renderer.window().request_redraw();

                                // limit to 60fps
                                if SystemTime::now().duration_since(last_frame).unwrap()
                                    < Duration::from_nanos(16666667)
                                {
                                    break 'RedrawLabel;
                                }

                                self.frame += 1;
                                last_frame = SystemTime::now();

                                // update sim
                                self.world.update(self.frame);
                                renderer.update(&mut self.world);

                                match renderer.render(&self.world) {
                                    Ok(_) => {}

                                    // Reconfigure the surface if it's lost or outdated
                                    Err(
                                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
                                    ) => renderer.resize(renderer.size()),

                                    // The system is out of memory, we should probably quit
                                    Err(wgpu::SurfaceError::OutOfMemory) => {
                                        log::error!("OutOfMemory");
                                        control_flow.exit();
                                    }

                                    // This happens when the a frame takes too long to present
                                    Err(wgpu::SurfaceError::Timeout) => {
                                        log::warn!("Surface timeout")
                                    }

                                    // This happens when the a frame takes too long to present
                                    Err(wgpu::SurfaceError::Other) => {
                                        log::warn!("Unexpected Surface error")
                                    }
                                }
                            }

                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        });
    }
}
