use macroquad::prelude::*;
use std::env;
use std::{thread, time};

fn window_config() -> Conf {
    Conf {
        window_title: "Break Out".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let score_text = "Score: 0";
    let mut player = Paddle::new_default();
    let mut ball = Ball::new_default();

    loop {
        clear_background(BLACK);

        // update
        player.update();
        ball.update();

        // check for collision
        let (is_collide, _cx, cy) = collisions(&mut ball, &mut player);
        if is_collide {
            // push ball outside paddle
            ball.y += cy;
            // reverse ball direction
            ball.dy = -ball.dy;
        }

        // space bar spawns a new ball to make debugging easier
        if is_key_pressed(KeyCode::Space) {
            ball = Ball::new_default();
        }

        // draw
        player.draw();
        ball.draw();
        draw_ui(score_text);

        // quit
        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;

        // wait? native is faster than wasm, wasm can't sleep
        if is_native() && get_fps() > 60 {
            thread::sleep(time::Duration::from_millis(1));
        }
    }
}

struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}

impl Paddle {
    fn new_default() -> Paddle {
        let w = 200_f32;
        Paddle {
            x: screen_width() / 2.0 - w / 2.0,
            y: 400f32,
            width: w,
            height: 50f32,
            color: BLUE,
        }
    }

    fn update(&mut self) {
        let dx = 2f32;

        // move left and right
        if is_key_down(KeyCode::Left) {
            self.x -= dx;
        }
        if is_key_down(KeyCode::Right) {
            self.x += dx;
        }

        // clamp on screen
        if self.x < 0f32 {
            self.x = 0f32;
        }
        if self.x + self.width > screen_width() {
            self.x = screen_width() - self.width;
        }
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, self.color);
    }
}

struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    w: f32,
    h: f32,
    color: Color,
}

impl Ball {
    fn new_default() -> Ball {
        let rx = rand::gen_range(-1f32, 1f32) * 2.0;
        let ry = rand::gen_range(-1f32, 1f32) * 2.0;

        Ball {
            x: 200_f32,
            y: 200_f32,
            dx: rx,
            dy: ry,
            w: 32_f32,
            h: 32_f32,
            color: RED,
        }
    }

    fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        if self.x > 600f32 || self.x < 0f32 {
            self.x = screen_width() / 2.0;
            self.y = screen_height() / 2.0;
            self.dx = rand::gen_range(-1f32, 1f32);
            self.dy = rand::gen_range(-1f32, 1f32);
        }
        if self.y > 600f32 || self.y < 0f32 {
            self.x = screen_width() / 2.0;
            self.y = screen_height() / 2.0;
            self.dx = rand::gen_range(-1f32, 1f32);
            self.dy = rand::gen_range(-1f32, 1f32);
        }
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }
}

// is the os native or wasm?
fn is_native() -> bool {
    let os = env::consts::OS.to_lowercase();
    if os == "windows" || os == "macos" || os == "linux" {
        return true;
    }
    false
}

fn draw_ui(score_text: &str) {
    let sw = screen_width();

    // draw the score, centered
    let text_dim = measure_text(score_text, None, 32, 1.0);
    let text_w = text_dim.width;
    let text_x = sw / 2.0 - text_w / 2.0;
    draw_text(score_text, text_x, 30.0, 32.0, WHITE);
}

fn collisions(ball: &mut Ball, paddle: &mut Paddle) -> (bool, f32, f32) {
    // https://math.stackexchange.com/questions/99565/simplest-way-to-calculate-the-intersect-area-of-two-rectangles

    // x_overlap and y_overlap are the width and height of the overlap rectangle. postive
    let mut x_overlap = (ball.x + ball.w).min(paddle.x + paddle.width) - (ball.x).max(paddle.x);
    let mut y_overlap = (ball.y + ball.h).min(paddle.y + paddle.height) - (ball.y).max(paddle.y);
    if x_overlap > 0f32 && y_overlap > 0f32 {
        if ball.x < paddle.x {
            x_overlap = -x_overlap
        }
        if ball.y < paddle.y {
            y_overlap = -y_overlap
        }
        return (true, x_overlap, y_overlap);
    }

    (false, 0f32, 0f32)
}
