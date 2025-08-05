use std::{cell::RefCell, rc::Rc};

use mlua::UserData;
use raylib::RaylibHandle;

/// A macro to wrap around the raylib input methods.
macro_rules! add_input_method {
    ($methods_userdata:ident, $method_name:expr, $input_func:ident) => {
        $methods_userdata.add_method($method_name, |_, this, value: String| {
            let rl = this.0.borrow();
            let Some(key) = raylib::ffi::KeyboardKey::from_lua_key(&value) else {
                log::error!("Invalid key: {}", value);
                return Ok(false);
            };
            Ok(rl.$input_func(key))
        });
    };
}

/// The input api for the lua runtime.
#[derive(Debug, Clone)]
pub struct LuaAPIInput(pub Rc<RefCell<RaylibHandle>>);

impl UserData for LuaAPIInput {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        add_input_method!(methods, "key_down", is_key_down);
        add_input_method!(methods, "key_up", is_key_up);
        add_input_method!(methods, "key_pressed", is_key_pressed);
        add_input_method!(methods, "key_released", is_key_released);

        methods.add_method(
            "get_axis",
            |_, this, (negative, positive): (String, String)| {
                let rl = this.0.borrow();
                let Some(positive_key) = raylib::ffi::KeyboardKey::from_lua_key(&positive) else {
                    log::error!("Invalid key: {}", positive);
                    return Ok(0f32);
                };
                let Some(negative_key) = raylib::ffi::KeyboardKey::from_lua_key(&negative) else {
                    log::error!("Invalid key: {}", negative);
                    return Ok(0f32);
                };

                let positive_down = rl.is_key_down(positive_key) as u8 as f32;
                let negative_down = rl.is_key_down(negative_key) as u8 as f32;

                Ok((-negative_down) + positive_down)
            },
        );
    }
}

/// Handles conversion from the lua string representation of a key.
pub trait FromLuaStringKey {
    /// Create instance from a lua string key.
    /// ### Returns
    /// The appropiate key from a lua key string.
    /// Returns None if the key string is not valid.
    fn from_lua_key(lua_str: &str) -> Option<Self>
    where
        Self: Sized;
}

