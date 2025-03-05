use core::f32;
use glyphon::{
    cosmic_text, Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Resolution, Shaping,
    SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};
use std::ops::{Add, Mul, Sub};
use wgpu::util::DeviceExt;
use wgpu::MultisampleState;
use winit::dpi::PhysicalSize;

use clay_layout::math::Dimensions;
use clay_layout::render_commands::RenderCommand;

pub struct TextLine {
    line: glyphon::Buffer,
    left: f32,
    top: f32,
    color: Color,
    bounds: Option<(UIPosition, UIPosition)>,
}

#[derive(Debug, Clone)]
pub struct UICornerRadii {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct UIColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct UIBorderThickness {
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct UIPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl UIPosition {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn rotate(&mut self, mut degrees: f32) {
        degrees = -degrees;

        degrees = degrees * (std::f32::consts::PI / 180.0);

        let (sn, cs) = degrees.sin_cos();

        let new = UIPosition {
            x: self.x * cs - self.y * sn,
            y: self.x * sn + self.y * cs,
            z: self.z,
        };
        *self = new;
    }

    pub fn with_x(&mut self, x: f32) -> UIPosition {
        UIPosition {
            x: self.x + x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn with_y(&mut self, y: f32) -> UIPosition {
        UIPosition {
            x: self.x,
            y: self.y + y,
            z: self.z,
        }
    }

    pub fn add_x(&mut self, x: f32) -> &mut Self {
        self.x += x;

        self
    }

    pub fn add_y(&mut self, y: f32) -> &mut Self {
        self.y += y;

        self
    }
}

impl Add for UIPosition {
    type Output = UIPosition;

    fn add(self, other: UIPosition) -> UIPosition {
        UIPosition {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z,
        }
    }
}

impl Add<f32> for UIPosition {
    type Output = UIPosition;

    fn add(self, rhs: f32) -> UIPosition {
        UIPosition {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z,
        }
    }
}

impl Sub<f32> for UIPosition {
    type Output = UIPosition;

    fn sub(self, rhs: f32) -> UIPosition {
        UIPosition {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z,
        }
    }
}

impl Mul<f32> for UIPosition {
    type Output = UIPosition;

    fn mul(self, rhs: f32) -> Self::Output {
        UIPosition {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z,
        }
    }
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct UISize {
    pub width: f32,
    pub height: f32,
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct UIVertex {
    pub position: UIPosition,
    pub color: UIColor,
    pub size: UISize,
}

impl UIVertex {
    pub fn new(size: (i32, i32)) -> Self {
        Self {
            position: UIPosition {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            color: UIColor {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            },
            size: UISize {
                width: size.0 as f32,
                height: size.1 as f32,
            },
        }
    }

    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTR: [wgpu::VertexAttribute; 3] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<UIVertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTR,
        }
    }
}

pub struct UIState {
    pub vertices: Vec<UIVertex>,
    pub buffer: wgpu::Buffer,
    pub number_of_vertices: usize,

    render_pipeline: wgpu::RenderPipeline,

    pub font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: glyphon::Viewport,
    atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,
    pub measurement_buffer: glyphon::Buffer,
    pub lines: Vec<TextLine>,

    pub dpi_scale: f32,
}

impl UIState {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pixel_format: wgpu::TextureFormat,
        size: PhysicalSize<u32>,
        dpi_scale: f32,
    ) -> Self {
        let (buffer, vertices) = make_ui_buffer(
            device,
            "ui triangle buffer",
            10000,
            (size.width as i32, size.height as i32),
        );

        let mut ui_pipeline_builder = UIPipeline::new(pixel_format);
        ui_pipeline_builder.add_buffer_layout(UIVertex::get_layout());
        let render_pipeline = ui_pipeline_builder.build_pipeline(&device);

        let mut font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(&device);
        let viewport = Viewport::new(&device, &cache);
        let mut atlas = TextAtlas::new(&device, &queue, &cache, pixel_format);
        let text_renderer = TextRenderer::new(
            &mut atlas,
            &device,
            MultisampleState::default(),
            Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
        );
        let measurement_buffer = Buffer::new(&mut font_system, Metrics::new(30.0, 42.0));

        Self {
            vertices,
            buffer,
            number_of_vertices: 0,
            render_pipeline,

            font_system,
            swash_cache,
            viewport,
            atlas,
            text_renderer,
            measurement_buffer,
            lines: Vec::<TextLine>::new(),
            dpi_scale,
        }
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass, queue: &wgpu::Queue) {
        render_pass.set_pipeline(&self.render_pipeline);

        queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&self.vertices.get(0..self.number_of_vertices).unwrap()),
        );

        render_pass.set_vertex_buffer(0, self.buffer.slice(..));
        render_pass.draw(0..self.number_of_vertices as u32, 0..1);

        self.number_of_vertices = 0;
    }

    fn render_text(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass,
        surface_config: &wgpu::SurfaceConfiguration,
    ) {
        self.atlas.trim();

        self.viewport.update(
            &queue,
            Resolution {
                width: surface_config.width,
                height: surface_config.height,
            },
        );

        let mut areas = Vec::<TextArea>::new();

        for text_line in self.lines.iter_mut() {
            areas.push(TextArea {
                buffer: &text_line.line,
                left: text_line.left,
                top: text_line.top,
                scale: 1.0,
                bounds: match text_line.bounds {
                    Some((position, bounds)) => TextBounds {
                        left: position.x as i32,
                        top: position.y as i32,
                        right: (position.x + bounds.x) as i32,
                        bottom: (position.y + bounds.y) as i32,
                    },
                    None => TextBounds {
                        left: 0,
                        top: 0,
                        right: surface_config.width as i32,
                        bottom: surface_config.height as i32,
                    },
                },
                default_color: text_line.color,
                custom_glyphs: &[],
            });
        }

        self.text_renderer
            .prepare_with_depth(
                device,
                queue,
                &mut self.font_system,
                &mut self.atlas,
                &mut self.viewport,
                areas.into_iter(),
                &mut self.swash_cache,
                |metadata| (metadata as f32) / 10000.0,
            )
            .unwrap();

        self.text_renderer
            .render(&self.atlas, &self.viewport, render_pass)
            .unwrap();

        self.lines.clear();
    }

    #[allow(dead_code)]
    pub fn measure_text(&mut self, text: &str, font_size: f32, line_height: f32) -> Dimensions {
        self.measurement_buffer.set_metrics_and_size(
            &mut self.font_system,
            Metrics {
                font_size: font_size * self.dpi_scale,
                line_height: match line_height {
                    0.0 => (font_size * 1.5) * self.dpi_scale,
                    _ => line_height * self.dpi_scale,
                },
            },
            None,
            None,
        );
        self.measurement_buffer.set_text(
            &mut self.font_system,
            text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
        self.measurement_buffer
            .shape_until_scroll(&mut self.font_system, false);

        (
            self.measurement_buffer.layout_runs().next().unwrap().line_w,
            self.measurement_buffer.metrics().line_height,
        )
            .into()
    }

    pub fn resize(&mut self, size: (i32, i32)) {
        for vertex in self.vertices.as_mut_slice() {
            vertex.size.width = size.0 as f32;
            vertex.size.height = size.1 as f32;
        }
    }

    pub fn triangle(&mut self, positions: &[UIPosition; 3], color: UIColor) {
        match self
            .vertices
            .get_mut(self.number_of_vertices..self.number_of_vertices + 3)
        {
            None => return,
            Some(vertices) => {
                for (vertex, position) in vertices.iter_mut().zip(positions.iter()) {
                    vertex.position = *position;
                    vertex.color = color;
                    self.number_of_vertices += 1;
                }
            }
        }
    }

    pub fn quad(&mut self, positions: &[UIPosition; 4], color: UIColor) {
        match self
            .vertices
            .get_mut(self.number_of_vertices..self.number_of_vertices + 6)
        {
            None => return,
            Some(vertices) => {
                vertices.get_mut(0).unwrap().position = positions[0];
                vertices.get_mut(0).unwrap().color = color;

                vertices.get_mut(1).unwrap().position = positions[1];
                vertices.get_mut(1).unwrap().color = color;

                vertices.get_mut(2).unwrap().position = positions[2];
                vertices.get_mut(2).unwrap().color = color;

                vertices.get_mut(3).unwrap().position = positions[0];
                vertices.get_mut(3).unwrap().color = color;

                vertices.get_mut(4).unwrap().position = positions[2];
                vertices.get_mut(4).unwrap().color = color;

                vertices.get_mut(5).unwrap().position = positions[3];
                vertices.get_mut(5).unwrap().color = color;

                self.number_of_vertices += 6;
            }
        }
    }

    pub fn line(
        &mut self,
        position: UIPosition,
        length: f32,
        angle: f32,
        thickness: f32,
        color: UIColor,
    ) {
        let mut line: [UIPosition; 4] = [UIPosition::new(); 4];

        line[0].add_y(-(thickness / 2.0));
        line[1].add_y(thickness / 2.0);
        line[2].add_x(length).add_y(thickness / 2.0);
        line[3].add_x(length).add_y(-(thickness / 2.0));

        for point in line.iter_mut() {
            point.rotate(angle);
            *point = *point + position;
        }

        self.quad(&line, color);
    }

    pub fn arc(
        &mut self,
        origin: UIPosition,
        radius: f32,
        degree_begin: f32,
        degree_end: f32,
        thickness: f32,
        color: UIColor,
    ) {
        let arc_length = (degree_end - degree_begin).abs();
        let number_of_segments = 10.0;
        let arc_segment_length = arc_length / number_of_segments; // 10 = number of segments
        let arc_segment_distance =
            (2.0 * std::f32::consts::PI * radius) * (arc_segment_length / 360.0);

        let mut arc_point = UIPosition {
            x: 0.0,
            y: 0.0,
            z: origin.z,
        };

        for i in 0..number_of_segments as i32 {
            arc_point.x = radius;
            arc_point.y = 0.0;
            arc_point.rotate(degree_begin + (arc_segment_length * (i as f32)));

            arc_point = arc_point + origin;

            self.line(
                arc_point,
                arc_segment_distance,
                degree_begin + 90.0 + (arc_segment_length * i as f32) + (arc_segment_length / 2.0),
                thickness,
                color,
            );
        }
    }

    pub fn filled_arc(
        &mut self,
        origin: UIPosition,
        radius: f32,
        degree_begin: f32,
        degree_end: f32,
        color: UIColor,
    ) {
        let arc_length = (degree_end - degree_begin).abs();
        let number_of_segments = 10.0;
        let arc_segment_length = arc_length / number_of_segments; // 10 = number of segments

        let mut current_point = UIPosition {
            x: 0.0,
            y: 0.0,
            z: origin.z,
        };
        let mut next_point = UIPosition {
            x: 0.0,
            y: 0.0,
            z: origin.z,
        };

        for i in 0..number_of_segments as i32 {
            current_point.x = radius;
            current_point.y = 0.0;
            current_point.rotate(degree_begin + (arc_segment_length * (i as f32 + 1.0)));

            next_point.x = radius;
            next_point.y = 0.0;
            next_point.rotate(degree_begin + (arc_segment_length * (i as f32 + 0.0)));

            self.triangle(
                &[current_point + origin, origin, next_point + origin],
                color,
            );
        }
    }

    pub fn rectangle(
        &mut self,
        mut position: UIPosition,
        size: UIPosition,
        thickness: UIBorderThickness,
        color: UIColor,
        radii: UICornerRadii,
    ) {
        self.arc(
            position + radii.top_left,
            radii.top_left,
            90.0,
            180.0,
            thickness.top,
            color,
        );
        self.arc(
            position
                .with_x(size.x - radii.top_right)
                .with_y(radii.top_right),
            radii.top_right,
            0.0,
            90.0,
            thickness.top,
            color,
        );
        self.arc(
            position
                .with_y(size.y - radii.bottom_left)
                .with_x(radii.bottom_left),
            radii.bottom_left,
            180.0,
            270.0,
            thickness.bottom,
            color,
        );
        self.arc(
            position + (size - radii.bottom_right),
            radii.bottom_right,
            270.0,
            360.0,
            thickness.bottom,
            color,
        );

        self.line(
            position.with_x(radii.top_left),
            size.x - (radii.top_left + radii.top_right),
            0.0,
            thickness.top,
            color,
        );
        self.line(
            position.with_y(radii.top_left),
            size.y - (radii.top_left + radii.bottom_left),
            270.0,
            thickness.left,
            color,
        );
        self.line(
            position.with_x(radii.bottom_left).with_y(size.y),
            size.x - (radii.bottom_left + radii.bottom_right),
            0.0,
            thickness.bottom,
            color,
        );
        self.line(
            position.with_x(size.x).with_y(radii.top_right),
            size.y - (radii.top_right + radii.bottom_right),
            270.0,
            thickness.right,
            color,
        );
    }

    #[allow(dead_code)]
    pub fn filled_rectangle(
        &mut self,
        mut position: UIPosition,
        size: UIPosition,
        color: UIColor,
        radii: UICornerRadii,
    ) {
        self.filled_arc(
            position + radii.top_left,
            radii.top_left,
            90.0,
            180.0,
            color,
        );
        self.filled_arc(
            position
                .with_x(size.x - radii.top_right)
                .with_y(radii.top_right),
            radii.top_right,
            0.0,
            90.0,
            color,
        );
        self.filled_arc(
            position
                .with_y(size.y - radii.bottom_left)
                .with_x(radii.bottom_left),
            radii.bottom_left,
            180.0,
            270.0,
            color,
        );
        self.filled_arc(
            position + (size - radii.top_right),
            radii.bottom_right,
            270.0,
            360.0,
            color,
        );

        // top
        self.quad(
            &[
                position.with_x(radii.top_left),
                position + radii.top_left,
                position
                    .with_x(size.x - radii.top_right)
                    .with_y(radii.top_right),
                position.with_x(size.x - radii.top_right),
            ],
            color,
        );
        // bottom
        self.quad(
            &[
                position
                    .with_x(radii.bottom_left)
                    .with_y(size.y - radii.bottom_left),
                position.with_x(radii.bottom_left).with_y(size.y),
                position.with_x(size.x - radii.bottom_right).with_y(size.y),
                position
                    .with_x(size.x - radii.bottom_right)
                    .with_y(size.y - radii.bottom_right),
            ],
            color,
        );
        // left
        self.quad(
            &[
                position.with_y(radii.top_left),
                position.with_y(size.y - radii.bottom_left),
                position
                    .with_x(radii.bottom_left)
                    .with_y(size.y - radii.bottom_left),
                position + radii.top_left,
            ],
            color,
        );
        // right
        self.quad(
            &[
                position.with_x(size.x - radii.top_right),
                position
                    .with_x(size.x - radii.bottom_right)
                    .with_y(size.y - radii.bottom_right),
                position.with_x(size.x).with_y(size.y - radii.bottom_right),
                position.with_x(size.x).with_y(radii.top_right),
            ],
            color,
        );
        // center
        self.quad(
            &[
                position + radii.top_left,
                position
                    .with_x(radii.bottom_left)
                    .with_y(size.y - radii.bottom_left),
                position
                    .with_x(size.x - radii.bottom_right)
                    .with_y(size.y - radii.bottom_right),
                position
                    .with_x(size.x - radii.top_right)
                    .with_y(radii.top_right),
            ],
            color,
        );
    }

    pub fn text(
        &mut self,
        text: &str,
        font_size: f32,
        line_height: f32,
        position: UIPosition,
        bounds: Option<(UIPosition, UIPosition)>,
        color: cosmic_text::Color,
        draw_order: f32,
    ) {
        let mut line = Buffer::new(&mut self.font_system, Metrics::new(font_size, line_height));

        line.set_text(
            &mut self.font_system,
            text,
            Attrs::new()
                .family(Family::SansSerif)
                .metadata((draw_order * 10000.0) as usize),
            Shaping::Advanced,
        );

        line.shape_until_scroll(&mut self.font_system, false);

        self.lines.push(TextLine {
            line,
            left: position.x,
            top: position.y,
            color,
            bounds,
        });
    }

    pub fn render_clay<'a, ImageElementData: 'a, CustomElementData: 'a>(
        &mut self,
        render_commands: impl Iterator<Item = RenderCommand<'a, ImageElementData, CustomElementData>>,
        render_pass: &mut wgpu::RenderPass,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_config: &wgpu::SurfaceConfiguration,
    ) {
        let mut scissor_position = UIPosition::new();
        let mut scissor_bounds = UIPosition::new();
        let mut scissor_active = false;
        let mut depth: f32 = 0.1;

        for command in render_commands {
            match command.config {
                clay_layout::render_commands::RenderCommandConfig::Rectangle(r) => {
                    self.filled_rectangle(
                        UIPosition {
                            x: command.bounding_box.x,
                            y: command.bounding_box.y,
                            z: depth as f32,
                        },
                        UIPosition {
                            x: command.bounding_box.width,
                            y: command.bounding_box.height,
                            z: depth as f32,
                        },
                        UIColor {
                            r: r.color.r / 255.0,
                            g: r.color.g / 255.0,
                            b: r.color.b / 255.0,
                        },
                        UICornerRadii {
                            top_left: r.corner_radii.top_left,
                            top_right: r.corner_radii.top_right,
                            bottom_left: r.corner_radii.bottom_left,
                            bottom_right: r.corner_radii.bottom_right,
                        },
                    );
                }
                clay_layout::render_commands::RenderCommandConfig::Border(b) => {
                    self.rectangle(
                        UIPosition {
                            x: command.bounding_box.x,
                            y: command.bounding_box.y,
                            z: depth as f32,
                        },
                        UIPosition {
                            x: command.bounding_box.width,
                            y: command.bounding_box.height,
                            z: depth as f32,
                        },
                        UIBorderThickness {
                            top: (b.width.top as f32),
                            left: (b.width.left as f32),
                            bottom: (b.width.bottom as f32),
                            right: (b.width.right as f32),
                        },
                        UIColor {
                            r: b.color.r / 255.0,
                            g: b.color.g / 255.0,
                            b: b.color.b / 255.0,
                        },
                        UICornerRadii {
                            top_left: (b.corner_radii.top_left),
                            top_right: (b.corner_radii.top_right),
                            bottom_left: b.corner_radii.bottom_left,
                            bottom_right: b.corner_radii.bottom_right,
                        },
                    );
                }
                clay_layout::render_commands::RenderCommandConfig::Text(text) => {
                    self.text(
                        text.text,
                        (text.font_size as f32) * self.dpi_scale,
                        match text.line_height {
                            0 => (text.font_size as f32) * 1.5 * self.dpi_scale,
                            _ => (text.line_height as f32) * self.dpi_scale,
                        },
                        UIPosition {
                            x: command.bounding_box.x,
                            y: command.bounding_box.y,
                            z: depth as f32,
                        },
                        match scissor_active {
                            true => Some((scissor_position.clone(), scissor_bounds.clone())),
                            false => None,
                        },
                        Color::rgb(text.color.r as u8, text.color.g as u8, text.color.b as u8),
                        depth,
                    );
                }
                clay_layout::render_commands::RenderCommandConfig::ScissorStart() => {
                    scissor_position.x = command.bounding_box.x;
                    scissor_position.y = command.bounding_box.y;
                    scissor_bounds.x = command.bounding_box.width;
                    scissor_bounds.y = command.bounding_box.height;
                    scissor_active = true;
                }
                clay_layout::render_commands::RenderCommandConfig::ScissorEnd() => {
                    scissor_active = false;
                }
                _ => {}
            }
            depth -= 0.0001;
        }

        if self.number_of_vertices > 0 {
            self.render(render_pass, queue);
        }
        if self.lines.len() > 0 {
            self.render_text(device, queue, render_pass, surface_config);
        }
    }
}

fn make_ui_buffer(
    device: &wgpu::Device,
    label: &str,
    number_of_triangles: usize,
    size: (i32, i32),
) -> (wgpu::Buffer, Vec<UIVertex>) {
    let vertices: Vec<UIVertex> = vec![UIVertex::new(size); number_of_triangles * 3];

    let buffer_desctriptor = wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    };

