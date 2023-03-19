use crate::common::*;
use std::{thread::sleep, time::Duration};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub enum BlockInput {
    Block,
    DontBlock,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
pub enum Keyboard {
    BackspaceKey,
    TabKey,
    EnterKey,
    EscapeKey,
    SpaceKey,
    PageUpKey,
    PageDownKey,
    EndKey,
    HomeKey,
    LeftKey,
    UpKey,
    RightKey,
    DownKey,
    InsertKey,
    DeleteKey,
    Numrow0Key,
    Numrow1Key,
    Numrow2Key,
    Numrow3Key,
    Numrow4Key,
    Numrow5Key,
    Numrow6Key,
    Numrow7Key,
    Numrow8Key,
    Numrow9Key,
    AKey,
    BKey,
    CKey,
    DKey,
    EKey,
    FKey,
    GKey,
    HKey,
    IKey,
    JKey,
    KKey,
    LKey,
    MKey,
    NKey,
    OKey,
    PKey,
    QKey,
    RKey,
    SKey,
    TKey,
    UKey,
    VKey,
    WKey,
    XKey,
    YKey,
    ZKey,
    LSuper,
    RSuper,
    Numpad0Key,
    Numpad1Key,
    Numpad2Key,
    Numpad3Key,
    Numpad4Key,
    Numpad5Key,
    Numpad6Key,
    Numpad7Key,
    Numpad8Key,
    Numpad9Key,
    F1Key,
    F2Key,
    F3Key,
    F4Key,
    F5Key,
    F6Key,
    F7Key,
    F8Key,
    F9Key,
    F10Key,
    F11Key,
    F12Key,
    F13Key,
    F14Key,
    F15Key,
    F16Key,
    F17Key,
    F18Key,
    F19Key,
    F20Key,
    F21Key,
    F22Key,
    F23Key,
    F24Key,
    NumLockKey,
    ScrollLockKey,
    CapsLockKey,
    LShiftKey,
    RShiftKey,
    LControlKey,
    RControlKey,
    LAltKey,
    RAltKey,

    BrowserBackKey,
    BrowserForwardKey,
    BrowserRefreshKey,

    VolumeMuteKey,
    VolumeDownKey,
    VolumeUpKey,

    MediaNextTrackKey,
    MediaPrevTrackKey,
    MediaStopKey,
    MediaPlayPauseKey,

    BackquoteKey,
    SlashKey,
    BackslashKey,
    CommaKey,
    PeriodKey,
    MinusKey,
    QuoteKey,
    SemicolonKey,
    LBracketKey,
    RBracketKey,
    EqualKey,

    #[strum(disabled)]
    OtherKey(u64),
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumIter)]
pub enum MouseButton {
    LeftButton,
    MiddleButton,
    RightButton,
    X1Button,
    X2Button,

    #[strum(disabled)]
    OtherButton(u32),
}

pub struct MouseCursor;

pub struct MouseWheel;

impl Keyboard {
    pub fn bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        KEYBOARD_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::NormalBind(Arc::new(callback)));
    }

    pub fn block_bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        KEYBOARD_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::BlockBind(Arc::new(callback)));
    }

    pub fn blockable_bind<F: Fn() -> BlockInput + Send + Sync + 'static>(self, callback: F) {
        KEYBOARD_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::BlockableBind(Arc::new(callback)));
    }

    pub fn bind_all<F: Fn(Keyboard) + Send + Sync + Clone + 'static>(callback: F) {
        for key in Keyboard::iter() {
            let callback = callback.clone();
            let fire = move || {
                callback(key);
            };

            KEYBOARD_BINDS
                .lock()
                .unwrap()
                .insert(key, Bind::NormalBind(Arc::new(fire)));
        }
    }

    pub fn unbind(self) {
        KEYBOARD_BINDS.lock().unwrap().remove(&self);
    }
}

impl MouseButton {
    pub fn bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::NormalBind(Arc::new(callback)));
    }

    pub fn block_bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::BlockBind(Arc::new(callback)));
    }

    pub fn blockable_bind<F: Fn() -> BlockInput + Send + Sync + 'static>(self, callback: F) {
        MOUSE_BINDS
            .lock()
            .unwrap()
            .insert(self, Bind::BlockableBind(Arc::new(callback)));
    }

    pub fn bind_all<F: Fn(MouseButton) + Send + Sync + Clone + 'static>(callback: F) {
        for btn in MouseButton::iter() {
            let callback = callback.clone();
            let fire = move || {
                callback(btn);
            };

            MOUSE_BINDS
                .lock()
                .unwrap()
                .insert(btn, Bind::NormalBind(Arc::new(fire)));
        }
    }

    pub fn unbind(self) {
        MOUSE_BINDS.lock().unwrap().remove(&self);
    }
}

