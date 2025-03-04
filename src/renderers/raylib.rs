use crate::render_commands::{RenderCommand, RenderCommandConfig};
use raylib::{
    ffi::{BeginScissorMode, EndScissorMode},
    prelude::*,
};

macro_rules! clay_to_raylib_color {
    ($color:expr) => {
        ::raylib::color::Color::new(
            $color.r as u8,
            $color.g as u8,
            $color.b as u8,
            $color.a as u8,
        )
    };
}

macro_rules! clay_to_raylib_rect {
    ($rect:expr) => {
        ::raylib::math::Rectangle::new(
            $rect.x as f32,
            $rect.y as f32,
            $rect.width as f32,
            $rect.height as f32,
        )
    };
}

#[doc = "This is a direct* port of Clay's raylib renderer. See [the C implementation](https://github.com/nicbarker/clay/blob/main/renderers/raylib/clay_renderer_raylib.c) for more info."]
pub fn clay_raylib_render<'rl, 'a, CustomElementData: 'a>(
    d: &mut RaylibDrawHandle<'rl>,
    render_commands: impl Iterator<Item = RenderCommand<'a, Texture2D, CustomElementData>>,
    mut handle_custom_element: impl FnMut(&CustomElementData, &mut RaylibDrawHandle<'rl>),
) {
    for command in render_commands {
        match command.config {
            RenderCommandConfig::Text(text) => {
                let text_data = text.text;
                d.draw_text(
                    text_data,
                    command.bounding_box.x as i32,
                    command.bounding_box.y as i32,
                    text.font_size.into(),
                    clay_to_raylib_color!(text.color),
                );
            }

            RenderCommandConfig::Image(image) => {
                let texture = image.data;

                d.draw_texture_ex(
                    texture,
                    Vector2::new(command.bounding_box.x, command.bounding_box.y),
                    0.,
                    command.bounding_box.width / texture.width as f32,
                    // TODO: backgrond color isnt in raylib bindings?
                    clay_to_raylib_color!(Color::WHITE),
                );
            }

            // safety: raylib's BeginScissorMode is safe to call with any values.
            // we need to use this here because the regular begin_scissor_mode
            // ends the scissor mode on drop.
            RenderCommandConfig::ScissorStart() => unsafe {
                BeginScissorMode(
                    command.bounding_box.x as i32,
                    command.bounding_box.y as i32,
                    command.bounding_box.width as i32,
                    command.bounding_box.height as i32,
                );
            },

            RenderCommandConfig::ScissorEnd() => unsafe {
                EndScissorMode();
            },

            RenderCommandConfig::Rectangle(rect) => {
                if rect.corner_radii.top_left > 0. {
                    let radius = (rect.corner_radii.top_left * 2.)
                        / if command.bounding_box.width > command.bounding_box.height {
                            command.bounding_box.height
                        } else {
                            command.bounding_box.width
                        };

                    d.draw_rectangle_rounded(
                        clay_to_raylib_rect!(command.bounding_box),
                        radius,
                        8,
                        clay_to_raylib_color!(rect.color),
                    );
                } else {
                    d.draw_rectangle(
                        command.bounding_box.x as i32,
                        command.bounding_box.y as i32,
                        command.bounding_box.width as i32,
                        command.bounding_box.height as i32,
                        clay_to_raylib_color!(rect.color),
                    );
                }
            }

            RenderCommandConfig::Border(border) => {
                if border.width.left > 0 {
                    d.draw_rectangle(
                        command.bounding_box.x as i32,
                        (command.bounding_box.y + border.corner_radii.top_left) as i32,
                        border.width.left as i32,
                        (command.bounding_box.height
                            - border.corner_radii.top_left
                            - border.corner_radii.bottom_left) as i32,
                        clay_to_raylib_color!(border.color),
                    );
                }

                if border.width.right > 0 {
                    d.draw_rectangle(
                        (command.bounding_box.x + command.bounding_box.width
                            - border.width.right as f32) as i32,
                        (command.bounding_box.y + border.corner_radii.top_right) as i32,
                        border.width.right as i32,
                        (command.bounding_box.height
                            - border.corner_radii.top_right
                            - border.corner_radii.bottom_right) as i32,
                        clay_to_raylib_color!(border.color),
                    );
                }

                if border.width.top > 0 {
                    d.draw_rectangle(
                        (command.bounding_box.x + border.corner_radii.top_left) as i32,
                        command.bounding_box.y as i32,
                        (command.bounding_box.width
                            - border.corner_radii.top_left
                            - border.corner_radii.top_right) as i32,
                        border.width.top as i32,
                        clay_to_raylib_color!(border.color),
                    );
                }

                if border.width.bottom > 0 {
                    d.draw_rectangle(
                        (command.bounding_box.x + border.corner_radii.bottom_left) as i32,
                        (command.bounding_box.y + command.bounding_box.height
                            - border.width.bottom as f32) as i32,
                        (command.bounding_box.width
                            - border.corner_radii.bottom_left
                            - border.corner_radii.bottom_right) as i32,
                        border.width.bottom as i32,
                        clay_to_raylib_color!(border.color),
                    )
                }

                if border.corner_radii.top_left > 0. {
                    let vec = Vector2::new(
                        (command.bounding_box.x + border.corner_radii.top_left) as f32,
                        (command.bounding_box.y + border.corner_radii.top_left) as f32,
                    );

                    d.draw_ring(
                        vec,
                        border.corner_radii.top_left - border.width.top as f32,
                        border.corner_radii.top_left,
                        180.,
                        270.,
                        10,
                        clay_to_raylib_color!(border.color),
                    );
                }

                if border.corner_radii.top_right > 0. {
                    let vec = Vector2::new(
                        (command.bounding_box.x + command.bounding_box.width
                            - border.corner_radii.top_right) as f32,
                        (command.bounding_box.y + border.corner_radii.top_right) as f32,
                    );

                    d.draw_ring(
                        vec,
                        border.corner_radii.top_right - border.width.top as f32,
                        border.corner_radii.top_right,
                        270.,
                        360.,
                        10,
                        clay_to_raylib_color!(border.color),
                    );
                }

                if border.corner_radii.bottom_left > 0. {
                    let vec = Vector2::new(
                        (command.bounding_box.x + border.corner_radii.bottom_left) as f32,
                        (command.bounding_box.y + command.bounding_box.height
                            - border.corner_radii.bottom_left) as f32,
                    );

                    d.draw_ring(
                        vec,
                        border.corner_radii.bottom_left - border.width.bottom as f32,
                        border.corner_radii.bottom_left,
                        90.,
                        180.,
                        10,
                        clay_to_raylib_color!(border.color),
                    );
                }

                if border.corner_radii.bottom_right > 0. {
                    let vec = Vector2::new(
                        (command.bounding_box.x + command.bounding_box.width
                            - border.corner_radii.bottom_right) as f32,
                        (command.bounding_box.y + command.bounding_box.height
                            - border.corner_radii.bottom_right) as f32,
                    );

                    d.draw_ring(
                        vec,
                        border.corner_radii.bottom_right - border.width.bottom as f32,
                        border.corner_radii.bottom_right,
                        0.,
                        90.,
                        10,
                        clay_to_raylib_color!(border.color),
                    );
                }
            }
            RenderCommandConfig::Custom(custom) => handle_custom_element(custom.data, &mut *d),
            RenderCommandConfig::None() => {}
        }
    }
}
