# 拼音乐园 · 手机端

用 [gpui-mobile](https://github.com/itsbalamurali/gpui-mobile) 把桌面端的三页搬到手机上。
桌面端代码一行没动，两边共用 `src/pinyin.rs` 里的拼音数据。

## 现在的状态

| 功能 | 状态 |
| --- | --- |
| 认一认（声母 / 韵母 / 整体认读、四声、上一个 / 下一个） | ✅ |
| 拼一拼（选声母 + 选韵母、拼读、四声例字） | ✅ |
| 家长教辅（HOW_TO + 拼读口诀） | ✅ |
| 竖屏布局、44px 点按区 | ✅ |
| 声音 | ⚠️ 空实现（见 `src/audio.rs`） |
| Android APK | ✅ 已验证 `arm64-v8a` debug 包 |
| iOS | ⚠️ 入口已写，未在 macOS 上验证 |

## 目录

```
mobile/
  Cargo.toml          # 独立 workspace，不影响桌面端的依赖
  .cargo/config.toml  # RUST_FONTCONFIG_DLOPEN，Android 交叉编译必需
  src/lib.rs          # 模块组织，复用 ../../src/pinyin.rs
  src/app.rs          # 三页界面（只依赖 gpui 的 div API）
  src/theme.rs        # 和桌面端同一套暖色
  src/audio.rs        # 播放接口 + 录音路径规则（待接 MediaPlayer）
  src/entry.rs        # android_main / iOS 回调
  vendor/xattr/       # Android 上 xattr 编不过的补丁
  build-android.ps1   # 一条命令出 APK
  android/            # NativeActivity 的 gradle 工程
```

## 为什么有两个 `gpui`

| 目标 | 依赖 | 作用 |
| --- | --- | --- |
| Android / iOS | `gpui`（Zed git）+ `gpui-mobile` | 真平台 + 渲染 |
| Windows / macOS | `gpui-pre`（crates.io） | 只做类型检查，跑不起来 |

`gpui-mobile` 还没发正式版（crates.io 上的 0.1.0 只有 `momentum`），只能用 git；
它固定要求 Zed 这个 rev 的 `gpui`，两边必须是同一份，否则类型不匹配。

桌面端之所以能用新版本写代码，是因为 `gpui-pre` 的 lib 名也叫 `gpui`，
`app.rs` 里一句 `use gpui::*;` 两边通用。

## 构建

前置：rustup、Android SDK + NDK、JDK 17、gradle。

```powershell
cd mobile
.\build-android.ps1                 # arm64 真机
.\build-android.ps1 -Abi x86_64     # 模拟器
.\build-android.ps1 -Abi all        # 两个架构都打
.\build-android.ps1 -Release        # release
```

脚本会：装对应的 rust target → 用 NDK 的 clang 交叉编译 → strip →
把 `libpinyin_study_mobile.so` 放进 `android/app/src/main/jniLibs/<abi>/` →
`gradlew assembleDebug`。产物在 `android/app/build/outputs/apk/debug/app-debug.apk`。

装到手机：

```powershell
adb install -r android\app\build\outputs\apk\debug\app-debug.apk
```

### 手工命令（脚本里有同样的步骤）

```powershell
$ndk = "E:\dev\AndroidSDK\ndk\28.2.13676358"
$bin = "$ndk\toolchains\llvm\prebuilt\windows-x86_64\bin"
$env:CC_aarch64_linux_android   = "$bin\aarch64-linux-android26-clang.cmd"
$env:AR_aarch64_linux_android   = "$bin\llvm-ar.exe"
$env:CXX_aarch64_linux_android  = "$bin\aarch64-linux-android26-clang++.cmd"
$env:CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER = "$bin\aarch64-linux-android26-clang.cmd"

rustup target add aarch64-linux-android
cargo build --target aarch64-linux-android
```

> 不建议用 `cargo ndk`：它在这个环境下会把 `-lnativewindow` 的 sysroot 路径搞丢，
> 链接阶段报 `unable to find library -lnativewindow`。脚本直接调 NDK 的 clang。

## 在模拟器上

已经验证：APK 能装、能启动，`gpui_mobile` 的平台层正常，创建出 1080×2400 的窗口。

但在本机的模拟器（NVIDIA 显卡直通）上，wgpu 探测 Vulkan 适配器时会 SIGSEGV：

```
gpui_wgpu: Testing adapter: NVIDIA GeForce RTX 3080 (Vulkan)...
F DEBUG: signal 11 (SIGSEGV), code 128 (SI_KERNEL), fault addr 0x0
```

gpui_wgpu 里 backends 写死成 `VULKAN | GL`，没有环境变量可切，所以这是模拟器
GPU 直通的问题，不是界面代码。真机、或把模拟器改成软件渲染
（`emulator -avd <name> -gpu swiftshader_indirect`）应该就正常，麻烦你在这两种环境里验一下。

另外启动日志里这句可以忽略：

```
loadClass(dev.gpui.mobile.GpuiPlatformView) failed
```

那是 gpui-mobile 的原生 View 宿主（WebView 那类），我们只用纯 GPUI 绘制，用不到。

## 踩过的坑

1. **`xattr` 编不过**：`gpui → http_client → async-tar → xattr`，
   Android 的 libc 没有 `ENOATTR`。用 `vendor/xattr` 空实现打补丁，
   我们的界面不解压 tar，没有副作用。
2. **fontconfig 找不到**：`yeslogic-fontconfig-sys` 的 build script 在 Android 上必然失败，
   `.cargo/config.toml` 里设 `RUST_FONTCONFIG_DLOPEN=on`。
3. **Gradle 9 打不了 AGP 8.7**：`android/gradlew` 固定用 8.9，不要删 `gradle/wrapper`。
4. **debug `.so` 有 460MB**：`llvm-strip --strip-unneeded` 后 ~35MB，
   APK 33.8MB；release 包会小很多。

## 下一步

- 接声音：Android 用 JNI 调 `MediaPlayer`（`audio::letter_clip` / `tone_clip` 已经给出资源路径），
  iOS 用 `AVAudioPlayer`；录音资源直接复用桌面端的 `assets/audio`。
- 「玩一玩」页（听音选字）还没做，桌面端有，可以照着 `pages/quiz.rs` 移植。
- 真机验证手势滚动（gpui-mobile 把触摸映射成鼠标事件，`.on_click` 可用）。
