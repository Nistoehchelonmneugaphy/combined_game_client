// MainActivity.java
package org.cgc.main;

import android.os.*;
import android.view.*;
import com.google.androidgamesdk.GameActivity;

public class MainActivity extends GameActivity {
    static {
        System.loadLibrary("client_app_rust_lib");
    }

    private FloatingInput floatingInput;
    private Handler mainHandler;
    public static MainActivity instance;
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        instance = this;
        mainHandler = new Handler(Looper.getMainLooper());
        floatingInput = new FloatingInput(this, getWindowManager());
    }
    public static MainActivity getInstance() {
    return instance;
    }
    // Called from Rust via JNI
    public void showFloatingInput(String existingValue, String hint) {
        mainHandler.post(() -> floatingInput.show(existingValue, hint));
    }

    // Called from Rust via JNI
    public void hideFloatingInput() {
        mainHandler.post(() -> floatingInput.hide());
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);
        if (hasFocus) hideSystemUi();
    }

    private void hideSystemUi() {
        getWindow().getDecorView().setSystemUiVisibility(
            View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
            | View.SYSTEM_UI_FLAG_LAYOUT_STABLE
            | View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
            | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
            | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
            | View.SYSTEM_UI_FLAG_FULLSCREEN
        );
    }
}