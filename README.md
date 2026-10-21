**Compilation**
I will list the following targets:
1. Android
2. Web
3. Cross-compilation for Windows on Linux using cargo-xwin


Packages I install before any compilation:

sudo apt install -y ninja-build build-essential pkg-config libssl-dev openssl  libasound2-dev  alsa-base libudev-dev g++ libx11-dev libwayland-dev libxkbcommon-dev  clang lld llvm

# rust installation
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"  
 rustup target add x86_64-pc-windows-msvc

# I guess wasm probably unneeded since handled automatically elsewhere
# Won't hurt to do it here first, though
 rustup target add wasm32-unknown-unknown

# android. For compilation, cargo-ndk will auto-install target, but vscode will complain if I start vscode before I have run ndk at least once, so since I don't necessarily want to do that (usually I open vscode first before using ndk), I guess I will manually call target android here

rustup target add aarch64-linux-android

# Install xwin (if gonna cross compile for windows)
cargo install cargo-xwin






# for web compilation
# --locked because of broken dependencies, idea to use --locked from https://github.com/trunk-rs/trunk/issues/1084#issuecomment-5042578031

cargo install --locked trunk


# previously, trunk build auto-installed wasm-opt, but today it failed due to timeout (manual wget worked fine) but worked after I manually ran the below command to install wasm-opt:
cargo install wasm-opt


# for compiling on android.
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

