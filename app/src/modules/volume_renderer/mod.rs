use std::sync::Arc;
use std::time::Duration;
use wgpu::Device;
use wgpu::util::DeviceExt;
use eframe::{egui_wgpu, Storage};
use eframe::epaint::Rgba;
use egui::{Visuals};
use crate::modules::volume_renderer::camera::{Camera, CameraBinding};
use crate::modules::volume_renderer::ray_cast_pipeline::RayCastPipeline;
use crate::modules::volume_renderer::global_uniform::{GlobalUniform, GlobalUniformBinding};
use crate::modules::volume_renderer::volume_texture::VolumeTexture;
use crate::utils::shader_compiler::ShaderCompiler;

mod camera;
mod ray_cast_pipeline;
mod global_uniform;
mod volume_texture;

//待定
// trait BasicPipeline {
//     fn from_path(&self,
//                  device: &wgpu::Device,
//                  path: &std::path::Path,
//                  shader_compiler: &mut crate::utils::ShaderCompiler) -> Self;
// }

pub struct VolumeRenderer {
    drag_delta: egui::Vec2,
}


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
        Self {
            drag_delta: egui::Vec2::default()
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
            ui.allocate_exact_size(
                egui::Vec2::new(Self::WIDTH as f32, Self::HEIGHT as f32),
                egui::Sense::drag()
            );
        self.drag_delta = response.drag_delta();

        let drag_delta = self.drag_delta;

        let func = egui_wgpu::CallbackFn::new()
            .prepare(move |device, queue, paint_callback_resources| {
                let resources: &mut RenderResources = paint_callback_resources.get_mut().unwrap();
                resources.prepare(device, queue, drag_delta);
            })
            .paint(move |_info, rpass, paint_callback_resources| {
                let resources: &RenderResources = paint_callback_resources.get().unwrap();
                resources.paint(rpass);
            });

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(func)
        };

        ui.painter().add(callback);
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
    pub const ROTATE_SPEED: f32 = 0.005f32;
    fn prepare(&mut self, _device: &wgpu::Device, queue: &wgpu::Queue, drag_delta: egui::Vec2) {
        // Update our uniform buffer with the angle from the UI
        self.camera.add_yaw(-drag_delta.x as f32 * Self::ROTATE_SPEED);
        self.camera.add_pitch(-drag_delta.y as f32 * Self::ROTATE_SPEED);
        self.camera_binding.update(queue, &mut self.camera);
    }

    fn paint<'rpass>(&'rpass self, rpass: &mut wgpu::RenderPass<'rpass>) {
        // Draw our volume!
        self.pipeline.record(rpass,
                             &self.global_uniform_binding,
                             &self.camera_binding,
                             &self.volume_texture);
    }
}
