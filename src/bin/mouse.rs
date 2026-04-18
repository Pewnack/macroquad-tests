use macroquad::prelude::*;

const FONT_SIZE: u16 = 40;
const INFO_FONT_SIZE: u16 = 28;

#[macroquad::main("Mouse Coordinates")]
async fn main() {
    let mut is_grabbed = false;

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::G) {
            is_grabbed = !is_grabbed;
            set_cursor_grab(is_grabbed);
        }

        let (x, y) = mouse_position();
        let text = format!("{x:.0}, {y:.0}");

        let text_dimensions = measure_text(&text, None, FONT_SIZE, 1.0);
        let text_x = (screen_width() - text_dimensions.width) * 0.5;
        let text_y = (screen_height() + text_dimensions.height) * 0.5;

        draw_text(&text, text_x, text_y, FONT_SIZE as f32, WHITE);

        let info_text = if is_grabbed {
            "Mouse is grabbed. Press G to release"
        } else {
            "Press G to grab the mouse"
        };
        let info_dimensions = measure_text(info_text, None, INFO_FONT_SIZE, 1.0);
        let info_x = (screen_width() - info_dimensions.width) * 0.5;
        let info_y = 40.0;

        draw_text(info_text, info_x, info_y, INFO_FONT_SIZE as f32, YELLOW);

        next_frame().await;
    }
}
