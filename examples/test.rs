use macroquad::prelude::*;

use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};

#[macroquad::main("UI showcase")]
async fn main() {
    for _ in 0..4 {
        clear_background(WHITE);

        let text = "!2";
        let y2 = 30.0;
        let x_right = screen_width()-120.0;

        draw_text_ex(
            text,
            x_right,
            20.0,
            TextParams {
                color: BLACK,
                font_size: 16,
                ..Default::default()
            }
        );

        root_ui().label(vec2(x_right, y2), text);

        next_frame().await;

        eprintln!("-------------------------------------");
    }
}
