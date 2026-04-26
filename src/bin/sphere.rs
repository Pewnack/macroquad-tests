use macroquad::prelude::*;

const ROTATION_SPEED: f32 = 1.5;
const INFO_FONT_SIZE: u16 = 22;

#[macroquad::main("Sphere")]
async fn main() {
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.3;

    loop {
        let dt = get_frame_time();

        if is_key_down(KeyCode::Left) {
            yaw -= ROTATION_SPEED * dt;
        }
        if is_key_down(KeyCode::Right) {
            yaw += ROTATION_SPEED * dt;
        }
        if is_key_down(KeyCode::Up) {
            pitch = (pitch - ROTATION_SPEED * dt).max(-std::f32::consts::FRAC_PI_2 + 0.01);
        }
        if is_key_down(KeyCode::Down) {
            pitch = (pitch + ROTATION_SPEED * dt).min(std::f32::consts::FRAC_PI_2 - 0.01);
        }

        let distance = 4.0_f32;
        let camera_pos = vec3(
            distance * yaw.cos() * pitch.cos(),
            distance * (-pitch).sin(),
            distance * yaw.sin() * pitch.cos(),
        );

        set_camera(&Camera3D {
            position: camera_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        clear_background(BLACK);

        draw_sphere(vec3(0.0, 0.0, 0.0), 1.0, None, BLUE);
        draw_sphere_wires(vec3(0.0, 0.0, 0.0), 1.0, None, WHITE);

        set_default_camera();

        let info = "Arrow keys to rotate";
        let dims = measure_text(info, None, INFO_FONT_SIZE, 1.0);
        draw_text(
            info,
            (screen_width() - dims.width) * 0.5,
            screen_height() - 20.0,
            INFO_FONT_SIZE as f32,
            YELLOW,
        );

        next_frame().await;
    }
}
