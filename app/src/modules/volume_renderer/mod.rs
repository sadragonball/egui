use std::sync::Arc;
use std::time::Duration;
use wgpu::util::DeviceExt;
use eframe::{egui_wgpu, Storage};
use eframe::emath::Vec2;
use eframe::epaint::Rgba;
use egui::{Visuals};
use crate::modules::volume_renderer::ray_cast_pipeline::RayCastPipeline;
use crate::modules::volume_renderer::render_resources::{Uniform, UniformBinding};
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

pub struct VolumeRenderer {
    shader_compiler: ShaderCompiler,
    volume_texture: VolumeTexture,
    pipeline: RayCastPipeline,
    uniform: Uniform,
    uniform_binding: UniformBinding,
}

impl VolumeRenderer {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Self {
        let wgpu_render_state = cc.wgpu_render_state.as_ref().expect("wgpu enabled");
        let device = &wgpu_render_state.device;

        let mut shader_compiler = ShaderCompiler::new();

        let path = std::path::Path::new("G:\\Github\\egui\\app\\raws\\bonsai_256x256x256_uint8.raw");

        let volume_texture =
            VolumeTexture::new(&wgpu_render_state.device, &wgpu_render_state.queue, path);

        let path = std::path::Path::new("app/src/shaders/raycast_naive.wgsl");

        let pipeline =
            RayCastPipeline::from_path(wgpu_render_state, path, &mut shader_compiler);


        // Because the graphics pipeline must have the same lifetime as the egui render pass,
        // instead of storing the pipeline in our `Custom3D` struct, we insert it into the
        // `paint_callback_resources` type map, which is stored alongside the render pass.
        wgpu_render_state
            .egui_rpass
            .write()
            .paint_callback_resources
            .insert(());

        Self {
            shader_compiler,
            volume_texture,
            pipeline,
            uniform: Uniform::default(),
            uniform_binding: UniformBinding::new(&device)
        }
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

            ui.allocate_exact_size(egui::Vec2::new(800., 600.), egui::Sense::drag());

        // self.angle += response.drag_delta().x * 0.01;

        // Clone locals so we can move them into the paint callback:
        // let angle = self.angle;

        // The callback function for WGPU is in two stages: prepare, and paint.
        //
        // The prepare callback is called every frame before paint and is given access to the wgpu
        // Device and Queue, which can be used, for instance, to update buffers and uniforms before
        // rendering.
        //
        // The paint callback is called after prepare and is given access to the render pass, which
        // can be used to issue draw commands.
        // let cb = egui_wgpu::CallbackFn::new()
        //     .prepare(move |device, queue, paint_callback_resources| {
        //         let resources: &TriangleRenderResources = paint_callback_resources.get().unwrap();
        //         resources.prepare(device, queue, angle);
        //     })
        //     .paint(move |_info, rpass, paint_callback_resources| {
        //         let resources: &TriangleRenderResources = paint_callback_resources.get().unwrap();
        //         resources.paint(rpass);
        //     });
        //
        // let callback = egui::PaintCallback {
        //     rect,
        //     callback: Arc::new(cb),
        // };
        //
        // ui.painter().add(callback);
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