    let buffer = device.create_buffer_init(&buffer_desctriptor);

    (buffer, vertices)
}

pub struct UIPipeline {
    pixel_format: wgpu::TextureFormat,
    vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'static>>,
}

impl UIPipeline {
    pub fn new(pixel_format: wgpu::TextureFormat) -> Self {
        Self {
            pixel_format,
            vertex_buffer_layouts: Vec::new(),
        }
    }

    pub fn add_buffer_layout(&mut self, layout: wgpu::VertexBufferLayout<'static>) {
        self.vertex_buffer_layouts.push(layout);
    }

    pub fn build_pipeline(&self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        // let mut filepath = current_dir().unwrap();
        // filepath.push(self.shader_file.as_str());
        // let filepath = filepath.into_os_string().into_string().unwrap();

        // let source_code = fs::read_to_string(filepath).expect("Can't read source code");
        let source_code = include_str!("ui_shader.wgsl");

        let shader_module_desc = wgpu::ShaderModuleDescriptor {
            label: Some("UI Shader Module"),
            source: wgpu::ShaderSource::Wgsl(source_code.into()),
        };
        let shader_module = device.create_shader_module(shader_module_desc);

        let piplaydesc = wgpu::PipelineLayoutDescriptor {
            label: Some("UI Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };
        let pipeline_layout = device.create_pipeline_layout(&piplaydesc);

        let render_targets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let render_pip_desc = wgpu::RenderPipelineDescriptor {
            label: Some("UI Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vs_main"),
                buffers: &self.vertex_buffer_layouts,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("fs_main"),
                targets: &render_targets,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Always, // 1.
                stencil: wgpu::StencilState::default(),       // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: 1,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        };

        device.create_render_pipeline(&render_pip_desc)
    }
}
