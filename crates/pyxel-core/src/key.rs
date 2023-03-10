use sdl2::keyboard::Keycode;

use crate::types::Key;

// Keyboard (based on rust-sdl2/src/sdl2/keyboard/keycode.rs)
pub const KEY_BACKSPACE: Key = Keycode::Backspace as Key;
pub const KEY_TAB: Key = Keycode::Tab as Key;
pub const KEY_RETURN: Key = Keycode::Return as Key;
pub const KEY_ESCAPE: Key = Keycode::Escape as Key;
pub const KEY_SPACE: Key = Keycode::Space as Key;
pub const KEY_EXCLAIM: Key = Keycode::Exclaim as Key;
pub const KEY_QUOTEDBL: Key = Keycode::Quotedbl as Key;
pub const KEY_HASH: Key = Keycode::Hash as Key;
pub const KEY_DOLLAR: Key = Keycode::Dollar as Key;
pub const KEY_PERCENT: Key = Keycode::Percent as Key;
pub const KEY_AMPERSAND: Key = Keycode::Ampersand as Key;
pub const KEY_QUOTE: Key = Keycode::Quote as Key;
pub const KEY_LEFTPAREN: Key = Keycode::LeftParen as Key;
pub const KEY_RIGHTPAREN: Key = Keycode::RightParen as Key;
pub const KEY_ASTERISK: Key = Keycode::Asterisk as Key;
pub const KEY_PLUS: Key = Keycode::Plus as Key;
pub const KEY_COMMA: Key = Keycode::Comma as Key;
pub const KEY_MINUS: Key = Keycode::Minus as Key;
pub const KEY_PERIOD: Key = Keycode::Period as Key;
pub const KEY_SLASH: Key = Keycode::Slash as Key;
pub const KEY_0: Key = Keycode::Num0 as Key;
pub const KEY_1: Key = Keycode::Num1 as Key;
pub const KEY_2: Key = Keycode::Num2 as Key;
pub const KEY_3: Key = Keycode::Num3 as Key;
pub const KEY_4: Key = Keycode::Num4 as Key;
pub const KEY_5: Key = Keycode::Num5 as Key;
pub const KEY_6: Key = Keycode::Num6 as Key;
pub const KEY_7: Key = Keycode::Num7 as Key;
pub const KEY_8: Key = Keycode::Num8 as Key;
pub const KEY_9: Key = Keycode::Num9 as Key;
pub const KEY_COLON: Key = Keycode::Colon as Key;
pub const KEY_SEMICOLON: Key = Keycode::Semicolon as Key;
pub const KEY_LESS: Key = Keycode::Less as Key;
pub const KEY_EQUALS: Key = Keycode::Equals as Key;
pub const KEY_GREATER: Key = Keycode::Greater as Key;
pub const KEY_QUESTION: Key = Keycode::Question as Key;
pub const KEY_AT: Key = Keycode::At as Key;
pub const KEY_LEFTBRACKET: Key = Keycode::LeftBracket as Key;
pub const KEY_BACKSLASH: Key = Keycode::Backslash as Key;
pub const KEY_RIGHTBRACKET: Key = Keycode::RightBracket as Key;
pub const KEY_CARET: Key = Keycode::Caret as Key;
pub const KEY_UNDERSCORE: Key = Keycode::Underscore as Key;
pub const KEY_BACKQUOTE: Key = Keycode::Backquote as Key;
pub const KEY_A: Key = Keycode::A as Key;
pub const KEY_B: Key = Keycode::B as Key;
pub const KEY_C: Key = Keycode::C as Key;
pub const KEY_D: Key = Keycode::D as Key;
pub const KEY_E: Key = Keycode::E as Key;
pub const KEY_F: Key = Keycode::F as Key;
pub const KEY_G: Key = Keycode::G as Key;
pub const KEY_H: Key = Keycode::H as Key;
pub const KEY_I: Key = Keycode::I as Key;
pub const KEY_J: Key = Keycode::J as Key;
pub const KEY_K: Key = Keycode::K as Key;
pub const KEY_L: Key = Keycode::L as Key;
pub const KEY_M: Key = Keycode::M as Key;
pub const KEY_N: Key = Keycode::N as Key;
pub const KEY_O: Key = Keycode::O as Key;
pub const KEY_P: Key = Keycode::P as Key;
pub const KEY_Q: Key = Keycode::Q as Key;
pub const KEY_R: Key = Keycode::R as Key;
pub const KEY_S: Key = Keycode::S as Key;
pub const KEY_T: Key = Keycode::T as Key;
pub const KEY_U: Key = Keycode::U as Key;
pub const KEY_V: Key = Keycode::V as Key;
pub const KEY_W: Key = Keycode::W as Key;
pub const KEY_X: Key = Keycode::X as Key;
pub const KEY_Y: Key = Keycode::Y as Key;
pub const KEY_Z: Key = Keycode::Z as Key;
pub const KEY_DELETE: Key = Keycode::Delete as Key;
pub const KEY_CAPSLOCK: Key = Keycode::CapsLock as Key;
pub const KEY_F1: Key = Keycode::F1 as Key;
pub const KEY_F2: Key = Keycode::F2 as Key;
pub const KEY_F3: Key = Keycode::F3 as Key;
pub const KEY_F4: Key = Keycode::F4 as Key;
pub const KEY_F5: Key = Keycode::F5 as Key;
pub const KEY_F6: Key = Keycode::F6 as Key;
pub const KEY_F7: Key = Keycode::F7 as Key;
pub const KEY_F8: Key = Keycode::F8 as Key;
pub const KEY_F9: Key = Keycode::F9 as Key;
pub const KEY_F10: Key = Keycode::F10 as Key;
pub const KEY_F11: Key = Keycode::F11 as Key;
pub const KEY_F12: Key = Keycode::F12 as Key;
pub const KEY_PRINTSCREEN: Key = Keycode::PrintScreen as Key;
pub const KEY_SCROLLLOCK: Key = Keycode::ScrollLock as Key;
pub const KEY_PAUSE: Key = Keycode::Pause as Key;
pub const KEY_INSERT: Key = Keycode::Insert as Key;
pub const KEY_HOME: Key = Keycode::Home as Key;
pub const KEY_PAGEUP: Key = Keycode::PageUp as Key;
pub const KEY_END: Key = Keycode::End as Key;
pub const KEY_PAGEDOWN: Key = Keycode::PageDown as Key;
pub const KEY_RIGHT: Key = Keycode::Right as Key;
pub const KEY_LEFT: Key = Keycode::Left as Key;
pub const KEY_DOWN: Key = Keycode::Down as Key;
pub const KEY_UP: Key = Keycode::Up as Key;
pub const KEY_NUMLOCKCLEAR: Key = Keycode::NumLockClear as Key;
pub const KEY_KP_DIVIDE: Key = Keycode::KpDivide as Key;
pub const KEY_KP_MULTIPLY: Key = Keycode::KpMultiply as Key;
pub const KEY_KP_MINUS: Key = Keycode::KpMinus as Key;
pub const KEY_KP_PLUS: Key = Keycode::KpPlus as Key;
pub const KEY_KP_ENTER: Key = Keycode::KpEnter as Key;
pub const KEY_KP_1: Key = Keycode::Kp1 as Key;
pub const KEY_KP_2: Key = Keycode::Kp2 as Key;
pub const KEY_KP_3: Key = Keycode::Kp3 as Key;
pub const KEY_KP_4: Key = Keycode::Kp4 as Key;
pub const KEY_KP_5: Key = Keycode::Kp5 as Key;
pub const KEY_KP_6: Key = Keycode::Kp6 as Key;
pub const KEY_KP_7: Key = Keycode::Kp7 as Key;
pub const KEY_KP_8: Key = Keycode::Kp8 as Key;
pub const KEY_KP_9: Key = Keycode::Kp9 as Key;
pub const KEY_KP_0: Key = Keycode::Kp0 as Key;
pub const KEY_KP_PERIOD: Key = Keycode::KpPeriod as Key;
pub const KEY_APPLICATION: Key = Keycode::Application as Key;
pub const KEY_POWER: Key = Keycode::Power as Key;
pub const KEY_KP_EQUALS: Key = Keycode::KpEquals as Key;
pub const KEY_F13: Key = Keycode::F13 as Key;
pub const KEY_F14: Key = Keycode::F14 as Key;
pub const KEY_F15: Key = Keycode::F15 as Key;
pub const KEY_F16: Key = Keycode::F16 as Key;
pub const KEY_F17: Key = Keycode::F17 as Key;
pub const KEY_F18: Key = Keycode::F18 as Key;
pub const KEY_F19: Key = Keycode::F19 as Key;
pub const KEY_F20: Key = Keycode::F20 as Key;
pub const KEY_F21: Key = Keycode::F21 as Key;
pub const KEY_F22: Key = Keycode::F22 as Key;
pub const KEY_F23: Key = Keycode::F23 as Key;
pub const KEY_F24: Key = Keycode::F24 as Key;
pub const KEY_EXECUTE: Key = Keycode::Execute as Key;
pub const KEY_HELP: Key = Keycode::Help as Key;
pub const KEY_MENU: Key = Keycode::Menu as Key;
pub const KEY_SELECT: Key = Keycode::Select as Key;
pub const KEY_STOP: Key = Keycode::Stop as Key;
pub const KEY_AGAIN: Key = Keycode::Again as Key;
pub const KEY_UNDO: Key = Keycode::Undo as Key;
pub const KEY_CUT: Key = Keycode::Cut as Key;
pub const KEY_COPY: Key = Keycode::Copy as Key;
pub const KEY_PASTE: Key = Keycode::Paste as Key;
pub const KEY_FIND: Key = Keycode::Find as Key;
pub const KEY_MUTE: Key = Keycode::Mute as Key;
pub const KEY_VOLUMEUP: Key = Keycode::VolumeUp as Key;
pub const KEY_VOLUMEDOWN: Key = Keycode::VolumeDown as Key;
pub const KEY_KP_COMMA: Key = Keycode::KpComma as Key;
pub const KEY_KP_EQUALSAS400: Key = Keycode::KpEqualsAS400 as Key;
pub const KEY_ALTERASE: Key = Keycode::AltErase as Key;
pub const KEY_SYSREQ: Key = Keycode::Sysreq as Key;
pub const KEY_CANCEL: Key = Keycode::Cancel as Key;
pub const KEY_CLEAR: Key = Keycode::Clear as Key;
pub const KEY_PRIOR: Key = Keycode::Prior as Key;
pub const KEY_RETURN2: Key = Keycode::Return2 as Key;
pub const KEY_SEPARATOR: Key = Keycode::Separator as Key;
pub const KEY_OUT: Key = Keycode::Out as Key;
pub const KEY_OPER: Key = Keycode::Oper as Key;
pub const KEY_CLEARAGAIN: Key = Keycode::ClearAgain as Key;
pub const KEY_CRSEL: Key = Keycode::CrSel as Key;
pub const KEY_EXSEL: Key = Keycode::ExSel as Key;
pub const KEY_KP_00: Key = Keycode::Kp00 as Key;
pub const KEY_KP_000: Key = Keycode::Kp000 as Key;
pub const KEY_THOUSANDSSEPARATOR: Key = Keycode::ThousandsSeparator as Key;
pub const KEY_DECIMALSEPARATOR: Key = Keycode::DecimalSeparator as Key;
pub const KEY_CURRENCYUNIT: Key = Keycode::CurrencyUnit as Key;
pub const KEY_CURRENCYSUBUNIT: Key = Keycode::CurrencySubUnit as Key;
pub const KEY_KP_LEFTPAREN: Key = Keycode::KpLeftParen as Key;
pub const KEY_KP_RIGHTPAREN: Key = Keycode::KpRightParen as Key;
pub const KEY_KP_LEFTBRACE: Key = Keycode::KpLeftBrace as Key;
pub const KEY_KP_RIGHTBRACE: Key = Keycode::KpRightBrace as Key;
pub const KEY_KP_TAB: Key = Keycode::KpTab as Key;
pub const KEY_KP_BACKSPACE: Key = Keycode::KpBackspace as Key;
pub const KEY_KP_A: Key = Keycode::KpA as Key;
pub const KEY_KP_B: Key = Keycode::KpB as Key;
pub const KEY_KP_C: Key = Keycode::KpC as Key;
pub const KEY_KP_D: Key = Keycode::KpD as Key;
pub const KEY_KP_E: Key = Keycode::KpE as Key;
pub const KEY_KP_F: Key = Keycode::KpF as Key;
pub const KEY_KP_XOR: Key = Keycode::KpXor as Key;
pub const KEY_KP_POWER: Key = Keycode::KpPower as Key;
pub const KEY_KP_PERCENT: Key = Keycode::KpPercent as Key;
pub const KEY_KP_LESS: Key = Keycode::KpLess as Key;
pub const KEY_KP_GREATER: Key = Keycode::KpGreater as Key;
pub const KEY_KP_AMPERSAND: Key = Keycode::KpAmpersand as Key;
pub const KEY_KP_DBLAMPERSAND: Key = Keycode::KpDblAmpersand as Key;
pub const KEY_KP_VERTICALBAR: Key = Keycode::KpVerticalBar as Key;
pub const KEY_KP_DBLVERTICALBAR: Key = Keycode::KpDblVerticalBar as Key;
pub const KEY_KP_COLON: Key = Keycode::KpColon as Key;
pub const KEY_KP_HASH: Key = Keycode::KpHash as Key;
pub const KEY_KP_SPACE: Key = Keycode::KpSpace as Key;
pub const KEY_KP_AT: Key = Keycode::KpAt as Key;
pub const KEY_KP_EXCLAM: Key = Keycode::KpExclam as Key;
pub const KEY_KP_MEMSTORE: Key = Keycode::KpMemStore as Key;
pub const KEY_KP_MEMRECALL: Key = Keycode::KpMemRecall as Key;
pub const KEY_KP_MEMCLEAR: Key = Keycode::KpMemClear as Key;
pub const KEY_KP_MEMADD: Key = Keycode::KpMemAdd as Key;
pub const KEY_KP_MEMSUBTRACT: Key = Keycode::KpMemSubtract as Key;
pub const KEY_KP_MEMMULTIPLY: Key = Keycode::KpMemMultiply as Key;
pub const KEY_KP_MEMDIVIDE: Key = Keycode::KpMemDivide as Key;
pub const KEY_KP_PLUSMINUS: Key = Keycode::KpPlusMinus as Key;
pub const KEY_KP_CLEAR: Key = Keycode::KpClear as Key;
pub const KEY_KP_CLEARENTRY: Key = Keycode::KpClearEntry as Key;
pub const KEY_KP_BINARY: Key = Keycode::KpBinary as Key;
pub const KEY_KP_OCTAL: Key = Keycode::KpOctal as Key;
pub const KEY_KP_DECIMAL: Key = Keycode::KpDecimal as Key;
pub const KEY_KP_HEXADECIMAL: Key = Keycode::KpHexadecimal as Key;
pub const KEY_LCTRL: Key = Keycode::LCtrl as Key;
pub const KEY_LSHIFT: Key = Keycode::LShift as Key;
pub const KEY_LALT: Key = Keycode::LAlt as Key;
pub const KEY_LGUI: Key = Keycode::LGui as Key;
pub const KEY_RCTRL: Key = Keycode::RCtrl as Key;
pub const KEY_RSHIFT: Key = Keycode::RShift as Key;
pub const KEY_RALT: Key = Keycode::RAlt as Key;
pub const KEY_RGUI: Key = Keycode::RGui as Key;
pub const KEY_MODE: Key = Keycode::Mode as Key;
pub const KEY_AUDIONEXT: Key = Keycode::AudioNext as Key;
pub const KEY_AUDIOPREV: Key = Keycode::AudioPrev as Key;
pub const KEY_AUDIOSTOP: Key = Keycode::AudioStop as Key;
pub const KEY_AUDIOPLAY: Key = Keycode::AudioPlay as Key;
pub const KEY_AUDIOMUTE: Key = Keycode::AudioMute as Key;
pub const KEY_MEDIASELECT: Key = Keycode::MediaSelect as Key;
pub const KEY_WWW: Key = Keycode::Www as Key;
pub const KEY_MAIL: Key = Keycode::Mail as Key;
pub const KEY_CALCULATOR: Key = Keycode::Calculator as Key;
pub const KEY_COMPUTER: Key = Keycode::Computer as Key;
pub const KEY_AC_SEARCH: Key = Keycode::AcSearch as Key;
pub const KEY_AC_HOME: Key = Keycode::AcHome as Key;
pub const KEY_AC_BACK: Key = Keycode::AcBack as Key;
pub const KEY_AC_FORWARD: Key = Keycode::AcForward as Key;
pub const KEY_AC_STOP: Key = Keycode::AcStop as Key;
pub const KEY_AC_REFRESH: Key = Keycode::AcRefresh as Key;
pub const KEY_AC_BOOKMARKS: Key = Keycode::AcBookmarks as Key;
pub const KEY_BRIGHTNESSDOWN: Key = Keycode::BrightnessDown as Key;
pub const KEY_BRIGHTNESSUP: Key = Keycode::BrightnessUp as Key;
pub const KEY_DISPLAYSWITCH: Key = Keycode::DisplaySwitch as Key;
pub const KEY_KBDILLUMTOGGLE: Key = Keycode::KbdIllumToggle as Key;
pub const KEY_KBDILLUMDOWN: Key = Keycode::KbdIllumDown as Key;
pub const KEY_KBDILLUMUP: Key = Keycode::KbdIllumUp as Key;
pub const KEY_EJECT: Key = Keycode::Eject as Key;
pub const KEY_SLEEP: Key = Keycode::Sleep as Key;
pub const KEY_NONE: Key = 10000;
pub const KEY_SHIFT: Key = 10001;
pub const KEY_CTRL: Key = 10002;
pub const KEY_ALT: Key = 10003;
pub const KEY_GUI: Key = 10004;

