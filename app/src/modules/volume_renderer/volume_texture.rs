use std::io::{Read, Write};
use std::num::NonZeroU32;
use egui::trace;

#[derive(Debug)]
pub struct VolumeTexture {
    pub texture: wgpu::Texture,
    pub bind_group: wgpu::BindGroup,
    pub sampler: wgpu::Sampler
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
        let file = std::fs::File::open(path);
        let mut data: Vec<u8> = vec![];
        tracing::debug_span!("hello");

        match file {
            Ok(mut file) => {
                let result = file.read_to_end(&mut data);

                match result {
                    Ok(size) => {
                        println!("{}", size);
                    },
                    Err(err) => {
                        panic!("write bytes failed!\n{}", err);
                    }
                }
            },
            Err(err) => {
                panic!("open file failed!\n{}", err);
            }
        };
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
            texture,
            bind_group,
            sampler
        }
    }
}