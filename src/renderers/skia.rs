use crate::math::{BoundingBox, Dimensions};
use crate::render_commands::{Custom, RenderCommand, RenderCommandConfig};
use crate::text::TextConfig;
use crate::{ClayLayoutScope, Color as ClayColor};

use skia_safe::{
    Canvas, ClipOp, Color, Font, Image, Paint, PaintCap, Point, RRect, Rect, SamplingOptions,
    Typeface,
};

pub fn clay_to_skia_color(color: ClayColor) -> Color {
    Color::from_argb(
        (color.a).round() as u8,
        (color.r).round() as u8,
        (color.g).round() as u8,
        (color.b).round() as u8,
    )
}

fn clay_to_skia_rect(rect: BoundingBox) -> Rect {
    Rect::from_xywh(rect.x, rect.y, rect.width, rect.height)
}
/// This is a direct* port of Clay's raylib renderer using skia_safe as the drawing API.
pub fn clay_skia_render<'a, CustomElementData: 'a>(
    canvas: &Canvas,
    render_commands: impl Iterator<Item = RenderCommand<'a, Image, CustomElementData>>,
    mut render_custom_element: impl FnMut(
        &RenderCommand<'a, Image, CustomElementData>,
        &Custom<'a, CustomElementData>,
        &Canvas,
    ),
    fonts: &[&Typeface],
) {
    for command in render_commands {
        match command.config {
            RenderCommandConfig::Text(text) => {
                let text_data = text.text;
                let mut paint = Paint::default();
                paint.set_color(clay_to_skia_color(text.color));
                let font = Font::new(fonts[text.font_id as usize].clone(), text.font_size as f32);
                let pos = Point::new(
                    command.bounding_box.x,
                    command.bounding_box.y + text.font_size as f32,
                );
                canvas.draw_str(&text_data, pos, &font, &paint);
            }

            RenderCommandConfig::Image(image) => {
                let skia_image = image.data;
                let mut paint = Paint::default();
                paint.set_color(Color::WHITE);
                paint.set_anti_alias(true);

                let bounds = clay_to_skia_rect(command.bounding_box);
                let has_border_radius = image.corner_radii.top_left > 0.
                    || image.corner_radii.top_right > 0.
                    || image.corner_radii.bottom_left > 0.
                    || image.corner_radii.bottom_right > 0.;
                if has_border_radius {
                    canvas.save();
                    let rrect = RRect::new_rect_radii(
                        bounds,
                        &[
                            Point::new(image.corner_radii.top_left, image.corner_radii.top_left),
                            Point::new(image.corner_radii.top_right, image.corner_radii.top_right),
                            Point::new(
                                image.corner_radii.bottom_left,
                                image.corner_radii.bottom_left,
                            ),
                            Point::new(
                                image.corner_radii.bottom_right,
                                image.corner_radii.bottom_right,
                            ),
                        ],
                    );
                    canvas.clip_rrect(rrect, ClipOp::Intersect, true);
                }

                canvas.draw_image_rect_with_sampling_options(
                    skia_image,
                    None,
                    bounds,
                    SamplingOptions::new(
                        skia_safe::FilterMode::Linear,
                        skia_safe::MipmapMode::Linear,
                    ),
                    &paint,
                );

                // Restore canvas state if we applied a clip
                if has_border_radius {
                    canvas.restore();
                }
            }

            RenderCommandConfig::ScissorStart() => {
                // Save the current state then clip to the bounding box.
                canvas.save();
                let clip_rect = clay_to_skia_rect(command.bounding_box);
                canvas.clip_rect(clip_rect, ClipOp::Intersect, false);
            }

            RenderCommandConfig::ScissorEnd() => {
                // Restore the previous state
                canvas.restore();
            }

            RenderCommandConfig::Rectangle(rect) => {
                let paint = {
                    let mut p = Paint::default();
                    p.set_color(clay_to_skia_color(rect.color));
                    p.set_anti_alias(true);
                    p.set_style(skia_safe::PaintStyle::Fill);
                    p
                };
                let bounds = clay_to_skia_rect(command.bounding_box);
                if rect.corner_radii.top_left > 0.
                    || rect.corner_radii.top_right > 0.
                    || rect.corner_radii.bottom_left > 0.
                    || rect.corner_radii.bottom_right > 0.
                {
                    let rrect = RRect::new_rect_radii(
                        bounds,
                        &[
                            Point::new(rect.corner_radii.top_left, rect.corner_radii.top_left),
                            Point::new(rect.corner_radii.top_right, rect.corner_radii.top_right),
                            Point::new(
                                rect.corner_radii.bottom_left,
                                rect.corner_radii.bottom_left,
                            ),
                            Point::new(
                                rect.corner_radii.bottom_right,
                                rect.corner_radii.bottom_right,
                            ),
                        ],
                    );
                    canvas.draw_rrect(rrect, &paint);
                } else {
                    canvas.draw_rect(bounds, &paint);
                }
            }

            RenderCommandConfig::Border(border) => {
                // Draw each border side using fill rectangles.
                let paint = {
                    let mut p = Paint::default();
                    p.set_color(clay_to_skia_color(border.color));
                    p.set_anti_alias(true);
                    p
                };

                let bb = &command.bounding_box;

                // Left border.
                if border.width.left > 0 {
                    let rect = Rect::from_xywh(
                        bb.x,
                        bb.y + border.corner_radii.top_left,
                        border.width.left as f32,
                        bb.height - border.corner_radii.top_left - border.corner_radii.bottom_left,
                    );
                    canvas.draw_rect(rect, &paint);
                }

                // Right border.
                if border.width.right > 0 {
                    let rect = Rect::from_xywh(
                        bb.x + bb.width - border.width.right as f32,
                        bb.y + border.corner_radii.top_right,
                        border.width.right as f32,
                        bb.height
                            - border.corner_radii.top_right
                            - border.corner_radii.bottom_right,
                    );
                    canvas.draw_rect(rect, &paint);
                }

                // Top border.
                if border.width.top > 0 {
                    let rect = Rect::from_xywh(
                        bb.x + border.corner_radii.top_left,
                        bb.y,
                        bb.width - border.corner_radii.top_left - border.corner_radii.top_right,
                        border.width.top as f32,
                    );
                    canvas.draw_rect(rect, &paint);
                }

                // Bottom border.
                if border.width.bottom > 0 {
                    let rect = Rect::from_xywh(
                        bb.x + border.corner_radii.bottom_left,
                        bb.y + bb.height - border.width.bottom as f32,
                        bb.width
                            - border.corner_radii.bottom_left
                            - border.corner_radii.bottom_right,
                        border.width.bottom as f32,
                    );
                    canvas.draw_rect(rect, &paint);
                }

                // For corner arcs, we draw strokes.
                let mut stroke = Paint::default();
                stroke.set_color(clay_to_skia_color(border.color));

                stroke.set_style(skia_safe::paint::Style::Stroke);
                stroke.set_anti_alias(true);

                // Helper to draw an arc.
                let mut draw_corner_arc = |canvas: &Canvas,
                                           center_x: f32,
                                           center_y: f32,
                                           radius: f32,
                                           start_angle: f32,
                                           sweep_angle: f32,
                                           width: u16| {
                    let radius = radius - (width as f32 / 2.);
                    let arc_rect = Rect::from_xywh(
                        center_x - radius,
                        center_y - radius,
                        radius * 2.0,
                        radius * 2.0,
                    );

                    stroke.set_stroke_width(width as f32);
                    stroke.set_stroke_cap(PaintCap::Round);
                    canvas.draw_arc(arc_rect, start_angle, sweep_angle, false, &stroke);
                };

                if border.corner_radii.top_left > 0. {
                    // top-left: arc from 180 to 270 degrees.
                    let center_x = bb.x + border.corner_radii.top_left;
                    let center_y = bb.y + border.corner_radii.top_left;

                    draw_corner_arc(
                        canvas,
                        center_x,
                        center_y,
                        border.corner_radii.top_left,
                        180.0,
                        90.0 / 2.,
                        border.width.left,
                    );

                    draw_corner_arc(
                        canvas,
                        center_x,
                        center_y,
                        border.corner_radii.top_left,
                        180.0 + 90.0 / 2.,
                        90.0 / 2.,
                        border.width.top,
                    );
                }

                if border.corner_radii.top_right > 0. {
                    // top-right: arc from 270 to 360 degrees.
                    let center_x = bb.x + bb.width - border.corner_radii.top_right;
                    let center_y = bb.y + border.corner_radii.top_right;

                    draw_corner_arc(
                        canvas,
                        center_x,
                        center_y,
                        border.corner_radii.top_right,
                        270.0,
                        90.0 / 2.0,
                        border.width.top,
                    );
                    draw_corner_arc(
                        canvas,
                        center_x,
                        center_y,
                        border.corner_radii.top_right,
                        270.0 + 90. / 2.,
                        90.0 / 2.0,
                        border.width.right,
                    );
                }

                if border.corner_radii.bottom_left > 0. {
                    // bottom-left: arc from 90 to 180 degrees.
                    let center_x = bb.x + border.corner_radii.bottom_left;
                    let center_y = bb.y + bb.height - border.corner_radii.bottom_left;

                    draw_corner_arc(
                        canvas,
                        center_x,
                        center_y,
                        border.corner_radii.bottom_left,
                        90.0,
                        90.0 / 2.,
                        border.width.bottom,
                    );

                    draw_corner_arc(
                        canvas,
                        center_x,
                        center_y,
                        border.corner_radii.bottom_left,
                        90.0 + 90. / 2.,
                        90.0 / 2.,
                        border.width.left,
                    );
                }

                if border.corner_radii.bottom_right > 0. {
                    // bottom-right: arc from 0 to 90 degrees.
                    let center_x = bb.x + bb.width - border.corner_radii.bottom_right;
                    let center_y = bb.y + bb.height - border.corner_radii.bottom_right;

                    draw_corner_arc(
                        canvas,
                        center_x,
                        center_y,
                        border.corner_radii.bottom_right,
                        0.,
                        90.0 / 2.,
                        border.width.right,
                    );
                    draw_corner_arc(
                        canvas,
                        center_x,
                        center_y,
                        border.corner_radii.bottom_right,
                        90.0 / 2.,
                        90.0 / 2.,
                        border.width.bottom,
                    );
                }
            }
            RenderCommandConfig::Custom(ref custom) => {
                render_custom_element(&command, custom, canvas)
            }
            RenderCommandConfig::None() => {}
        }
    }
}

pub type SkiaClayScope<'clay, 'render, CustomElements> =
    ClayLayoutScope<'clay, 'render, Image, CustomElements>;

pub fn get_source_dimensions_from_skia_image(image: &Image) -> Dimensions {
    (image.width() as f32, image.height() as f32).into()
}

pub fn create_measure_text_function(
    fonts: &'static [&Typeface],
) -> impl Fn(&str, &TextConfig) -> Dimensions + 'static {
    |text, text_config| {
        let font = Font::new(
            fonts[text_config.font_id as usize],
            text_config.font_size as f32,
        );
        let width = font.measure_str(text, None).0;
        (width, font.metrics().1.bottom - font.metrics().1.top).into()
    }
}
