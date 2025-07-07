mod modules {
    pub mod pattern;
    pub mod point;
    pub mod ui;
}

use macroquad::prelude::*;
use modules::pattern::Pattern;
use modules::ui::draw_windows_ui;

#[macroquad::main("chaikin")]
async fn main() {
    let mut pattern: Pattern = Pattern::new();
    let mut clocker: f64 = 0.;

    let mut info_msg = String::new();

    loop {
        if pattern.draging {
            match pattern.dragable_point {
                Some((fixe_point, path_point)) => {
                    let new_position = mouse_position();
                    pattern.fixed_point[fixe_point].set_position(new_position);
                    pattern.path[path_point].set_position(new_position);
                }
                None => pattern.draging = false,
            };
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            info_msg = String::from("");
            if !pattern.animation_start && !pattern.draging {
                pattern.append_point(mouse_position());
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            if !pattern.draging && !pattern.animation_start {
                pattern.check_point_colision(mouse_position());
            } else {
                pattern.draging = false;
            }
        }

        if pattern.animation_start {
            pattern.path = pattern.final_path[pattern.frame].clone();
            if (get_time() - clocker) >= 0.5 {
                pattern.frame = (pattern.frame + 1) % pattern.final_path.len();
                clocker = get_time();
            }
        }

        if is_key_pressed(KeyCode::Enter)
            && pattern.animation_start == false
            && pattern.path.len() > 2
        {
            pattern.chaikin();
        }
        if is_key_pressed(KeyCode::Enter)
            && pattern.animation_start == false
            && pattern.path.len() <= 1
        {
            info_msg = String::from("Animation must have 2 point to start !");
        }

        if is_key_pressed(KeyCode::Space) {
            pattern = Pattern::new();
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        pattern.draw_path();
        draw_windows_ui(pattern.animation_start, &info_msg);

        next_frame().await;
    }
}
