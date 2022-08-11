use std::io::{Read, Write};
use std::num::NonZeroU32;
use egui::trace;

#[derive(Debug)]
pub struct VolumeTexture {
    pub texture: Option<wgpu::Texture>,
    pub bind_group: Option<wgpu::BindGroup>,
    pub sampler: Option<wgpu::Sampler>,
    pub path: Option<String>
}

impl Default for VolumeTexture {
    fn default() -> Self {
        Self {
            texture: None,
            bind_group: None,
            sampler: None,
            path: None
        }
    }
}

impl VolumeTexture {
    pub const DESC: wgpu::BindGroupLayoutDescriptor<'static> = wgpu::BindGroupLayoutDescriptor {
        label: Some("Volume Texture Description"),
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
    };

    #[tracing::instrument]
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, path: &std::path::Path) -> Self {
        let mut file = std::fs::File::open(path).unwrap();
        println!("Root Dir {:?}", std::path::Component::RootDir);
        let mut data: Vec<u8> = vec![];

        file.read_to_end(&mut data).expect("read file failed");

        let size = wgpu::Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 256
        };

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("Volume Texture"),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D3,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            }
        );
        let texture_view = texture.create_view(&Default::default());

        queue.write_texture(
            texture.as_image_copy(),
            data.as_slice(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(256),
                rows_per_image: NonZeroU32::new(256),
            },
            size);

        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("Present Sampler"),
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            },
        );

        let bind_group_layout = device.create_bind_group_layout(&Self::DESC);
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("Volume Texture Bind Group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler)
                    }
                ]
            }
        );

        Self {
            texture: texture.into(),
            bind_group: bind_group.into(),
            sampler: sampler.into(),
            path: Some(String::from(path.to_str().unwrap()))
        }
    }
}