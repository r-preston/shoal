#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod actors;
mod renderer;
mod utility;
mod world;

use crate::utility::*;
use renderer::{camera::Camera, Renderer};
use std::{
    alloc::System,
    time::{Duration, SystemTime},
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};
use world::World;

/*
// things I will need
fish - flocking, try and stay in sphere, avoid predators. need velocity, position
sharks - roam near prey, sometimes attack
world - hold list of actors, passes data to renderer
renderer (main) - track timer and draw things
camera - moveable

// workflow:
have an array of fish positions and velocities
create a density and velocity field map from fish positions each tick
update all fish with using these maps
update predators using maps
multiple density maps? fish vs threat
*/
fn main() {
    let start = SystemTime::now();

    let mut state = State::new(World::new(100.0, 5, 10, 0));
    pollster::block_on(state.run());

    let end = SystemTime::now();
    println!(
        "{} frames in {}ms",
        state.frame(),
        SystemTime::now().duration_since(start).unwrap().as_millis()
    );
}

struct State {
    world: World,
    camera: Camera,
    frame: u32,
}

impl State {
    pub fn new(world: World) -> State {
        let camera = Camera::new(1.3 * world.size());
        Self {
            world,
            camera,
            frame: 0,
        }
    }

    pub fn frame(&self) -> u32 {
        self.frame
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
    pub async fn run(&mut self) {
        env_logger::init();

        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let mut renderer = Renderer::new(&window).await;

        let perspective = cgmath::PerspectiveFov::<f32> {
            fovy: Radians::from(Degrees(90.0)),
            aspect: (window.inner_size().width as f32) / (window.inner_size().height as f32),
            near: 1.0,
            far: self.world.size() * 3.0,
        };
        let perspective_matrix = Mat4::from(perspective);

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
                            /*
                            WindowEvent::CursorMoved {
                                position: mouse_position, ..
                            } => match key_event {
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::KeyW),
                                    ..
                                } => {println!("W");}
                                _ => {}
                            },
                            */
                            WindowEvent::Resized(physical_size) => {
                                renderer.resize(*physical_size);
                            }

                            WindowEvent::RedrawRequested => 'RedrawLabel: {
                                renderer.window().request_redraw();
                                //println!("{}", SystemTime::now().duration_since(last_frame).unwrap().as_millis());

                                if SystemTime::now().duration_since(last_frame).unwrap()
                                    < Duration::from_nanos(16666667)
                                {
                                    break 'RedrawLabel;
                                }

                                println!("{}", self.frame);
                                self.frame += 1;
                                last_frame = SystemTime::now();

                                renderer.update();

                                match renderer.render() {
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
