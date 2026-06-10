#![cfg(target_os = "android")]
use bevy::{
    android::android_activity::AndroidApp,
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    prelude::*,
};
use jni::{JValue, jni_sig, jni_str, objects::JObject, refs::Global};
// use jni::{JValue, jni_sig, jni_str, objects::JObject};
// use ndk_context::android_context;
use std::sync::Mutex;

pub static EVENT_QUEUE: Mutex<Vec<KeyboardInput>> = Mutex::new(Vec::new());

pub struct AndroidMobileInputPlugin;

impl Plugin for AndroidMobileInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, poll_events);
    }
}

pub fn show_keyboard(hint: &str) {
    let existing_value = "test";
    warn!("Calling android_mobile_input::show_keyboard()");
    let android_app = bevy::android::ANDROID_APP.get().unwrap();

    let vm = unsafe { jni::JavaVM::from_raw(android_app.vm_as_ptr().cast()) };

    // I think the android_app and vm declarations are fine. The below needs to be potentially modified to work.

    let result = vm.attach_current_thread(|env| {
        let raw_activity_global = android_app.activity_as_ptr() as jni::sys::jobject;
        // SAFETY: The reference / pointer is valid as long as `app` is valid
        let activity = unsafe { env.as_cast_raw::<Global<JObject>>(&raw_activity_global)? };
        let hint_jstring = env.new_string(hint).unwrap();
        let existing_value_jstring = env.new_string(existing_value).unwrap();
        env.call_method(
            &activity,
            jni_str!("showFloatingInput"),
            jni_sig!("(Ljava/lang/String;Ljava/lang/String;)V"),
            &[JValue::Object(&existing_value_jstring.into()), JValue::Object(&hint_jstring.into())],
        )
        .unwrap();

        Ok::<_, jni::errors::Error>(())
    });
    result.unwrap();
}
pub fn hide_keyboard() {
    return;
    // let ctx = android_context();
    // let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) };
    // let result = vm.attach_current_thread(|env| {
    //     let activity = unsafe { JObject::from_raw(env, ctx.context() as _) };

    //     env.call_method(
    //         &activity,
    //         jni_str!("hideFloatingInput"),
    //         jni_sig!("()V"),
    //         &[],
    //     )
    //     .unwrap();

    //     Ok::<_, jni::errors::Error>(())
    // });
    // result.unwrap();
}

fn poll_events(mut keyboard_events: MessageWriter<KeyboardInput>) {
    let Ok(mut queue) = EVENT_QUEUE.lock() else {
        return;
    };
    for event in queue.drain(..) {
        keyboard_events.write(event);
    }
}

// OMG my first unsafe code!!!
#[unsafe(no_mangle)]
pub extern "system" fn Java_org_cgc_main_FloatingInput_onTextChanged(
    _env: jni::EnvUnowned,
    _class: jni::objects::JClass,
    text: jni::objects::JString,
) {
    
    let text: String = jni::objects::JString::from(text).to_string();
warn!("Called Rust fn from Java, Yay! text: {text}");
    let Ok(mut queue) = EVENT_QUEUE.lock() else {
        return;
    };
    for ch in text.chars() {
        let event = KeyboardInput {
            key_code: KeyCode::Unidentified(bevy::input::keyboard::NativeKeyCode::Unidentified),
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
#[derive(Resource)]
pub struct AndroidAppResource(pub AndroidApp);

/// Called from Java for Enter / Backspace
#[unsafe(no_mangle)]
pub extern "C" fn Java_org_cgc_main_FloatingInput_onSpecialKey(
    _env: jni::EnvUnowned,
    _class: jni::objects::JClass,
    key: jni::objects::JString,
) {
    let key: String = jni::objects::JString::from(key).to_string();
    warn!("Java_org_cgc_main_FloatingInput_onSpecialKey: {key}");
    let (key_code, logical_key) = match key.as_str() {
        "Enter" => (KeyCode::Enter, Key::Enter),
        "Backspace" => (KeyCode::Backspace, Key::Backspace),
        "Escape" => (KeyCode::Escape, Key::Escape),
        _ => return,
    };
    let Ok(mut queue) = EVENT_QUEUE.lock() else {
        return;
    };
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
