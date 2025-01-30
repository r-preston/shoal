#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod actors;
mod world;
mod utility;
mod renderer;

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use renderer::Renderer;
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
    let mut state = State::new(
        World::new(100.0, 5, 10, 0)
    );
    pollster::block_on(state.run());
}

struct State {
    world: World
}

impl State {

pub fn new(world: World) -> State {
    Self {
        world
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run(&mut self) {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut renderer = Renderer::new(&window).await;

    let _ = event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == renderer.window().id() => if !renderer.input(event) { 
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
                },
                
                WindowEvent::RedrawRequested => {
                    // This tells winit that we want another frame after this one
                    renderer.window().request_redraw();
        
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
                },

                _ => {}
            }
        },
        _ => {}
    });
}

}
