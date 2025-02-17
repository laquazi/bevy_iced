use crate::iced::{
    touch::{self, Finger},
    Point,
};
use bevy_input::prelude::KeyCode as BevyKeyCode;
use bevy_input::prelude::MouseButton;
use bevy_input::touch::{TouchInput, TouchPhase};
use bevy_math::Vec2;
use iced_core::keyboard::key::Named;
use iced_core::keyboard::Key as IcedKeyCode;

pub fn key_code(virtual_keycode: BevyKeyCode) -> IcedKeyCode {
    match virtual_keycode {
        BevyKeyCode::Unidentified(_) => todo!(),
        BevyKeyCode::Backquote => todo!(),
        BevyKeyCode::Backslash => todo!(),
        BevyKeyCode::BracketLeft => todo!(),
        BevyKeyCode::BracketRight => todo!(),
        BevyKeyCode::Comma => todo!(),
        BevyKeyCode::Digit0 => IcedKeyCode::Character("0".into()),
        BevyKeyCode::Digit1 => IcedKeyCode::Character("1".into()),
        BevyKeyCode::Digit2 => IcedKeyCode::Character("2".into()),
        BevyKeyCode::Digit3 => IcedKeyCode::Character("3".into()),
        BevyKeyCode::Digit4 => IcedKeyCode::Character("4".into()),
        BevyKeyCode::Digit5 => IcedKeyCode::Character("5".into()),
        BevyKeyCode::Digit6 => IcedKeyCode::Character("6".into()),
        BevyKeyCode::Digit7 => IcedKeyCode::Character("7".into()),
        BevyKeyCode::Digit8 => IcedKeyCode::Character("8".into()),
        BevyKeyCode::Digit9 => IcedKeyCode::Character("9".into()),
        BevyKeyCode::Equal => todo!(),
        BevyKeyCode::IntlBackslash => todo!(),
        BevyKeyCode::IntlRo => todo!(),
        BevyKeyCode::IntlYen => todo!(),
        BevyKeyCode::KeyA => IcedKeyCode::Character("a".into()),
        BevyKeyCode::KeyB => IcedKeyCode::Character("b".into()),
        BevyKeyCode::KeyC => IcedKeyCode::Character("c".into()),
        BevyKeyCode::KeyD => IcedKeyCode::Character("d".into()),
        BevyKeyCode::KeyE => IcedKeyCode::Character("e".into()),
        BevyKeyCode::KeyF => IcedKeyCode::Character("f".into()),
        BevyKeyCode::KeyG => IcedKeyCode::Character("g".into()),
        BevyKeyCode::KeyH => IcedKeyCode::Character("h".into()),
        BevyKeyCode::KeyI => IcedKeyCode::Character("i".into()),
        BevyKeyCode::KeyJ => IcedKeyCode::Character("j".into()),
        BevyKeyCode::KeyK => IcedKeyCode::Character("k".into()),
        BevyKeyCode::KeyL => IcedKeyCode::Character("l".into()),
        BevyKeyCode::KeyM => IcedKeyCode::Character("m".into()),
        BevyKeyCode::KeyN => IcedKeyCode::Character("n".into()),
        BevyKeyCode::KeyO => IcedKeyCode::Character("o".into()),
        BevyKeyCode::KeyP => IcedKeyCode::Character("p".into()),
        BevyKeyCode::KeyQ => IcedKeyCode::Character("q".into()),
        BevyKeyCode::KeyR => IcedKeyCode::Character("r".into()),
        BevyKeyCode::KeyS => IcedKeyCode::Character("s".into()),
        BevyKeyCode::KeyT => IcedKeyCode::Character("t".into()),
        BevyKeyCode::KeyU => IcedKeyCode::Character("u".into()),
        BevyKeyCode::KeyV => IcedKeyCode::Character("v".into()),
        BevyKeyCode::KeyW => IcedKeyCode::Character("w".into()),
        BevyKeyCode::KeyX => IcedKeyCode::Character("x".into()),
        BevyKeyCode::KeyY => IcedKeyCode::Character("y".into()),
        BevyKeyCode::KeyZ => IcedKeyCode::Character("z".into()),
        BevyKeyCode::Minus => todo!(),
        BevyKeyCode::Period => todo!(),
        BevyKeyCode::Quote => todo!(),
        BevyKeyCode::Semicolon => todo!(),
        BevyKeyCode::Slash => todo!(),
        BevyKeyCode::AltLeft => todo!(),
        BevyKeyCode::AltRight => todo!(),
        BevyKeyCode::Backspace => IcedKeyCode::Named(Named::Backspace),
        BevyKeyCode::CapsLock => IcedKeyCode::Named(Named::CapsLock),
        BevyKeyCode::ContextMenu => IcedKeyCode::Named(Named::ContextMenu),
        BevyKeyCode::ControlLeft => todo!(),
        BevyKeyCode::ControlRight => todo!(),
        BevyKeyCode::Enter => IcedKeyCode::Named(Named::Enter),
        BevyKeyCode::SuperLeft => todo!(),
        BevyKeyCode::SuperRight => todo!(),
        BevyKeyCode::ShiftLeft => todo!(),
        BevyKeyCode::ShiftRight => todo!(),
        BevyKeyCode::Space => IcedKeyCode::Named(Named::Space),
        BevyKeyCode::Tab => IcedKeyCode::Named(Named::Tab),
        BevyKeyCode::Convert => IcedKeyCode::Named(Named::Convert),
        BevyKeyCode::KanaMode => IcedKeyCode::Named(Named::KanaMode),
        BevyKeyCode::Lang1 => todo!(),
        BevyKeyCode::Lang2 => todo!(),
        BevyKeyCode::Lang3 => todo!(),
        BevyKeyCode::Lang4 => todo!(),
        BevyKeyCode::Lang5 => todo!(),
        BevyKeyCode::NonConvert => IcedKeyCode::Named(Named::NonConvert),
        BevyKeyCode::Delete => IcedKeyCode::Named(Named::Delete),
        BevyKeyCode::End => IcedKeyCode::Named(Named::End),
        BevyKeyCode::Help => IcedKeyCode::Named(Named::Help),
        BevyKeyCode::Home => IcedKeyCode::Named(Named::Home),
        BevyKeyCode::Insert => IcedKeyCode::Named(Named::Insert),
        BevyKeyCode::PageDown => IcedKeyCode::Named(Named::PageDown),
        BevyKeyCode::PageUp => IcedKeyCode::Named(Named::PageUp),
        BevyKeyCode::ArrowDown => IcedKeyCode::Named(Named::ArrowDown),
        BevyKeyCode::ArrowLeft => IcedKeyCode::Named(Named::ArrowLeft),
        BevyKeyCode::ArrowRight => IcedKeyCode::Named(Named::ArrowRight),
        BevyKeyCode::ArrowUp => IcedKeyCode::Named(Named::ArrowUp),
        BevyKeyCode::NumLock => IcedKeyCode::Named(Named::NumLock),
        BevyKeyCode::Numpad0 => todo!(),
        BevyKeyCode::Numpad1 => todo!(),
        BevyKeyCode::Numpad2 => todo!(),
        BevyKeyCode::Numpad3 => todo!(),
        BevyKeyCode::Numpad4 => todo!(),
        BevyKeyCode::Numpad5 => todo!(),
        BevyKeyCode::Numpad6 => todo!(),
        BevyKeyCode::Numpad7 => todo!(),
        BevyKeyCode::Numpad8 => todo!(),
        BevyKeyCode::Numpad9 => todo!(),
        BevyKeyCode::NumpadAdd => todo!(),
        BevyKeyCode::NumpadBackspace => todo!(),
        BevyKeyCode::NumpadClear => todo!(),
        BevyKeyCode::NumpadClearEntry => todo!(),
        BevyKeyCode::NumpadComma => todo!(),
        BevyKeyCode::NumpadDecimal => todo!(),
        BevyKeyCode::NumpadDivide => todo!(),
        BevyKeyCode::NumpadEnter => todo!(),
        BevyKeyCode::NumpadEqual => todo!(),
        BevyKeyCode::NumpadHash => todo!(),
        BevyKeyCode::NumpadMemoryAdd => todo!(),
        BevyKeyCode::NumpadMemoryClear => todo!(),
        BevyKeyCode::NumpadMemoryRecall => todo!(),
        BevyKeyCode::NumpadMemoryStore => todo!(),
        BevyKeyCode::NumpadMemorySubtract => todo!(),
        BevyKeyCode::NumpadMultiply => todo!(),
        BevyKeyCode::NumpadParenLeft => todo!(),
        BevyKeyCode::NumpadParenRight => todo!(),
        BevyKeyCode::NumpadStar => todo!(),
        BevyKeyCode::NumpadSubtract => todo!(),
        BevyKeyCode::Escape => IcedKeyCode::Named(Named::Escape),
        BevyKeyCode::Fn => IcedKeyCode::Named(Named::Fn),
        BevyKeyCode::FnLock => IcedKeyCode::Named(Named::FnLock),
        BevyKeyCode::PrintScreen => IcedKeyCode::Named(Named::PrintScreen),
        BevyKeyCode::ScrollLock => IcedKeyCode::Named(Named::ScrollLock),
        BevyKeyCode::Pause => IcedKeyCode::Named(Named::Pause),
        BevyKeyCode::BrowserBack => IcedKeyCode::Named(Named::BrowserBack),
        BevyKeyCode::BrowserFavorites => IcedKeyCode::Named(Named::BrowserFavorites),
        BevyKeyCode::BrowserForward => IcedKeyCode::Named(Named::BrowserForward),
        BevyKeyCode::BrowserHome => IcedKeyCode::Named(Named::BrowserHome),
        BevyKeyCode::BrowserRefresh => IcedKeyCode::Named(Named::BrowserRefresh),
        BevyKeyCode::BrowserSearch => IcedKeyCode::Named(Named::BrowserSearch),
        BevyKeyCode::BrowserStop => IcedKeyCode::Named(Named::BrowserStop),
        BevyKeyCode::Eject => IcedKeyCode::Named(Named::Eject),
        BevyKeyCode::LaunchApp1 => todo!(),
        BevyKeyCode::LaunchApp2 => todo!(),
        BevyKeyCode::LaunchMail => IcedKeyCode::Named(Named::LaunchMail),
        BevyKeyCode::MediaPlayPause => IcedKeyCode::Named(Named::MediaPlayPause),
        BevyKeyCode::MediaSelect => todo!(),
        BevyKeyCode::MediaStop => IcedKeyCode::Named(Named::MediaStop),
        BevyKeyCode::MediaTrackNext => IcedKeyCode::Named(Named::MediaTrackNext),
        BevyKeyCode::MediaTrackPrevious => IcedKeyCode::Named(Named::MediaTrackPrevious),
        BevyKeyCode::Power => IcedKeyCode::Named(Named::Power),
        BevyKeyCode::Sleep => todo!(),
        BevyKeyCode::AudioVolumeDown => IcedKeyCode::Named(Named::AudioVolumeDown),
        BevyKeyCode::AudioVolumeMute => IcedKeyCode::Named(Named::AudioVolumeMute),
        BevyKeyCode::AudioVolumeUp => IcedKeyCode::Named(Named::AudioVolumeUp),
        BevyKeyCode::WakeUp => IcedKeyCode::Named(Named::WakeUp),
        BevyKeyCode::Meta => IcedKeyCode::Named(Named::Meta),
        BevyKeyCode::Hyper => IcedKeyCode::Named(Named::Hyper),
        BevyKeyCode::Turbo => todo!(),
        BevyKeyCode::Abort => todo!(),
        BevyKeyCode::Resume => todo!(),
        BevyKeyCode::Suspend => todo!(),
        BevyKeyCode::Again => IcedKeyCode::Named(Named::Again),
        BevyKeyCode::Copy => IcedKeyCode::Named(Named::Copy),
        BevyKeyCode::Cut => IcedKeyCode::Named(Named::Cut),
        BevyKeyCode::Find => IcedKeyCode::Named(Named::Find),
        BevyKeyCode::Open => IcedKeyCode::Named(Named::Open),
        BevyKeyCode::Paste => IcedKeyCode::Named(Named::Paste),
        BevyKeyCode::Props => IcedKeyCode::Named(Named::Props),
        BevyKeyCode::Select => IcedKeyCode::Named(Named::Select),
        BevyKeyCode::Undo => IcedKeyCode::Named(Named::Undo),
        BevyKeyCode::Hiragana => IcedKeyCode::Named(Named::Hiragana),
        BevyKeyCode::Katakana => IcedKeyCode::Named(Named::Katakana),
        BevyKeyCode::F1 => IcedKeyCode::Named(Named::F1),
        BevyKeyCode::F2 => IcedKeyCode::Named(Named::F2),
        BevyKeyCode::F3 => IcedKeyCode::Named(Named::F3),
        BevyKeyCode::F4 => IcedKeyCode::Named(Named::F4),
        BevyKeyCode::F5 => IcedKeyCode::Named(Named::F5),
        BevyKeyCode::F6 => IcedKeyCode::Named(Named::F6),
        BevyKeyCode::F7 => IcedKeyCode::Named(Named::F7),
        BevyKeyCode::F8 => IcedKeyCode::Named(Named::F8),
        BevyKeyCode::F9 => IcedKeyCode::Named(Named::F9),
        BevyKeyCode::F10 => IcedKeyCode::Named(Named::F10),
        BevyKeyCode::F11 => IcedKeyCode::Named(Named::F11),
        BevyKeyCode::F12 => IcedKeyCode::Named(Named::F12),
        BevyKeyCode::F13 => IcedKeyCode::Named(Named::F13),
        BevyKeyCode::F14 => IcedKeyCode::Named(Named::F14),
        BevyKeyCode::F15 => IcedKeyCode::Named(Named::F15),
        BevyKeyCode::F16 => IcedKeyCode::Named(Named::F16),
        BevyKeyCode::F17 => IcedKeyCode::Named(Named::F17),
        BevyKeyCode::F18 => IcedKeyCode::Named(Named::F18),
        BevyKeyCode::F19 => IcedKeyCode::Named(Named::F19),
        BevyKeyCode::F20 => IcedKeyCode::Named(Named::F20),
        BevyKeyCode::F21 => IcedKeyCode::Named(Named::F21),
        BevyKeyCode::F22 => IcedKeyCode::Named(Named::F22),
        BevyKeyCode::F23 => IcedKeyCode::Named(Named::F23),
        BevyKeyCode::F24 => IcedKeyCode::Named(Named::F24),
        BevyKeyCode::F25 => IcedKeyCode::Named(Named::F25),
        BevyKeyCode::F26 => IcedKeyCode::Named(Named::F26),
        BevyKeyCode::F27 => IcedKeyCode::Named(Named::F27),
        BevyKeyCode::F28 => IcedKeyCode::Named(Named::F28),
        BevyKeyCode::F29 => IcedKeyCode::Named(Named::F29),
        BevyKeyCode::F30 => IcedKeyCode::Named(Named::F30),
        BevyKeyCode::F31 => IcedKeyCode::Named(Named::F31),
        BevyKeyCode::F32 => IcedKeyCode::Named(Named::F32),
        BevyKeyCode::F33 => IcedKeyCode::Named(Named::F33),
        BevyKeyCode::F34 => IcedKeyCode::Named(Named::F34),
        BevyKeyCode::F35 => IcedKeyCode::Named(Named::F35),
    }
}

