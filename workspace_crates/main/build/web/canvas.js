// canvas.js

function setupCanvasPointerBlock() {
    const canvas = document.getElementById("bevy_canvas");

    if (!canvas) {
        console.warn("Canvas not found: bevy_canvas");
        return;
    }

    let keyboardOpen = false;

    // Call these from your Rust via wasm_bindgen to update the state
    window.setKeyboardOpen = (val) => { keyboardOpen = val; };

    canvas.addEventListener("focus", () => {
        const input = document.getElementById("mobile-keyboard-input");
        if (keyboardOpen && document.activeElement !== input) {
            input?.focus();
        }
    }, { capture: true });
}

// Run as soon as DOM is ready
if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", setupCanvasPointerBlock);
} else {
    setupCanvasPointerBlock();
}