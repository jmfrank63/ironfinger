use raylib::prelude::*;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
// const BACKGROUND_COLOR: Color = Color::new(18, 18, 18, 255);
// const TEXT_COLOR: Color = Color::new(237, 237, 237, 255);

/// Macro that expands to a static array of all keys you want to check.
macro_rules! all_keys {
    () => {
        [
            KeyboardKey::KEY_APOSTROPHE,
            KeyboardKey::KEY_COMMA,
            KeyboardKey::KEY_MINUS,
            KeyboardKey::KEY_PERIOD,
            KeyboardKey::KEY_SLASH,
            KeyboardKey::KEY_ZERO,
            KeyboardKey::KEY_ONE,
            KeyboardKey::KEY_TWO,
            KeyboardKey::KEY_THREE,
            KeyboardKey::KEY_FOUR,
            KeyboardKey::KEY_FIVE,
            KeyboardKey::KEY_SIX,
            KeyboardKey::KEY_SEVEN,
            KeyboardKey::KEY_EIGHT,
            KeyboardKey::KEY_NINE,
            KeyboardKey::KEY_SEMICOLON,
            KeyboardKey::KEY_EQUAL,
            KeyboardKey::KEY_A,
            KeyboardKey::KEY_B,
            KeyboardKey::KEY_C,
            KeyboardKey::KEY_D,
            KeyboardKey::KEY_E,
            KeyboardKey::KEY_F,
            KeyboardKey::KEY_G,
            KeyboardKey::KEY_H,
            KeyboardKey::KEY_I,
            KeyboardKey::KEY_J,
            KeyboardKey::KEY_K,
            KeyboardKey::KEY_L,
            KeyboardKey::KEY_M,
            KeyboardKey::KEY_N,
            KeyboardKey::KEY_O,
            KeyboardKey::KEY_P,
            KeyboardKey::KEY_Q,
            KeyboardKey::KEY_R,
            KeyboardKey::KEY_S,
            KeyboardKey::KEY_T,
            KeyboardKey::KEY_U,
            KeyboardKey::KEY_V,
            KeyboardKey::KEY_W,
            KeyboardKey::KEY_X,
            KeyboardKey::KEY_Y,
            KeyboardKey::KEY_Z,
            KeyboardKey::KEY_LEFT_BRACKET,
            KeyboardKey::KEY_BACKSLASH,
            KeyboardKey::KEY_RIGHT_BRACKET,
            KeyboardKey::KEY_GRAVE,
            KeyboardKey::KEY_SPACE,
            KeyboardKey::KEY_ESCAPE,
            KeyboardKey::KEY_ENTER,
            KeyboardKey::KEY_TAB,
            KeyboardKey::KEY_BACKSPACE,
            KeyboardKey::KEY_INSERT,
            KeyboardKey::KEY_DELETE,
            KeyboardKey::KEY_RIGHT,
            KeyboardKey::KEY_LEFT,
            KeyboardKey::KEY_DOWN,
            KeyboardKey::KEY_UP,
            KeyboardKey::KEY_PAGE_UP,
            KeyboardKey::KEY_PAGE_DOWN,
            KeyboardKey::KEY_HOME,
            KeyboardKey::KEY_END,
            KeyboardKey::KEY_CAPS_LOCK,
            KeyboardKey::KEY_SCROLL_LOCK,
            KeyboardKey::KEY_NUM_LOCK,
            KeyboardKey::KEY_PRINT_SCREEN,
            KeyboardKey::KEY_PAUSE,
            KeyboardKey::KEY_F1,
            KeyboardKey::KEY_F2,
            KeyboardKey::KEY_F3,
            KeyboardKey::KEY_F4,
            KeyboardKey::KEY_F5,
            KeyboardKey::KEY_F6,
            KeyboardKey::KEY_F7,
            KeyboardKey::KEY_F8,
            KeyboardKey::KEY_F9,
            KeyboardKey::KEY_F10,
            KeyboardKey::KEY_F11,
            KeyboardKey::KEY_F12,
            KeyboardKey::KEY_LEFT_SHIFT,
            KeyboardKey::KEY_LEFT_CONTROL,
            KeyboardKey::KEY_LEFT_ALT,
            KeyboardKey::KEY_LEFT_SUPER,
            KeyboardKey::KEY_RIGHT_SHIFT,
            KeyboardKey::KEY_RIGHT_CONTROL,
            KeyboardKey::KEY_RIGHT_ALT,
            KeyboardKey::KEY_RIGHT_SUPER,
            KeyboardKey::KEY_KB_MENU,
        ]
    };
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Ironfinger")
        .vsync()
        .build();

    rl.set_exit_key(None);
    let mut text = String::new();

    while !rl.window_should_close() {
        // Collect a list of all keys that are currently pressed.
        let mut pressed_keys = Vec::new();
        for key in all_keys!().iter() {
            if rl.is_key_down(*key) {
                pressed_keys.push(format!("\n{:?}", key));
                if let Some(c) = rl.get_char_pressed() {
                    text.push(c);
                }
            }
            // if rl.is_key_up(*key) {
            //     let last = text.pop();
            //     if let Some(last) = last {
            //         text.clear();
            //         text.push(last);
            //     }
            // }
        }

        // Build the text to display.
        let display_text = if pressed_keys.is_empty() {
            "Press any key...".to_owned()
        } else {
            format!("Keys currently pressed:\n{}\n\n{}", pressed_keys.join(", "), text)
        };

        // The drawing block using RAII; when `d` goes out of scope EndDrawing() is called.
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(18, 18, 18, 255));
        d.draw_text(&display_text, 20, 20, 20, Color::DARKORCHID );
    }
}