// Mouse
pub const MOUSE_POS_X: Key = 20000;
pub const MOUSE_POS_Y: Key = 20001;
pub const MOUSE_WHEEL_X: Key = 20002;
pub const MOUSE_WHEEL_Y: Key = 20003;
pub const MOUSE_BUTTON_LEFT: Key = 20004;
pub const MOUSE_BUTTON_MIDDLE: Key = 20005;
pub const MOUSE_BUTTON_RIGHT: Key = 20006;
pub const MOUSE_BUTTON_X1: Key = 20007;
pub const MOUSE_BUTTON_X2: Key = 20008;
pub const MOUSE_BUTTON_UNKNOWN: Key = 20009;

// Gamepad1
pub const GAMEPAD1_AXIS_LEFTX: Key = 30000;
pub const GAMEPAD1_AXIS_LEFTY: Key = 30001;
pub const GAMEPAD1_AXIS_RIGHTX: Key = 30002;
pub const GAMEPAD1_AXIS_RIGHTY: Key = 30003;
pub const GAMEPAD1_AXIS_TRIGGERLEFT: Key = 30004;
pub const GAMEPAD1_AXIS_TRIGGERRIGHT: Key = 30005;
pub const GAMEPAD1_BUTTON_A: Key = 31000;
pub const GAMEPAD1_BUTTON_B: Key = 31001;
pub const GAMEPAD1_BUTTON_X: Key = 31002;
pub const GAMEPAD1_BUTTON_Y: Key = 31003;
pub const GAMEPAD1_BUTTON_BACK: Key = 31004;
pub const GAMEPAD1_BUTTON_GUIDE: Key = 31005;
pub const GAMEPAD1_BUTTON_START: Key = 31006;
pub const GAMEPAD1_BUTTON_LEFTSTICK: Key = 31007;
pub const GAMEPAD1_BUTTON_RIGHTSTICK: Key = 31008;
pub const GAMEPAD1_BUTTON_LEFTSHOULDER: Key = 31009;
pub const GAMEPAD1_BUTTON_RIGHTSHOULDER: Key = 31010;
pub const GAMEPAD1_BUTTON_DPAD_UP: Key = 31011;
pub const GAMEPAD1_BUTTON_DPAD_DOWN: Key = 31012;
pub const GAMEPAD1_BUTTON_DPAD_LEFT: Key = 31013;
pub const GAMEPAD1_BUTTON_DPAD_RIGHT: Key = 31014;

