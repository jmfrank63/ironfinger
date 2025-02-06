use raylib::prelude::*;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const BACKGROUND_COLOR: Color = Color::new(18, 18, 18, 255);
// const TEXT_COLOR: Color = Color::new(237, 237, 237, 255);

struct Ball {
    position: Vector2,
    speed: f32,
    radius: f32,
    color: Color,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Ironfinger")
        .vsync()
        .build();

    let mut ball = Ball {
        position: Vector2::new(WIDTH / 2.0, HEIGHT / 2.0),
        speed: 3.0,
        radius: 40.0,
        color: Color::DARKBLUE,
    };
    rl.set_exit_key(None);
    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            ball.position.x += ball.speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            ball.position.x -= ball.speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            ball.position.y -= ball.speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            ball.position.y += ball.speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_R) {
            ball.position = Vector2::new(WIDTH / 2.0, HEIGHT / 2.0);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            println!("ESC key pressed");
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if ball.color == Color::DARKBLUE {
                ball.color = Color::DARKPURPLE;
            } else {
                ball.color = Color::DARKBLUE;
            }
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(BACKGROUND_COLOR);
        // d.draw_text("Hello, world!", 12, 12, 20, TEXT_COLOR);
        d.draw_circle_v(ball.position, ball.radius, ball.color);
    }
}