pub fn mouse_button(button: MouseButton) -> iced_core::mouse::Button {
    use iced_core::mouse::Button;
    match button {
        MouseButton::Left => Button::Left,
        MouseButton::Right => Button::Right,
        MouseButton::Middle => Button::Middle,
        MouseButton::Back => Button::Back,
        MouseButton::Forward => Button::Forward,
        MouseButton::Other(val) => Button::Other(val),
    }
}

pub fn touch_event(bevy_touch_input: &TouchInput) -> touch::Event {
    match *bevy_touch_input {
        TouchInput {
            phase: TouchPhase::Started,
            position: Vec2 { x, y },
            id: finger,
            ..
        } => touch::Event::FingerPressed {
            id: Finger(finger),
            position: Point { x, y },
        },
        TouchInput {
            phase: TouchPhase::Canceled,
            position: Vec2 { x, y },
            id: finger,
            ..
        } => touch::Event::FingerLost {
            id: Finger(finger),
            position: Point { x, y },
        },
        TouchInput {
            phase: TouchPhase::Ended,
            position: Vec2 { x, y },
            id: finger,
            ..
        } => touch::Event::FingerLifted {
            id: Finger(finger),
            position: Point { x, y },
        },
        TouchInput {
            phase: TouchPhase::Moved,
            position: Vec2 { x, y },
            id: finger,
            ..
        } => touch::Event::FingerMoved {
            id: Finger(finger),
            position: Point { x, y },
        },
    }
}
