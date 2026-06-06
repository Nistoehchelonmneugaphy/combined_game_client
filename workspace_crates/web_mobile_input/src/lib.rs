// Base was from https://github.com/mirsella/bevy_mobile_input

use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    prelude::*,
};
use std::sync::Mutex;
use web_sys::{
    HtmlInputElement,
    wasm_bindgen::{JsCast, prelude::Closure},
};

static EVENT_QUEUE: Mutex<Vec<KeyboardInput>> = Mutex::new(Vec::new());

#[cfg(target_arch = "wasm32")]
pub fn hide_keyboard() {
    let Some(input) = get_input_element() else {
        return;
    };
    let _ = input.class_list().remove_1("active");
    let _ = input.blur();
}

#[cfg(target_arch = "wasm32")]
pub fn show_keyboard() {
    let Some(input) = get_input_element() else {
        return;
    };
    let _ = input.class_list().add_1("active");

    let closure = Closure::wrap(Box::new(move || {
        if let Some(el) = get_input_element() {
            let _ = el.focus();
        }
    }) as Box<dyn FnMut()>);

    web_sys::window()
        .unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            200, // 200ms should be enough for winit to finish
        )
        .unwrap();
    closure.forget();
}

pub struct MobileInputPlugin;

impl Plugin for MobileInputPlugin {
    fn build(&self, app: &mut App) {
        setup_event_listeners();
        app.add_systems(First, poll_events);
    }
}

#[cfg(target_arch = "wasm32")]
fn get_input_element() -> Option<HtmlInputElement> {
    use web_sys::wasm_bindgen::JsCast;
    web_sys::window()?
        .document()?
        .get_element_by_id("mobile-keyboard-input")?
        .dyn_into::<HtmlInputElement>()
        .ok()
}

#[cfg(target_arch = "wasm32")]
fn setup_event_listeners() {
    let Some(input) = get_input_element() else {
        warn!("mobile-keyboard-input element not found");
        return;
    };

    let composition_handler = Closure::wrap(Box::new(move |e: web_sys::CompositionEvent| {
        let Some(data) = e.data() else { return };

        web_sys::console::log_1(&format!("compositionend: data={:?}", data).into());

        // Clear the input element
        if let Some(el) = get_input_element() {
            el.set_value("");
        }

        for ch in data.chars() {
            if let Ok(mut queue) = EVENT_QUEUE.lock() {
                let event = KeyboardInput {
                    key_code: KeyCode::Unidentified(
                        bevy::input::keyboard::NativeKeyCode::Unidentified,
                    ),
                    logical_key: Key::Character(ch.to_string().into()),
                    state: ButtonState::Pressed,
                    repeat: false,
                    text: Some(ch.to_string().into()),
                    window: Entity::PLACEHOLDER,
                };
                let mut release = event.clone();
                release.state = ButtonState::Released;
                release.text = None;
                queue.push(event);
                queue.push(release);
            }
        }
    }) as Box<dyn FnMut(_)>);

    let _ = input.add_event_listener_with_callback(
        "compositionend",
        composition_handler.as_ref().unchecked_ref(),
    );

    let keydown_handler = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        if e.is_composing() {
            return;
        }
        // Clear the input element after each character to prevent accumulation
        if let Some(el) = get_input_element() {
            el.set_value("");
        }
        let key = e.key();

        let (key_code, logical_key) = match key.as_str() {
            "Enter" => (KeyCode::Enter, Key::Enter),
            "Backspace" => (KeyCode::Backspace, Key::Backspace),
            "Escape" => (KeyCode::Escape, Key::Escape),
            _ => return,
        };

        if let Ok(mut queue) = EVENT_QUEUE.lock() {
            let event = KeyboardInput {
                key_code,
                logical_key,
                state: ButtonState::Pressed,
                repeat: false,
                text: None,
                window: Entity::PLACEHOLDER,
            };
            let mut release = event.clone();
            release.state = ButtonState::Released;
            release.text = None;

            queue.push(event);
            queue.push(release);
        }

        if key == "Enter" {
            if let Some(el) = get_input_element() {
                el.set_value("");
            }
            hide_keyboard();
        }
    }) as Box<dyn FnMut(_)>);

    let blur_handler = Closure::wrap(Box::new(move |_: web_sys::FocusEvent| {
        hide_keyboard();
    }) as Box<dyn FnMut(_)>);

    let _ = input
        .add_event_listener_with_callback("input", composition_handler.as_ref().unchecked_ref());
    let _ =
        input.add_event_listener_with_callback("keydown", keydown_handler.as_ref().unchecked_ref());
    let _ = input.add_event_listener_with_callback("blur", blur_handler.as_ref().unchecked_ref());

    blur_handler.forget();
    composition_handler.forget();
    keydown_handler.forget();
}

#[cfg(target_arch = "wasm32")]
fn poll_events(mut keyboard_events: MessageWriter<KeyboardInput>) {
    let Ok(mut queue) = EVENT_QUEUE.lock() else {
        return;
    };
    for event in queue.drain(..) {
        keyboard_events.write(event);
    }
}