pub fn from_keybd_key(k: Keyboard) -> Option<char> {
    match k {
        Keyboard::AKey => Some('a'),
        Keyboard::BKey => Some('b'),
        Keyboard::CKey => Some('c'),
        Keyboard::DKey => Some('d'),
        Keyboard::EKey => Some('e'),
        Keyboard::FKey => Some('f'),
        Keyboard::GKey => Some('g'),
        Keyboard::HKey => Some('h'),
        Keyboard::IKey => Some('i'),
        Keyboard::JKey => Some('j'),
        Keyboard::KKey => Some('k'),
        Keyboard::LKey => Some('l'),
        Keyboard::MKey => Some('m'),
        Keyboard::NKey => Some('n'),
        Keyboard::OKey => Some('o'),
        Keyboard::PKey => Some('p'),
        Keyboard::QKey => Some('q'),
        Keyboard::RKey => Some('r'),
        Keyboard::SKey => Some('s'),
        Keyboard::TKey => Some('t'),
        Keyboard::UKey => Some('u'),
        Keyboard::VKey => Some('v'),
        Keyboard::WKey => Some('w'),
        Keyboard::XKey => Some('x'),
        Keyboard::YKey => Some('y'),
        Keyboard::ZKey => Some('z'),
        Keyboard::Numpad0Key => Some('0'),
        Keyboard::Numpad1Key => Some('1'),
        Keyboard::Numpad2Key => Some('2'),
        Keyboard::Numpad3Key => Some('3'),
        Keyboard::Numpad4Key => Some('4'),
        Keyboard::Numpad5Key => Some('5'),
        Keyboard::Numpad6Key => Some('6'),
        Keyboard::Numpad7Key => Some('7'),
        Keyboard::Numpad8Key => Some('8'),
        Keyboard::Numpad9Key => Some('9'),
        Keyboard::Numrow0Key => Some('0'),
        Keyboard::Numrow1Key => Some('1'),
        Keyboard::Numrow2Key => Some('2'),
        Keyboard::Numrow3Key => Some('3'),
        Keyboard::Numrow4Key => Some('4'),
        Keyboard::Numrow5Key => Some('5'),
        Keyboard::Numrow6Key => Some('6'),
        Keyboard::Numrow7Key => Some('7'),
        Keyboard::Numrow8Key => Some('8'),
        Keyboard::Numrow9Key => Some('9'),
        Keyboard::BackslashKey => Some('\\'),
        Keyboard::SlashKey => Some('/'),
        Keyboard::CommaKey => Some(','),
        Keyboard::PeriodKey => Some('.'),
        Keyboard::MinusKey => Some('-'),
        Keyboard::QuoteKey => Some('"'),
        Keyboard::SemicolonKey => Some(';'),
        Keyboard::LBracketKey => Some('['),
        Keyboard::RBracketKey => Some(']'),
        Keyboard::EqualKey => Some('='),
        _ => None,
    }
}

pub fn get_keybd_key(c: char) -> Option<Keyboard> {
    match c {
        ' ' => Some(Keyboard::SpaceKey),
        'A' | 'a' => Some(Keyboard::AKey),
        'B' | 'b' => Some(Keyboard::BKey),
        'C' | 'c' => Some(Keyboard::CKey),
        'D' | 'd' => Some(Keyboard::DKey),
        'E' | 'e' => Some(Keyboard::EKey),
        'F' | 'f' => Some(Keyboard::FKey),
        'G' | 'g' => Some(Keyboard::GKey),
        'H' | 'h' => Some(Keyboard::HKey),
        'I' | 'i' => Some(Keyboard::IKey),
        'J' | 'j' => Some(Keyboard::JKey),
        'K' | 'k' => Some(Keyboard::KKey),
        'L' | 'l' => Some(Keyboard::LKey),
        'M' | 'm' => Some(Keyboard::MKey),
        'N' | 'n' => Some(Keyboard::NKey),
        'O' | 'o' => Some(Keyboard::OKey),
        'P' | 'p' => Some(Keyboard::PKey),
        'Q' | 'q' => Some(Keyboard::QKey),
        'R' | 'r' => Some(Keyboard::RKey),
        'S' | 's' => Some(Keyboard::SKey),
        'T' | 't' => Some(Keyboard::TKey),
        'U' | 'u' => Some(Keyboard::UKey),
        'V' | 'v' => Some(Keyboard::VKey),
        'W' | 'w' => Some(Keyboard::WKey),
        'X' | 'x' => Some(Keyboard::XKey),
        'Y' | 'y' => Some(Keyboard::YKey),
        'Z' | 'z' => Some(Keyboard::ZKey),
        '0' | ')' => Some(Keyboard::Numrow0Key),
        '1' | '!' => Some(Keyboard::Numrow1Key),
        '2' | '@' => Some(Keyboard::Numrow2Key),
        '3' | '#' => Some(Keyboard::Numrow3Key),
        '4' | '$' => Some(Keyboard::Numrow4Key),
        '5' | '%' => Some(Keyboard::Numrow5Key),
        '6' | '^' => Some(Keyboard::Numrow6Key),
        '7' | '&' => Some(Keyboard::Numrow7Key),
        '8' | '*' => Some(Keyboard::Numrow8Key),
        '9' | '(' => Some(Keyboard::Numrow9Key),
        '`' | '~' => Some(Keyboard::BackquoteKey),
        '/' | '?' => Some(Keyboard::SlashKey),
        ',' | '<' => Some(Keyboard::CommaKey),
        '.' | '>' => Some(Keyboard::PeriodKey),
        '-' | '_' => Some(Keyboard::MinusKey),
        ';' | ':' => Some(Keyboard::SemicolonKey),
        '[' | '{' => Some(Keyboard::LBracketKey),
        ']' | '}' => Some(Keyboard::RBracketKey),
        '=' | '+' => Some(Keyboard::EqualKey),
        '\\' | '|' => Some(Keyboard::BackslashKey),
        '\'' | '"' => Some(Keyboard::QuoteKey),
        _ => None,
    }
}

pub struct KeySequence(pub &'static str);

impl KeySequence {
    pub fn send(&self) {
        for c in self.0.chars() {
            let mut uppercase = false;

            if let Some(keybd_key) = {
                if c.is_uppercase()
                    || [
                        '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '|',
                        ':', '"', '<', '>', '?', '~',
                    ]
                    .contains(&c)
                {
                    uppercase = true;
                }

                get_keybd_key(c)
            } {
                if uppercase {
                    Keyboard::LShiftKey.press();
                }

                keybd_key.press();
                sleep(Duration::from_millis(20));
                keybd_key.release();

                if uppercase {
                    Keyboard::LShiftKey.release();
                }
            };
        }
    }
}