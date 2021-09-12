use macroquad::prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "RTS".to_owned(),
        fullscreen: false,
        window_width: 1024 as i32,
        window_height: 800 as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }

        clear_background(LIGHTGRAY);

        set_camera(&Camera2D {
            zoom: vec2(1. / 400., 1. / 300.),
            target: vec2(400., 300.),
            ..Default::default()
        });
        draw_circle(0., 0., 100., YELLOW);
        draw_line(0.0, 0.0, 780., 580., 10.01, RED);

        next_frame().await
    }
}