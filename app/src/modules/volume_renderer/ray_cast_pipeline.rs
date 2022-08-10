use std::path::Path;
use wgpu::util::DeviceExt;
use eframe::egui_wgpu;
use crate::modules::volume_renderer::camera::CameraBinding;
use crate::modules::volume_renderer::render_resources::{UniformBinding, Uniform};
use crate::utils::shader_compiler::ShaderCompiler;

pub struct RayCastPipeline {
    pub pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_count: usize,
}

impl RayCastPipeline {
    pub fn from_path(render_state: &egui_wgpu::RenderState,
                     path: &std::path::Path,
                     shader_compiler: &mut ShaderCompiler) -> Self {
        let shader = unsafe {
            render_state.device.create_shader_module_spirv(&wgpu::ShaderModuleDescriptorSpirV {
                label: path.to_str(),
                source: shader_compiler.create_shader_module(path).unwrap().into(),
            })
        };

        Self::new_with_module(render_state, &shader)
    }

    pub fn new_with_module(render_state: &egui_wgpu::RenderState, module: &wgpu::ShaderModule) -> Self {
        let vertices = [
            1., 1., 0., 0., 1., 0., 1., 1., 1., 0., 1., 1., 0., 0., 1., 0., 1., 0., 0., 0., 0., 1.,
            1., 0., 1., 0., 0., 1., 1., 1., 1., 0., 1., 0., 0., 1., 1., 0., 0., 0., 0., 0.,
        ];

        let vertex_buffer = render_state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Volume Vertex Buffer"),
            contents: bytemuck::cast_slice::<f32, _>(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let vertex_count = vertices.len() / 3;

        let pipeline = Self::make_pipeline(&render_state.device, &render_state.target_format, module);
        Self {
            pipeline,
            vertex_buffer,
            vertex_count,
        }
    }

    fn make_pipeline(device: &wgpu::Device, target_format: &wgpu::TextureFormat, module: &wgpu::ShaderModule) -> wgpu::RenderPipeline {
        let global_bind_group_layout = device.create_bind_group_layout(&Uniform::DESC);
        let camera_bind_group_layout = device.create_bind_group_layout(&CameraBinding::DESC);
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Foot BGL"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D3,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Screen Pass Layout"),
            bind_group_layouts: &[
                &global_bind_group_layout,
                &camera_bind_group_layout,
                &texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Raycast Pipeline"),
            layout: Some(&layout),
            fragment: Some(wgpu::FragmentState {
                module,
                entry_point: "fs_main",
                targets: &[Some((*target_format).into())],
            }),
            vertex: wgpu::VertexState {
                module,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 3 * 4,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                }],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                cull_mode: Some(wgpu::Face::Front),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        })
    }
}

impl<'a> RayCastPipeline {
    pub fn record<'pass>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'pass>,
        render_resources: &'a UniformBinding,
        camera_binding: &'a CameraBinding,
    )
        where 'a: 'pass {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        render_pass.set_bind_group(0, &render_resources.bind_group, &[]);
        render_pass.set_bind_group(1, &camera_binding.bind_group, &[]);
        render_pass.draw(0..self.vertex_count as _, 0..1);
    }
}