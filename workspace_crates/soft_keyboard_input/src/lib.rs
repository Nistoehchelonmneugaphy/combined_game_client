use bevy::prelude::*;

pub struct SoftKeyboardInputPlugin;

impl Plugin for SoftKeyboardInputPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_os = "android")]
        app.add_plugins(android::SoftKeyboardInputPlugin);
        #[cfg(target_arch = "wasm32")]
        app.add_plugins(browser::SoftKeyboardInputPlugin);

        // I don't suppose that any other target really needs this plugin... Is this right?
        // let _ = app in order to remove clippy warning about unused var app. Alternative is to do conditional parameter declaration in signature with app on wasm/android and _app on non wasm android, though doing that seems verbose. Not sure which is better, because the let _ statement also looks kinda meh.
        #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
        let _ = app;
    }
}

#[cfg(target_os = "android")]
mod android;
#[cfg(target_arch = "wasm32")]
mod browser;

// In principle, I could detect whether the user is using a hardware keyboard whether on PC or mobile or browser, and only show if the answer is no (in terms of PC, for example the user could be using a laptop with detachable screen in tablet mode). But for now I just show if user is on android or mobile wasm for simplicity.

pub fn show_soft_keyboard_if_needed(existing_value: &str, hint: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if web::is_mobile() && !browser::is_keyboard_shown() {
            browser::show_keyboard();
        }
    }
    #[cfg(target_os = "android")]
    {
        android::show_keyboard(existing_value, hint);
    }

    // do nothing on non-wasm/droid targets, silence clippy unused warnings using the let _ _ below
    #[cfg(not(any(target_os = "android", target_arch = "wasm32")))]
    {
        let (_, _) = (existing_value, hint);
    }
}

pub fn hide_soft_keyboard_if_needed() {
    #[cfg(target_arch = "wasm32")]
    {
        if web::is_mobile() && browser::is_keyboard_shown() {
            browser::hide_keyboard();
        }
    }
    #[cfg(target_os = "android")]
    {
        android::hide_keyboard();
    }
}
