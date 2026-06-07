#![cfg(target_arch = "wasm32")]
use bevy::prelude::*;

pub fn is_mobile() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    // check if touch is supported... maybe user_agent check would be more useful, though?
    //  or other methods?
    let result = window.navigator().max_touch_points() > 0;
    debug!("is_mobile call returns {result}");
    result
}