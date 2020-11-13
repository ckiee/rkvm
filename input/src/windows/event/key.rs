use crate::event::Key;
use winapi::um::winuser;

impl Key {
    pub(crate) fn to_raw(&self) -> Option<u16> {
        use Key::*;

        let code = match *self {
            Cancel => winuser::VK_CANCEL,
            Back => winuser::VK_BACK,
            Tab => winuser::VK_TAB,
            Clear => winuser::VK_CLEAR,
            Enter => winuser::VK_RETURN,

            Menu => winuser::VK_MENU,
            Pause => winuser::VK_PAUSE,
            CapsLock => winuser::VK_CAPITAL,
            KatakanaHiragana => winuser::VK_KANA, // TODO: Check for correctness.
            Hangeul => winuser::VK_HANGUL,
            Esc => winuser::VK_ESCAPE,
            Space => winuser::VK_SPACE,
            Pageup => winuser::VK_PRIOR,
            Pagedown => winuser::VK_NEXT,
            End => winuser::VK_END,
            Home => winuser::VK_HOME,
            Left => winuser::VK_LEFT,
            Up => winuser::VK_UP,
            Right => winuser::VK_RIGHT,
            Down => winuser::VK_DOWN,
            Select => winuser::VK_SELECT,
            Print => winuser::VK_PRINT,
            Open => winuser::VK_EXECUTE, // TODO: Check for correctness.
            SysRq => winuser::VK_SNAPSHOT,
            Insert => winuser::VK_INSERT,
            Delete => winuser::VK_DELETE,
            Help => winuser::VK_HELP,
            N0 => 0x30,
            N1 => 0x31,
            N2 => 0x32,
            N3 => 0x33,
            N4 => 0x34,
            N5 => 0x35,
            N6 => 0x36,
            N7 => 0x37,
            N8 => 0x38,
            N9 => 0x39,
            A => 0x41,
            B => 0x42,
            C => 0x43,
            D => 0x44,
            E => 0x45,
            F => 0x46,
            G => 0x47,
            H => 0x48,
            I => 0x49,
            J => 0x4A,
            K => 0x4B,
            L => 0x4C,
            M => 0x4D,
            N => 0x4E,
            O => 0x4F,
            P => 0x50,
            Q => 0x51,
            R => 0x52,
            S => 0x53,
            T => 0x54,
            U => 0x55,
            V => 0x56,
            W => 0x57,
            X => 0x58,
            Y => 0x59,
            Z => 0x5A,
            LeftMeta => winuser::VK_LWIN,
            RightMeta => winuser::VK_RWIN,
            Sleep => winuser::VK_SLEEP,
            Kp0 => winuser::VK_NUMPAD0,
            Kp1 => winuser::VK_NUMPAD1,
            Kp2 => winuser::VK_NUMPAD2,
            Kp3 => winuser::VK_NUMPAD3,
            Kp4 => winuser::VK_NUMPAD4,
            Kp5 => winuser::VK_NUMPAD5,
            Kp6 => winuser::VK_NUMPAD6,
            Kp7 => winuser::VK_NUMPAD7,
            Kp8 => winuser::VK_NUMPAD8,
            Kp9 => winuser::VK_NUMPAD9,
            KpPlus => winuser::VK_ADD,
            KpComma => winuser::VK_SEPARATOR,
            KpMinus => winuser::VK_SUBTRACT,
            KpDott => winuser::VK_DECIMAL,
            KpSlash => winuser::VK_DIVIDE,
            F1 => winuser::VK_F1,
            F2 => winuser::VK_F2,
            F3 => winuser::VK_F3,
            F4 => winuser::VK_F4,
            F5 => winuser::VK_F5,
            F6 => winuser::VK_F6,
            F7 => winuser::VK_F7,
            F8 => winuser::VK_F8,
            F9 => winuser::VK_F9,
            F10 => winuser::VK_F10,
            F11 => winuser::VK_F11,
            F12 => winuser::VK_F12,
            F13 => winuser::VK_F13,
            F14 => winuser::VK_F14,
            F15 => winuser::VK_F15,
            F16 => winuser::VK_F16,
            F17 => winuser::VK_F17,
            F18 => winuser::VK_F18,
            F19 => winuser::VK_F19,
            F20 => winuser::VK_F20,
            F21 => winuser::VK_F21,
            F22 => winuser::VK_F22,
            F23 => winuser::VK_F23,
            F24 => winuser::VK_F24,
            NumLock => winuser::VK_NUMLOCK,
            ScrollLock => winuser::VK_SCROLL,
            LeftShift => winuser::VK_LSHIFT,
            RightShift => winuser::VK_RSHIFT,
            LeftCtrl => winuser::VK_LCONTROL,
            RightCtrl => winuser::VK_RCONTROL,
            LeftAlt => winuser::VK_LMENU,
            RightAlt => winuser::VK_RMENU,
            Previous => winuser::VK_BROWSER_BACK,
            Next => winuser::VK_BROWSER_FORWARD,
            Stop => winuser::VK_BROWSER_STOP,
            Search => winuser::VK_BROWSER_SEARCH,
            Favorites => winuser::VK_BROWSER_FAVORITES,
            Homepage => winuser::VK_BROWSER_HOME,
            Mute => winuser::VK_VOLUME_MUTE,
            VolumeUp => winuser::VK_VOLUME_UP,
            VolumeDown => winuser::VK_VOLUME_DOWN,
            Nextsong => winuser::VK_MEDIA_NEXT_TRACK,
            PreviousSong => winuser::VK_MEDIA_PREV_TRACK,
            PlayPause => winuser::VK_MEDIA_PLAY_PAUSE,
            Email => winuser::VK_LAUNCH_MAIL,
            Play => winuser::VK_PLAY,
            Zoom => winuser::VK_ZOOM,
            _ => return None,
        };

        Some(code as _)
    }
}
