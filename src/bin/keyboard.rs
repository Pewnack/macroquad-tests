use macroquad::prelude::*;

const FONT_SIZE: u16 = 40;
const QUALIFIER_FONT_SIZE: u16 = 28;

fn is_modifier(key: KeyCode) -> bool {
    matches!(
        key,
        KeyCode::LeftShift
            | KeyCode::RightShift
            | KeyCode::LeftControl
            | KeyCode::RightControl
            | KeyCode::LeftAlt
            | KeyCode::RightAlt
            | KeyCode::LeftSuper
            | KeyCode::RightSuper
    )
}

fn key_name(key: KeyCode) -> String {
    format!("{key:?}")
}

#[macroquad::main("Keyboard")]
async fn main() {
    loop {
        clear_background(BLACK);

        let keys_down = get_keys_down();

        let modifiers: Vec<String> = keys_down
            .iter()
            .filter(|&&k| is_modifier(k))
            .map(|&k| key_name(k))
            .collect();

        let regular_keys: Vec<String> = keys_down
            .iter()
            .filter(|&&k| !is_modifier(k))
            .map(|&k| key_name(k))
            .collect();

        if !regular_keys.is_empty() {
            let text = regular_keys.join(" + ");
            let dims = measure_text(&text, None, FONT_SIZE, 1.0);
            let x = (screen_width() - dims.width) * 0.5;
            let y = (screen_height() + dims.height) * 0.5;
            draw_text(&text, x, y, FONT_SIZE as f32, WHITE);
        }

        if !modifiers.is_empty() {
            let qualifier_text = modifiers.join(" + ");
            let dims = measure_text(&qualifier_text, None, QUALIFIER_FONT_SIZE, 1.0);
            let x = (screen_width() - dims.width) * 0.5;
            let y = screen_height() * 0.5 - FONT_SIZE as f32;
            draw_text(&qualifier_text, x, y, QUALIFIER_FONT_SIZE as f32, YELLOW);
        }

        next_frame().await;
    }
}
