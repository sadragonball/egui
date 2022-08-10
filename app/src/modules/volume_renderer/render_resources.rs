use wgpu::util::DeviceExt;
use crate::utils::NonZeroSized;

pub struct UniformBinding {
    pub bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer
}

impl UniformBinding {
    pub fn new(device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Global Uniform"),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                contents: bytemuck::bytes_of(&Uniform::default()),
            }
        );

        let layout = device.create_bind_group_layout(&Uniform::DESC);

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Global Uniform Bind Group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding()
                }
            ],
        });

        Self {
            bind_group: uniform_bind_group,
            uniform_buffer: buffer
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, uniform: &Uniform) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(uniform));
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniform {
    pub position: [f32; 3],
    pub frame: u32,
    pub resolution: [f32; 2],
    pub mouse: [f32; 2],
    pub mouse_pressed: u32,
    pub time: f32,
    pub time_delta: f32,
    _padding: f32,
}

impl Default for Uniform {
    fn default() -> Self {
        Self {
            position: [0.; 3],
            time: 0.,
            resolution: [1920., 1080.],
            mouse: [0.; 2],
            mouse_pressed: false as _,
            frame: 0,
            time_delta: 1. / 60.,
            _padding: 0.,

        }
    }
}

impl Uniform {
    pub const DESC: wgpu::BindGroupLayoutDescriptor<'static> = wgpu::BindGroupLayoutDescriptor {
        label: Some("Global Uniform Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT.union(wgpu::ShaderStages::COMPUTE),
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: Some(Uniform::SIZE),
            },
            count: None,
        }],
    };

    #[allow(dead_code)]
    pub fn new(
        position: [f32; 3],
        resolution: [f32; 2],
        mouse: [f32; 2],
        mouse_pressed: u32,
        time: f32,
        time_delta: f32,
        frame: u32,
    ) -> Self {
        Self {
            position,
            resolution,
            mouse,
            mouse_pressed,
            time,
            time_delta,
            frame,
            _padding: 0.,
        }
    }
}