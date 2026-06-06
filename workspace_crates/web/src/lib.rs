#[cfg(target_arch = "wasm32")]
pub fn is_mobile() -> bool {
    let Some(window) = web_sys::window() else {
        return false;
    };
    // check if touch is supported
    window.navigator().max_touch_points() > 0
}