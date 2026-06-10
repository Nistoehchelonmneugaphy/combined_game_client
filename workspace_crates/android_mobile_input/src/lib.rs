#![cfg(target_os = "android")]
use bevy::{
    android::android_activity::AndroidApp,
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    prelude::*,
};

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

pub fn show_keyboard(_hint: &str) {
    return;
    // below does nothing except crash, so...
    // let ctx = android_context();
    // let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) };
    // let result = vm.attach_current_thread(|env| {
    //             println!("{vm:#?}");
    //     let activity = env
    //         .call_static_method(
    //             jni_str!("org/cgc/main/MainActivity"),
    //             jni_str!("getInstance"),
    //             jni_sig!("()Lorg/cgc/main/MainActivity;"),
    //             &[],
    //         )?
    //         .l()?;
    //     let hint_jstring = env.new_string(hint).unwrap();
    //     let cls = env.get_object_class(&activity)?;
    //     let name = env
    //         .call_method(
    //             &cls,
    //             jni_str!("getName"),
    //             jni_sig!("()Ljava/lang/String;"),
    //             &[],
    //         )?
    //         .l()?;

    //     warn!("Activity class = {:#?}", name);
    //     env.call_method(
    //         &activity,
    //         jni_str!("showFloatingInput"),
    //         jni_sig!("(Ljava/lang/String;)V"),
    //         &[JValue::Object(&hint_jstring.into())],
    //     )
    //     .unwrap();

    //     Ok::<_, jni::errors::Error>(())
    // });
    // result.unwrap();
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
// #[unsafe(no_mangle)]
// pub extern "C" fn Java_org_cgc_main_FloatingInput_onTextChanged(
//     _env: jni::EnvUnowned,
//     _class: jni::objects::JClass,
//     text: jni::objects::JString,
// ) {
//     let text: String = jni::objects::JString::from(text).to_string();
//     let Ok(mut queue) = EVENT_QUEUE.lock() else {
//         return;
//     };
//     for ch in text.chars() {
//         let event = KeyboardInput {
//             key_code: KeyCode::Unidentified(bevy::input::keyboard::NativeKeyCode::Unidentified),
//             logical_key: Key::Character(ch.to_string().into()),
//             state: ButtonState::Pressed,
//             repeat: false,
//             text: Some(ch.to_string().into()),
//             window: Entity::PLACEHOLDER,
//         };
//         let mut release = event.clone();
//         release.state = ButtonState::Released;
//         release.text = None;
//         queue.push(event);
//         queue.push(release);
//     }
// }
// #[derive(Resource)]
// pub struct AndroidAppResource(pub AndroidApp);

// /// Called from Java for Enter / Backspace
// #[unsafe(no_mangle)]
// pub extern "C" fn Java_org_cgc_main_FloatingInput_onSpecialKey(
//     _env: jni::EnvUnowned,
//     _class: jni::objects::JClass,
//     key: jni::objects::JString,
// ) {
//     let key: String = jni::objects::JString::from(key).to_string();
//     let (key_code, logical_key) = match key.as_str() {
//         "Enter" => (KeyCode::Enter, Key::Enter),
//         "Backspace" => (KeyCode::Backspace, Key::Backspace),
//         "Escape" => (KeyCode::Escape, Key::Escape),
//         _ => return,
//     };
//     let Ok(mut queue) = EVENT_QUEUE.lock() else {
//         return;
//     };
//     let event = KeyboardInput {
//         key_code,
//         logical_key,
//         state: ButtonState::Pressed,
//         repeat: false,
//         text: None,
//         window: Entity::PLACEHOLDER,
//     };
//     let mut release = event.clone();
//     release.state = ButtonState::Released;
//     release.text = None;

//     queue.push(event);
//     queue.push(release);
// }
