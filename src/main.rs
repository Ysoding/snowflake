use ::core::f64;

use raylib::ffi::Vector2;
use raylib::prelude::*;

const SCREEN_FACTOR: i32 = 80;
const SCREEN_WIDTH: i32 = 16 * SCREEN_FACTOR;
const SCREEN_HEIGHT: i32 = 9 * SCREEN_FACTOR;
const BRANCH_THICK: f32 = 10.0;
const BRANCH_COUNT: i32 = 7;
const BRANCH_ANGLE: f32 = ((2.0 * f64::consts::PI) / BRANCH_COUNT as f64) as f32;
const BRANCH_LEN: i32 = SCREEN_FACTOR * 2;

fn draw_snowflake(
    d: &mut RaylibDrawHandle,
    center: Vector2,
    level: i32,
    length: f32,
    thick: f32,
    hue: f32,
) {
    if level <= 0 {
        return;
    }

    let color = Color::color_from_hsv(hue, 1.0, 1.0);
    for i in 0..BRANCH_COUNT {
        let branch = Vector2 {
            x: center.x + (BRANCH_ANGLE * i as f32).cos() * length,
            y: center.y + (BRANCH_ANGLE * i as f32).sin() * length,
        };
        d.draw_line_ex(center, branch, thick, color);
        draw_snowflake(d, branch, level - 1, length * 0.5, thick * 0.5, hue + 70.0);
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Snowflake")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        let center = Vector2 {
            x: d.get_screen_width() as f32 / 2.0,
            y: d.get_screen_height() as f32 / 2.0,
        };
        draw_snowflake(&mut d, center, 5, BRANCH_LEN as f32, BRANCH_THICK, 5.0);
    }
}
