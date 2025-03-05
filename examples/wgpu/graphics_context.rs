use wgpu::{Device, Queue, RenderPass, SurfaceConfiguration};
use winit::window::Window;

pub struct GraphicsContext<'a> {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    depth_texture: DepthTexture,
    size: (i32, i32),
    pub window: Window,
}

impl<'a> GraphicsContext<'a> {
    pub fn new(window: Window) -> Self {
        let size = (
            window.inner_size().width as i32,
            window.inner_size().height as i32,
        );

        let instance = wgpu::Instance::default();

        let surface = unsafe {
            instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap())
        }
        .unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        ))
        .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0 as u32,
            height: size.1 as u32,
            present_mode: surface_capabilities.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let depth_texture = DepthTexture::new(&device, &config);

        Self {
            instance,
            window,
            surface,
            device,
            queue,
            config,
            size,
            depth_texture,
        }
    }

    pub fn render<F: FnOnce(&mut RenderPass, &Device, &Queue, &SurfaceConfiguration)>(
        &mut self,
        ui: F,
    ) -> Result<(), wgpu::SurfaceError> {
        let drawable = self.surface.get_current_texture()?;

        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &drawable
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default()),
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        };

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("RenderPass"),
                color_attachments: &[Some(color_attachment)],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            ui(&mut render_pass, &self.device, &self.queue, &self.config);
        }

        self.queue.submit(std::iter::once(command_encoder.finish()));
        drawable.present();
        Ok(())
    }

    pub fn resize(&mut self) {
        let new_size = (
            self.window.inner_size().width as i32,
            self.window.inner_size().height as i32,
        );

        if new_size.0 > 0 && new_size.1 > 0 {
            self.size = new_size;
            self.config.width = new_size.0 as u32;
            self.config.height = new_size.1 as u32;
            self.surface.configure(&self.device, &self.config);
        }

        self.depth_texture = DepthTexture::new(&self.device, &self.config);
    }

    pub fn _update_surface(&mut self) {
        let target = unsafe { wgpu::SurfaceTargetUnsafe::from_window(&self.window) }.unwrap();
        self.surface = unsafe { self.instance.create_surface_unsafe(target) }.unwrap();
    }
}

pub struct DepthTexture {
    #[allow(dead_code)]
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    #[allow(dead_code)]
    pub sampler: wgpu::Sampler,
}

impl DepthTexture {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                // 2.
                width: config.width.max(1),
                height: config.height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            label: Some("depth_texture"),
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            // 4.
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual), // 5.
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
        }
    }
}
