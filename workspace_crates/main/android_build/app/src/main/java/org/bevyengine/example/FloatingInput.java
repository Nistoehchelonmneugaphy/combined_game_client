// FloatingInput.java
package org.cgc.main;

import android.content.Context;
import android.graphics.*;
import android.text.*;
import android.view.*;
import android.view.inputmethod.*;
import android.widget.*;

public class FloatingInput {

    // JNI callbacks — implemented in your Rust lib
    public static native void onTextChanged(String text);
    public static native void onSpecialKey(String keyName);

    private final WindowManager windowManager;
    private EditText editText;
    private View container;
    private boolean visible = false;

    public FloatingInput(Context context, WindowManager wm) {
        this.windowManager = wm;
        buildView(context);
    }

    private void buildView(Context context) {
        // Semi-transparent pill sitting just above the IME
        container = new FrameLayout(context);
        container.setBackgroundColor(Color.argb(220, 30, 30, 30));

        editText = new EditText(context);
        editText.setTextColor(Color.WHITE);
        editText.setHintTextColor(Color.GRAY);
        editText.setHint("Type here…");
        editText.setBackground(null);
        editText.setSingleLine(true);
        editText.setImeOptions(EditorInfo.IME_ACTION_DONE);

        // Forward every keystroke to Rust
        editText.addTextChangedListener(new TextWatcher() {
            String previous = "";

            @Override public void beforeTextChanged(CharSequence s, int st, int c, int a) {
                previous = s.toString();
            }
            @Override public void onTextChanged(CharSequence s, int st, int b, int c) {}

            @Override public void afterTextChanged(Editable s) {
                String current = s.toString();
                if (current.length() > previous.length()) {
                    // Characters were added — send the delta
                    String added = current.substring(previous.length());
                    FloatingInput.onTextChanged(added);
                } else if (current.length() < previous.length()) {
                    // Deletion — send Backspace for each removed char
                    int removed = previous.length() - current.length();
                    for (int i = 0; i < removed; i++) {
                        FloatingInput.onSpecialKey("Backspace");
                    }
                }
                previous = current;
            }
        });

        editText.setOnEditorActionListener((v, actionId, event) -> {
            if (actionId == EditorInfo.IME_ACTION_DONE) {
                FloatingInput.onSpecialKey("Enter");
                hide();
                return true;
            }
            return false;
        });

        int pad = dpToPx(context, 12);
        ((FrameLayout) container).addView(editText,
            new FrameLayout.LayoutParams(
                ViewGroup.LayoutParams.MATCH_PARENT,
                ViewGroup.LayoutParams.WRAP_CONTENT));
        container.setPadding(pad, pad / 2, pad, pad / 2);
    }

    public void show(String existingValue, String hint) {
        if (visible) return;

        WindowManager.LayoutParams params = new WindowManager.LayoutParams(
            WindowManager.LayoutParams.MATCH_PARENT,
            WindowManager.LayoutParams.WRAP_CONTENT,
            WindowManager.LayoutParams.TYPE_APPLICATION_PANEL,
            // FOCUSABLE so the EditText can receive IME input
            // NOT_TOUCH_MODAL so Bevy canvas touches still register
            WindowManager.LayoutParams.FLAG_NOT_TOUCH_MODAL,
            PixelFormat.TRANSLUCENT
        );
        params.gravity = Gravity.BOTTOM;
        // Sits above the soft keyboard; WindowInsets will push it up
        params.softInputMode =
            WindowManager.LayoutParams.SOFT_INPUT_ADJUST_PAN;

        editText.setHint(hint);
        editText.setText(existingValue);
        windowManager.addView(container, params);
        visible = true;

        // Request focus + show IME
        editText.post(() -> {
            editText.requestFocus();
            InputMethodManager imm = (InputMethodManager)
                editText.getContext()
                    .getSystemService(Context.INPUT_METHOD_SERVICE);
            imm.showSoftInput(editText, InputMethodManager.SHOW_IMPLICIT);
        });
    }

    public void hide() {
        if (!visible) return;
        InputMethodManager imm = (InputMethodManager)
            editText.getContext()
                .getSystemService(Context.INPUT_METHOD_SERVICE);
        imm.hideSoftInputFromWindow(editText.getWindowToken(), 0);
        windowManager.removeView(container);
        visible = false;
    }

    private static int dpToPx(Context ctx, int dp) {
        return Math.round(dp * ctx.getResources().getDisplayMetrics().density);
    }
}