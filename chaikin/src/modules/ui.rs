use macroquad::{prelude::*, window};

pub fn draw_windows_ui(animation: bool, info_msg: &str) {
    draw_text(
        &format!("{} FPS", get_fps()),
        window::screen_width() - 100.,
        30.,
        30.,
        WHITE,
    );

    draw_text(
        &info_msg,
        (window::screen_width() / 2.) - (info_msg.len() * 7) as f32,
        window::screen_height() / 2.,
        30.,
        RED,
    );

    if !animation {
        draw_text("Left click to create points", 20.0, 30.0, 24.0, WHITE);
        draw_text("Right click to drage points", 20.0, 50.0, 24.0, WHITE);
    } else {
        draw_text(
            "Press SPACE to clear the windows.",
            20.0,
            30.0,
            24.0,
            ORANGE,
        );
    }
}
