use crate::conversions;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    prelude::EventReader,
    system::{Res, ResMut, Resource, SystemParam},
};
use bevy_input::keyboard::KeyCode;
use bevy_input::touch::TouchInput;
use bevy_input::{
    keyboard::KeyboardInput,
    mouse::{MouseButtonInput, MouseWheel},
    ButtonInput, ButtonState,
};
use bevy_window::{CursorEntered, CursorLeft, CursorMoved, ReceivedCharacter};
use iced_core::keyboard::Key;
use iced_core::{keyboard, mouse, Event as IcedEvent, Point};

#[derive(Resource, Deref, DerefMut, Default)]
pub struct IcedEventQueue(Vec<iced_core::Event>);

#[derive(SystemParam)]
pub struct InputEvents<'w, 's> {
    cursor_entered: EventReader<'w, 's, CursorEntered>,
    cursor_left: EventReader<'w, 's, CursorLeft>,
    cursor: EventReader<'w, 's, CursorMoved>,
    mouse_button: EventReader<'w, 's, MouseButtonInput>,
    mouse_wheel: EventReader<'w, 's, MouseWheel>,
    received_character: EventReader<'w, 's, ReceivedCharacter>,
    keyboard_input: EventReader<'w, 's, KeyboardInput>,
    touch_input: EventReader<'w, 's, TouchInput>,
}

fn compute_modifiers(input_map: &ButtonInput<KeyCode>) -> keyboard::Modifiers {
    let mut modifiers = keyboard::Modifiers::default();
    if input_map.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
        modifiers |= keyboard::Modifiers::CTRL;
    }
    if input_map.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        modifiers |= keyboard::Modifiers::SHIFT;
    }
    if input_map.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
        modifiers |= keyboard::Modifiers::ALT;
    }
    if input_map.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]) {
        modifiers |= keyboard::Modifiers::LOGO;
    }
    modifiers
}

pub fn process_input(
    mut events: InputEvents,
    mut event_queue: ResMut<IcedEventQueue>,
    input_map: Res<ButtonInput<KeyCode>>,
) {
    event_queue.clear();

    for ev in events.cursor.read() {
        event_queue.push(IcedEvent::Mouse(mouse::Event::CursorMoved {
            position: Point::new(ev.position.x, ev.position.y),
        }));
    }

    for ev in events.mouse_button.read() {
        let button = conversions::mouse_button(ev.button);
        event_queue.push(IcedEvent::Mouse(match ev.state {
            ButtonState::Pressed => mouse::Event::ButtonPressed(button),
            ButtonState::Released => mouse::Event::ButtonReleased(button),
        }))
    }

    for _ev in events.cursor_entered.read() {
        event_queue.push(IcedEvent::Mouse(mouse::Event::CursorEntered));
    }

    for _ev in events.cursor_left.read() {
        event_queue.push(IcedEvent::Mouse(mouse::Event::CursorLeft));
    }

    for ev in events.mouse_wheel.read() {
        event_queue.push(IcedEvent::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Pixels { x: ev.x, y: ev.y },
        }));
    }

    for ev in events.received_character.read() {
        event_queue.push(IcedEvent::Keyboard(keyboard::Event::KeyPressed {
            key: Key::Character(ev.char.clone()),
            location: keyboard::Location::Standard,
            modifiers: Default::default(),
            text: Some(ev.char.clone()),
        }));
    }

    for ev in events.keyboard_input.read() {
        let bevy_key_code = ev.key_code;
        use keyboard::Event::*;
        let modifiers = compute_modifiers(&input_map);
        let event = match bevy_key_code {
            KeyCode::ControlLeft
            | KeyCode::ControlRight
            | KeyCode::ShiftLeft
            | KeyCode::ShiftRight
            | KeyCode::AltLeft
            | KeyCode::AltRight
            | KeyCode::SuperLeft
            | KeyCode::SuperRight => ModifiersChanged(modifiers),
            bevy_key_code => {
                let iced_key_code = conversions::key_code(bevy_key_code);
                if ev.state.is_pressed() {
                    match iced_key_code.clone() {
                        Key::Character(x) => KeyPressed {
                            key: iced_key_code,
                            modifiers,
                            location: keyboard::Location::Standard,
                            text: Some(x),
                        },
                        Key::Named(_) | Key::Unidentified => KeyPressed {
                            key: iced_key_code,
                            modifiers,
                            location: keyboard::Location::Standard,
                            text: None,
                        },
                    }
                } else {
                    KeyReleased {
                        key: iced_key_code,
                        modifiers,
                        location: keyboard::Location::Standard,
                    }
                }
            }
        };

        event_queue.push(IcedEvent::Keyboard(event));
    }

    for ev in events.touch_input.read() {
        event_queue.push(IcedEvent::Touch(conversions::touch_event(ev)));
    }
}
