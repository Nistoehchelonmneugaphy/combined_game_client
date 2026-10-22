**Compilation**

I will list the following targets:
1. Android
2. Web
3. For Windows on Linux using cargo-xwin


# Stuff to install beforehand:
```bash
sudo apt install -y \
    ninja-build build-essential pkg-config \
    libssl-dev openssl  libasound2-dev \
    alsa-base libudev-dev g++ libx11-dev libwayland-dev \
    libxkbcommon-dev  clang lld llvm

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"

# Windows target  
 rustup target add x86_64-pc-windows-msvc

# I guess wasm probably unneeded since handled automatically elsewhere
# Won't hurt to do it here first, though
 rustup target add wasm32-unknown-unknown

# android. For compilation, cargo-ndk will auto-install target,
# but vscode will complain if I start vscode before I have run ndk at least once (because ndk autoinstalls the target if needed, I guess),
# so since I don't necessarily want to do that (usually I open vscode first before using ndk),
# I guess I will manually call target android here

rustup target add aarch64-linux-android

# Install cargo-xwin (if gonna cross compile for windows)
cargo install cargo-xwin

# for web compilation
# --locked because of broken dependencies, idea to use --locked from https://github.com/trunk-rs/trunk/issues/1084#issuecomment-5042578031
# I asume this was fixed by now, TODO: check if needed

cargo install --locked trunk


# previously, trunk build auto-installed wasm-opt, but today it failed due to timeout (manual wget worked fine) but worked after I manually ran the below command to install wasm-opt:
cargo install wasm-opt


# For compiling on android
# noninteractive because otherwise google-android-platform-tools-installer asks to pick a mirror manually
sudo DEBIAN_FRONTEND=noninteractive apt install -y google-android-platform-tools-installer libgtk-3-dev lldb squashfs-tools openjdk-21-jdk-headless kotlin gradle

cargo install cargo-ndk
# Android SDK


mkdir -p ~/android-sdk/cmdline-tools
cd ~/android-sdk/cmdline-tools
# replace with actual latest, find at https://developer.android.com/studio#command-line-tools-only
# wget https://dl.google.com/android/repository/commandlinetools-linux-14742923_latest.zip
wget https://dl.google.com/android/repository/commandlinetools-linux-15859902_latest.zip
unzip commandlinetools-linux-*.zip
mv cmdline-tools/ latest
rm commandlinetools-linux-*.zip

# tools like adb and fastboot
cd ~/android-sdk/
wget https://dl.google.com/android/repository/platform-tools-latest-linux.zip
unzip platform-tools-latest-linux.zip
rm platform-tools-latest-linux.zip

cat <<'EOF' >> ~/.bashrc

# Android SDK
export ANDROID_SDK_ROOT="$HOME/android-sdk"
export PATH="$ANDROID_SDK_ROOT/cmdline-tools/latest/bin:$PATH"
export PATH="$ANDROID_SDK_ROOT/platform-tools:$PATH"
EOF

source ~/.bashrc 
yes | sdkmanager --licenses

# find latest: sdkmanager --list | grep "build-tools"
sdkmanager "build-tools;37.0.0"
# find latest: sdkmanager --list | grep "platforms"
sdkmanager "platforms;android-37.0"

# find latest using sdkmanager --list | grep "ndk"
# previous used: 30.0.14904198
sdkmanager "ndk;30.0.15729638"
 
#append the latest version ndk
cat <<'EOF' >> ~/.bashrc 
# replace with actual latest ndk installed
export ANDROID_NDK_ROOT="$ANDROID_SDK_ROOT/ndk/30.0.15729638"
EOF

sudo apt install  zip -y
# Not sure if this is needed
curl -s "https://get.sdkman.io" | bash 
source "$HOME/.sdkman/bin/sdkman-init.sh" 
sdk install gradle 



source ~/.bashrc
echo "Installed sdk and ndk and added to path, and also installed sdkman"
```

# 1. Android compilation 
```bash
# first step
 cargo ndk -P 26 -t arm64-v8a build -o /home/user1/combined_game_client/workspace_crates/main/android_build/app/src/main/jniLibs -p main --release

# second is to clean if needed
/home/user1/combined_game_client/workspace_crates/main/android_build/gradlew clean --project-dir /home/user1/combined_game_client/workspace_crates/main/android_build

# assemble apk
/home/user1/combined_game_client/workspace_crates/main/android_build/gradlew assembleDebug --project-dir /home/user1/combined_game_client/workspace_crates/main/android_build

# compilation complete.
# The resulting apk is located at /home/user1/combined_game_client/workspace_crates/main/android_build/app/build/outputs/apk/debug/app-debug.apk
```

Installing the apk:

If using a remote machine, you can transfer the apk to a physical phone however you want, like using scp, 
one way is to use remote adb:
Start adb on local machine: 
```bash
adb devices
```
(starts if not started and shows connected phones)

Then connect to remote:
```bash
ssh  -R 5037:localhost:5037 vps_dev
```

In another termibal tab on the local machine check that the list is same as on remote when using adb devices
So, now all adb commands work the same on remote and local.

```bash
# push install apk using adb to phone
adb install -r /home/user1/combined_game_client/workspace_crates/main/android_build/app/build/outputs/apk/debug/app-debug.apk
```

# 2. Web
```bash
# to unpack assets.zip/assets/* into assets/
unzip -o assets.zip -d .
```
```bash
trunk build --release
# when dev for convenience
trunk serve --address 0.0.0.0 --port 3456 --release
``` 
Reminder: the auth server needs this header so the web app running in the browser can contact it. 
At least using testing. I assume in production a more granular header is needed? Or public facing auth servers can use this?

```nginx
add_header 'Access-Control-Allow-Origin' '*';
```


# 3. Windows on Linux using cargo-xwin

If you are running Rust on a Linux machine, you can compile a native Windows executable using cargo-xwin:

```bash
cargo xwin build --target x86_64-pc-windows-msvc --release
```

The first time cargo-xwin is ran, it will install Windows SDK stuff, which may take a while. If it hangs for a long time on downloading/unpacking something, try restarting the command.
