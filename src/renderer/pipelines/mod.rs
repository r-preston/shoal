pub mod skybox;

use wgpu::{Device, SurfaceConfiguration};

use crate::renderer::Renderer;
use crate::utility::Vec4;
use crate::world::World;

pub struct Pipelines(Vec<Box<dyn Pipeline>>);

impl Pipelines {
    pub fn generate(device: &Device, config: &SurfaceConfiguration) -> Pipelines {
        let mut pipelines = Vec::<Box<dyn Pipeline>>::new();

        pipelines.push(Box::new(skybox::SkyboxPipeline::new(device, config)));

        Pipelines(pipelines)
    }

    pub fn pipelines(&self) -> &Vec<Box<dyn Pipeline>> {
        &self.0
    }
}
pub trait Pipeline {
    fn geometry(&self, world: &World) -> &Box<[Vec4]>;

    fn pipeline(&self) -> &wgpu::RenderPipeline;
}
