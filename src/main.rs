use macroquad::prelude::*;
use std::env;
use std::{thread, time};
// use rand::Rng;
// use rand::Rng;
// use rand::Rng;

fn window_config() -> Conf {
    Conf {
        window_title: "Break Out".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    // set up
    let mut score: f32 = 0f32;
    let mut score_text = "Score: 0".to_string();
    let mut player = Paddle::new_default();
    let mut ball = Ball::new_default();
    let mut block_count = 16;

    // let event_queue: Vec<Event> = vec![Event::Start];

    // let mut block_list: Vec<Block> = vec![];
    let mut block_list = blocks_create(4, 4, block_count, 32f32, 64f32);

    loop {
        clear_background(BLACK);

        let delta_t = get_frame_time();

        // update
        player.update(delta_t);
        score = ball.update(delta_t, score);

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
        let mut die_list = vec![];
        // let mut i=0;
        for (i, block) in block_list.iter_mut().enumerate() {
            // }.into_iter().enumerate() {
            // loop{
            // let &mut block = block_list[i];
            let is_alive = block.update(&mut ball);
            if !is_alive {
                die_list.push(i);
                // i+=1;
                continue;
            }

            block.draw();
        }
        // remove dead blocks
        for i in die_list.into_iter().rev() {
            score += 1f32;
            block_list.swap_remove(i);
        }

        // all blocks gone?
        block_list = if block_list.len() == 0 {
            block_count += 2;
            blocks_create(4, 4, block_count, 32f32, 64f32)
        } else {
            block_list
        };

        // draw ui
        score_text = draw_ui(score, score_text);

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
        let default_width = 150_f32;
        let default_height = 25_f32;

        Paddle {
            x: screen_width() / 2.0 - default_width / 2.0,
            y: screen_height() - (default_height * 3.0),
            width: default_width,
            height: default_height,
            color: BLUE,
        }
    }

    fn update(&mut self, _delta_t: f32) {
        // mPos.y += velocityScale * mVelocity * deltaTime
        let dx = 4f32 * 60f32 * _delta_t;

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
        let delta = 4f32;
        let choices = vec![delta, -delta];
        let rand_x = *rand::ChooseRandom::choose(&choices).unwrap();
        let rand_y = *rand::ChooseRandom::choose(&choices).unwrap();

        let x_mid = screen_width() / 2f32;
        let x_offset = rand::gen_range(-1f32, 1f32) * 64.0;
        let y_mid = screen_height() / 2f32;
        let y_offset = rand::gen_range(-1f32, 1f32) * 64.0;
        Ball {
            x: x_mid + x_offset,
            y: y_mid + y_offset,
            dx: rand_x,
            dy: rand_y,
            w: 32_f32,
            h: 32_f32,
            color: GREEN,
        }
    }
    fn reset(&mut self) {
        let delta = 4f32;
        let choices = vec![delta, -delta];
        let rand_x = *rand::ChooseRandom::choose(&choices).unwrap();
        let rand_y = *rand::ChooseRandom::choose(&choices).unwrap();

        self.x = screen_width() / 2f32;
        self.y = screen_height() / 2f32;
        self.dx = rand_x;
        self.dy = rand_y;
    }

    fn update(&mut self, _delta_t: f32, score: f32) -> f32 {
        let mut score_new = score;

        let sw = screen_width();
        let sh = screen_height();

        if self.x + self.w >= sw || self.x <= 0f32 {
            // flip direction
            self.dx *= -1f32;
            // stop buzzing on edges
            if self.x < 0f32 {
                self.x = 0f32;
            } else {
                self.x = sw - self.w;
            }

            // small penalty for hitting the edge
            score_new -= 0.01f32;
        }
        if self.y < 0f32 {
            // top, good
            score_new += 0.1f32;
            // flip direction
            self.dy *= -1f32;
            // stop buzzing on edges
            self.y = 0f32;
        }
        if self.y > sh {
            // bottom, bad
            score_new -= 1f32;

            self.reset();
        }

        self.x += self.dx * 60f32 * _delta_t;
        self.y += self.dy * 60f32 * _delta_t;

        // round to nearest n-th
        let fraction = 10;
        let score_int = (score_new * fraction as f32) as i32;
        score_new = score_int as f32 / fraction as f32;

        score_new
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

fn draw_ui(score: f32, score_text_in: String) -> String {
    // TODO how to optimize this?
    let mut score_text = score_text_in;
    if score_text != format!("Score: {}", score) {
        score_text = format!("Score: {}", score);
    }
    let score_ref = score_text.as_str();

    let sw = screen_width();

    // draw the score, centered
    let text_dim = measure_text(score_ref, None, 32, 1.0);
    let text_w = text_dim.width;
    let text_x = sw / 2.0 - text_w / 2.0;
    draw_text(score_ref, text_x, 30.0, 32.0, WHITE);

    score_text
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

fn collisions_block(ball: &mut Ball, paddle: &mut Block) -> (bool, f32, f32) {
    // https://math.stackexchange.com/questions/99565/simplest-way-to-calculate-the-intersect-area-of-two-rectangles

    // x_overlap and y_overlap are the width and height of the overlap rectangle. postive
    let mut x_overlap = (ball.x + ball.w).min(paddle.x + paddle.w) - (ball.x).max(paddle.x);
    let mut y_overlap = (ball.y + ball.h).min(paddle.y + paddle.h) - (ball.y).max(paddle.y);
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

#[derive(Copy, Clone)]
struct Block {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    count: i8,
    color: Color,
}

impl Block {
    fn new(x: f32, y: f32, w: f32, h: f32, count: i8, color: Color) -> Block {
        Block {
            x,
            y,
            w,
            h,
            count,
            color,
        }
    }

    fn update(&mut self, ball: &mut Ball) -> bool {
        // return is_alive
        let (collided, _x_overlap, _y_overlap) = collisions_block(ball, self);
        if collided {
            self.count -= 1;
            if self.count <= 0 {
                return false;
            }
        }
        true
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.color);
    }
}

fn blocks_create(
    cols: i32,
    rows: i32,
    count: i32,
    block_height: f32,
    block_width: f32,
) -> Vec<Block> {
    let mut blocks = Vec::new();

    let color = RED;
    let mut x = 0f32;
    let mut y = 0f32;
    let w = screen_width() / cols as f32;
    let h = screen_height() / 2f32 / rows as f32;
    let mut count_create = 0;

    for _col in 0..rows {
        for _row in 0..cols {
            let bx = x + w / 2.0 - block_width as f32 / 2.0;
            let by = y + h / 2.0 - block_height as f32 / 2.0;
            blocks.push(Block::new(bx, by, block_width, block_height, 1, color));
            count_create += 1;
            if count_create >= count {
                return blocks;
            }
            y += h;
        }
        x += w;
        y = 0f32;
    }
    blocks
}
