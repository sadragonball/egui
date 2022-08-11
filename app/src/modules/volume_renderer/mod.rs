use std::sync::Arc;
use std::time::Duration;
use wgpu::Device;
use wgpu::util::DeviceExt;
use eframe::{egui_wgpu, Storage};
use eframe::emath::Vec2;
use eframe::epaint::Rgba;
use egui::{Visuals};
use crate::modules::volume_renderer::camera::{Camera, CameraBinding};
use crate::modules::volume_renderer::ray_cast_pipeline::RayCastPipeline;
use crate::modules::volume_renderer::render_resources::{GlobalUniform, GlobalUniformBinding};
use crate::modules::volume_renderer::volume_texture::VolumeTexture;
use crate::utils::shader_compiler::ShaderCompiler;

mod camera;
mod ray_cast_pipeline;
mod render_resources;
mod volume_texture;

//待定
// trait BasicPipeline {
//     fn from_path(&self,
//                  device: &wgpu::Device,
//                  path: &std::path::Path,
//                  shader_compiler: &mut crate::utils::ShaderCompiler) -> Self;
// }

pub struct VolumeRenderer {}


impl VolumeRenderer {
    pub const WIDTH: u32 = 800;
    pub const HEIGHT: u32 = 600;

    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().expect("wgpu enabled");
        
        let device = &wgpu_render_state.device;

        let mut shader_compiler = ShaderCompiler::new();

        let path = std::path::Path::new("E:\\Github\\egui\\app\\raws\\bonsai_256x256x256_uint8.raw");

        let volume_texture =
            VolumeTexture::new(&wgpu_render_state.device, &wgpu_render_state.queue, path);

        let path = std::path::Path::new("E:\\GitHub\\egui\\app\\src\\shaders\\raycast_naive.wgsl");

        let pipeline =
            RayCastPipeline::from_path(wgpu_render_state, path, &mut shader_compiler);

        let camera = Camera::new(
            1.,
            0.5,
            1.,
            (0., 0., 0.).into(),
            Self::WIDTH as f32 / Self::HEIGHT as f32,
        );


        // Because the graphics pipeline must have the same lifetime as the egui render pass,
        // instead of storing the pipeline in our `Custom3D` struct, we insert it into the
        // `paint_callback_resources` type map, which is stored alongside the render pass.
        wgpu_render_state
            .egui_rpass
            .write()
            .paint_callback_resources
            .insert((
                RenderResources {
                    volume_texture,
                    pipeline,
                    camera,
                    camera_binding: CameraBinding::new(&device),
                    global_uniform: GlobalUniform::default(),
                    global_uniform_binding: GlobalUniformBinding::new(&device),
                }
            ));
        Self {}
    }
}

impl eframe::App for VolumeRenderer {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("Volume renderer first render triangle of ");
                        ui.hyperlink_to("wgpu", "https://wgpu.rs");
                        ui.label(" Rust graphics api");
                    });

                    ui.label("It's not a very impressive demo, but it shows you can embed 3D inside of egui.");

                    egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        self.painting(ui);
                    });

                    ui.label("Drag to rotate!");
                });
        });
    }
}

// for renderer
impl VolumeRenderer {
    fn painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(
                egui::Vec2::new(Self::WIDTH as f32, Self::HEIGHT as f32),
                egui::Sense::drag()
            );
    }
}

struct TriangleRenderResources {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
}

impl TriangleRenderResources {
    fn prepare(&self, _device: &wgpu::Device, queue: &wgpu::Queue, angle: f32) {
        // Update our uniform buffer with the angle from the UI

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[angle]));
    }

    fn paint<'rpass>(&'rpass self, rpass: &mut wgpu::RenderPass<'rpass>) {
        // Draw our triangle!
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw(0..3, 0..1);
    }
}

struct RenderResources {
    volume_texture: VolumeTexture,
    pipeline: RayCastPipeline,
    camera: Camera,
    camera_binding: CameraBinding,
    global_uniform: GlobalUniform,
    global_uniform_binding: GlobalUniformBinding,
}

impl RenderResources {
    fn prepare(&self, _device: &wgpu::Device, queue: &wgpu::Queue, angle: f32) {
        // Update our uniform buffer with the angle from the UI

        // queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[angle]));
    }

    fn paint<'rpass>(&'rpass self, rpass: &mut wgpu::RenderPass<'rpass>) {
        // Draw our volume!
        self.pipeline.record(rpass,
                             &self.global_uniform_binding,
                             &self.camera_binding,
                             &self.volume_texture);
    }
}
