#![cfg(target_os = "android")]
use bevy::{
    android::android_activity::AndroidApp, input::{ButtonState, keyboard::{Key, KeyboardInput}}, prelude::*
};
use jni::vm::JavaVM;
use std::sync::Mutex;

pub static EVENT_QUEUE: Mutex<Vec<KeyboardInput>> = Mutex::new(Vec::new());

pub struct AndroidMobileInputPlugin;

impl Plugin for AndroidMobileInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, poll_events);
    }
}


fn get_java_vm(app: &AndroidApp) -> JavaVM {
    // AndroidApp exposes the raw JavaVM pointer
    unsafe { JavaVM::from_raw(app.vm_as_ptr() as *mut _) }
}


fn poll_events(mut keyboard_events: MessageWriter<KeyboardInput>) {
    let Ok(mut queue) = EVENT_QUEUE.lock() else { return };
    for event in queue.drain(..) {
        keyboard_events.write(event);
    }
}