impl FromLuaStringKey for raylib::ffi::KeyboardKey {
    fn from_lua_key(lua_str: &str) -> Option<Self> {
        Some(match lua_str.to_lowercase().as_str() {
            "apostrophe" => Self::KEY_APOSTROPHE,
            "comma" => Self::KEY_COMMA,
            "minus" => Self::KEY_MINUS,
            "period" => Self::KEY_PERIOD,
            "slash" => Self::KEY_SLASH,
            "zero" => Self::KEY_ZERO,
            "one" => Self::KEY_ONE,
            "two" => Self::KEY_TWO,
            "three" => Self::KEY_THREE,
            "four" => Self::KEY_FOUR,
            "five" => Self::KEY_FIVE,
            "six" => Self::KEY_SIX,
            "seven" => Self::KEY_SEVEN,
            "eight" => Self::KEY_EIGHT,
            "nine" => Self::KEY_NINE,
            "semicolon" => Self::KEY_SEMICOLON,
            "equal" => Self::KEY_EQUAL,
            "a" => Self::KEY_A,
            "b" => Self::KEY_B,
            "c" => Self::KEY_C,
            "d" => Self::KEY_D,
            "e" => Self::KEY_E,
            "f" => Self::KEY_F,
            "g" => Self::KEY_G,
            "h" => Self::KEY_H,
            "i" => Self::KEY_I,
            "j" => Self::KEY_J,
            "k" => Self::KEY_K,
            "l" => Self::KEY_L,
            "m" => Self::KEY_M,
            "n" => Self::KEY_N,
            "o" => Self::KEY_O,
            "p" => Self::KEY_P,
            "q" => Self::KEY_Q,
            "r" => Self::KEY_R,
            "s" => Self::KEY_S,
            "t" => Self::KEY_T,
            "u" => Self::KEY_U,
            "v" => Self::KEY_V,
            "w" => Self::KEY_W,
            "x" => Self::KEY_X,
            "y" => Self::KEY_Y,
            "z" => Self::KEY_Z,
            "left_bracket" => Self::KEY_LEFT_BRACKET,
            "backslash" => Self::KEY_BACKSLASH,
            "right_bracket" => Self::KEY_RIGHT_BRACKET,
            "grave" => Self::KEY_GRAVE,
            "space" => Self::KEY_SPACE,
            "escape" => Self::KEY_ESCAPE,
            "enter" => Self::KEY_ENTER,
            "tab" => Self::KEY_TAB,
            "backspace" => Self::KEY_BACKSPACE,
            "insert" => Self::KEY_INSERT,
            "delete" => Self::KEY_DELETE,
            "right" => Self::KEY_RIGHT,
            "left" => Self::KEY_LEFT,
            "down" => Self::KEY_DOWN,
            "up" => Self::KEY_UP,
            "page_up" => Self::KEY_PAGE_UP,
            "page_down" => Self::KEY_PAGE_DOWN,
            "home" => Self::KEY_HOME,
            "end" => Self::KEY_END,
            "caps_lock" => Self::KEY_CAPS_LOCK,
            "scroll_lock" => Self::KEY_SCROLL_LOCK,
            "num_lock" => Self::KEY_NUM_LOCK,
            "print_screen" => Self::KEY_PRINT_SCREEN,
            "pause" => Self::KEY_PAUSE,
            "f1" => Self::KEY_F1,
            "f2" => Self::KEY_F2,
            "f3" => Self::KEY_F3,
            "f4" => Self::KEY_F4,
            "f5" => Self::KEY_F5,
            "f6" => Self::KEY_F6,
            "f7" => Self::KEY_F7,
            "f8" => Self::KEY_F8,
            "f9" => Self::KEY_F9,
            "f10" => Self::KEY_F10,
            "f11" => Self::KEY_F11,
            "f12" => Self::KEY_F12,
            "left_shift" => Self::KEY_LEFT_SHIFT,
            "left_control" => Self::KEY_LEFT_CONTROL,
            "left_alt" => Self::KEY_LEFT_ALT,
            "left_super" => Self::KEY_LEFT_SUPER,
            "right_shift" => Self::KEY_RIGHT_SHIFT,
            "right_control" => Self::KEY_RIGHT_CONTROL,
            "right_alt" => Self::KEY_RIGHT_ALT,
            "right_super" => Self::KEY_RIGHT_SUPER,
            "menu" => Self::KEY_KB_MENU,
            "keypad_0" => Self::KEY_KP_0,
            "keypad_1" => Self::KEY_KP_1,
            "keypad_2" => Self::KEY_KP_2,
            "keypad_3" => Self::KEY_KP_3,
            "keypad_4" => Self::KEY_KP_4,
            "keypad_5" => Self::KEY_KP_5,
            "keypad_6" => Self::KEY_KP_6,
            "keypad_7" => Self::KEY_KP_7,
            "keypad_8" => Self::KEY_KP_8,
            "keypad_9" => Self::KEY_KP_9,
            "keypad_decimal" => Self::KEY_KP_DECIMAL,
            "keypad_divide" => Self::KEY_KP_DIVIDE,
            "keypad_multiply" => Self::KEY_KP_MULTIPLY,
            "keypad_subtract" => Self::KEY_KP_SUBTRACT,
            "keypad_add" => Self::KEY_KP_ADD,
            "keypad_enter" => Self::KEY_KP_ENTER,
            "keypad_equal" => Self::KEY_KP_EQUAL,
            "back" => Self::KEY_BACK,
            "android_menu" => Self::KEY_MENU,
            "volume_up" => Self::KEY_VOLUME_UP,
            "volume_down" => Self::KEY_VOLUME_DOWN,
            _ => return None,
        })
    }
}
