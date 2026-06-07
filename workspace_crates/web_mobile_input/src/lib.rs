#![cfg(target_arch = "wasm32")]
// Base was from https://github.com/mirsella/bevy_mobile_input

use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    prelude::*,
};

use std::sync::Mutex;

use web_sys::{FocusOptions, HtmlInputElement, wasm_bindgen::JsCast};

static EVENT_QUEUE: Mutex<Vec<KeyboardInput>> = Mutex::new(Vec::new());

const MOBILE_INPUT_ELEMENT_ID: &str = "mobile-keyboard-input";

pub fn is_keyboard_already_visible() -> bool {
    let document = web_sys::window().unwrap().document().unwrap();
    let active = document.active_element();
    let input = document.get_element_by_id(MOBILE_INPUT_ELEMENT_ID);
    let result = match (active, input) {
        (Some(active), Some(input)) => active == input,
        _ => false,
    };
    debug!("is_keyboard_currently_visible_already: {result}");
    result
}

pub fn hide_keyboard() {
    let Some(input) = get_input_element() else {
        return;
    };
    debug!("hide_keyboard called");
    let _ = input.blur();
}

pub fn show_keyboard() {
    use web_sys::wasm_bindgen::prelude::ScopedClosure;

    let closure = ScopedClosure::wrap(Box::new(move || {
        if let Some(element) = get_input_element() {
            let options = FocusOptions::new();
            options.set_prevent_scroll(true);
            let _ = element.focus_with_options(&options);
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

fn get_input_element() -> Option<HtmlInputElement> {
    use web_sys::wasm_bindgen::JsCast;
    web_sys::window()?
        .document()?
        .get_element_by_id(MOBILE_INPUT_ELEMENT_ID)?
        .dyn_into::<HtmlInputElement>()
        .ok()
}

fn setup_event_listeners() {
    use web_sys::wasm_bindgen::prelude::ScopedClosure;

    let Some(input) = get_input_element() else {
        warn!("mobile-keyboard-input element not found");
        return;
    };

    let composition_handler = ScopedClosure::wrap(Box::new(move |e: web_sys::CompositionEvent| {
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

    let keydown_handler = ScopedClosure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        if event.is_composing() {
            return;
        }
        // Clear the input element after each character to prevent accumulation

        if let Some(el) = get_input_element() {
            el.set_value("");
        }
        let key = event.key();

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

    let _ = input
        .add_event_listener_with_callback("input", composition_handler.as_ref().unchecked_ref());
    let _ =
        input.add_event_listener_with_callback("keydown", keydown_handler.as_ref().unchecked_ref());

    composition_handler.forget();
    keydown_handler.forget();
}

fn poll_events(mut keyboard_events: MessageWriter<KeyboardInput>) {
    let Ok(mut queue) = EVENT_QUEUE.lock() else {
        return;
    };
    for event in queue.drain(..) {
        keyboard_events.write(event);
    }
}