// Gamepad2
pub const GAMEPAD2_AXIS_LEFTX: Key = 40000;
pub const GAMEPAD2_AXIS_LEFTY: Key = 40001;
pub const GAMEPAD2_AXIS_RIGHTX: Key = 40002;
pub const GAMEPAD2_AXIS_RIGHTY: Key = 40003;
pub const GAMEPAD2_AXIS_TRIGGERLEFT: Key = 40004;
pub const GAMEPAD2_AXIS_TRIGGERRIGHT: Key = 40005;
pub const GAMEPAD2_BUTTON_A: Key = 41000;
pub const GAMEPAD2_BUTTON_B: Key = 41001;
pub const GAMEPAD2_BUTTON_X: Key = 41002;
pub const GAMEPAD2_BUTTON_Y: Key = 41003;
pub const GAMEPAD2_BUTTON_BACK: Key = 41004;
pub const GAMEPAD2_BUTTON_GUIDE: Key = 41005;
pub const GAMEPAD2_BUTTON_START: Key = 41006;
pub const GAMEPAD2_BUTTON_LEFTSTICK: Key = 41007;
pub const GAMEPAD2_BUTTON_RIGHTSTICK: Key = 41008;
pub const GAMEPAD2_BUTTON_LEFTSHOULDER: Key = 41009;
pub const GAMEPAD2_BUTTON_RIGHTSHOULDER: Key = 41010;
pub const GAMEPAD2_BUTTON_DPAD_UP: Key = 41011;
pub const GAMEPAD2_BUTTON_DPAD_DOWN: Key = 41012;
pub const GAMEPAD2_BUTTON_DPAD_LEFT: Key = 41013;
pub const GAMEPAD2_BUTTON_DPAD_RIGHT: Key = 41014;

pub fn is_keyboard_key(key: Key) -> bool {
    !(MOUSE_POS_X..=GAMEPAD2_BUTTON_DPAD_RIGHT).contains(&key)
}

pub const fn to_integrated_key(key: Key) -> Option<Key> {
    match key {
        KEY_LSHIFT | KEY_RSHIFT => Some(KEY_SHIFT),
        KEY_LCTRL | KEY_RCTRL => Some(KEY_CTRL),
        KEY_LALT | KEY_RALT => Some(KEY_ALT),
        KEY_LGUI | KEY_RGUI => Some(KEY_GUI),
        _ => None,
    }
}
