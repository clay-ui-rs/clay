use clay_layout::{grow, renderers::clay_raylib_render, Clay, Declaration};
use raylib::prelude::*;

pub fn main() {
    let mut clay = Clay::new((800., 600.).into());

    let (mut rl, thread) = raylib::init()
        .resizable()
        .size(800, 600)
        .title("Clay Raylib Example")
        .build();

    while !rl.window_should_close() {
        clay.set_layout_dimensions(
            (rl.get_screen_width() as f32, rl.get_screen_height() as f32).into(),
        );

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        let clay = clay.begin::<_, ()>();

        #[rustfmt::skip]
        clay.with(
            &Declaration::new()
                .layout()
                    .width(grow!())
                    .height(grow!())
                .end(),
            |c| {
                c.with(
                    &Declaration::new()
                        .layout()
                            .width(grow!())
                            .height(grow!())
                        .end()
                        .corner_radius()
                            .all(24.)
                        .end()
                        .background_color((0xFF, 0x00, 0x00).into()),
                    |_| {}
                );

                c.with(
                    &Declaration::new()
                        .layout()
                            .width(grow!())
                            .height(grow!())
                        .end()
                        .corner_radius()
                            .all(24.)
                        .end()
                        .background_color((0x00, 0xFF, 0x00).into()),
                    |_| {}
                );
            },
        );

        let commands = clay.end();

        clay_raylib_render(&mut d, commands, |_, _| {})
    }
}
