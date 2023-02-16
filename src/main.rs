use macroquad::prelude::*;

fn window_config() -> Conf {
    Conf {
        window_title: "Break Out".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut player = Player::new_default();
    let mut ball = Ball::new_default();

    loop {
        clear_background(BLACK);

        let sh = screen_height();
        let sw = screen_width();

        let score_text = "Score: 0";
        let text_dim = measure_text(score_text, None, 32, 1.0);
        let text_w = text_dim.width;
        let text_x = sw / 2.0 - text_w / 2.0;

        draw_text(score_text, text_x, 30.0, 32.0, WHITE);

        player.update();
        player.draw();

        ball.update();
        ball.draw();

        // quit
        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}

struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}

impl Player {
    fn new_default() -> Player {
        Player {
            x: 200f32,
            y: 400f32,
            width: 100f32,
            height: 50f32,
            color: RED,
        }
    }

    fn update(&mut self) {
        // move left and right
        let dx = 0.5f32;
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
        Ball {
            x: 200_f32,
            y: 200_f32,
            dx: 0.2_f32,
            dy: 0.2_f32,
            w: 32_f32,
            h: 32_f32,
            color: WHITE,
        }
    }

    fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        if self.x > 600f32 || self.x < 0f32 {
            self.x = 300f32;
        }
        if self.y > 600f32 || self.y < 0f32 {
            self.y = 300f32;
        }
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }
}
