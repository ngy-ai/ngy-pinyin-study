# 拼音乐园 · 手机端

用 [gpui-mobile](https://github.com/itsbalamurali/gpui-mobile) 把桌面端的三页搬到手机上。
桌面端代码一行没动，两边共用 `shared/` 里 `pinyin-core` 的拼音数据。

## 现在的状态

| 功能 | 状态 |
| --- | --- |
| 认一认（声母 23 / 韵母 23 / 整体认读 16 三个分类、四声、上一个 / 下一个） | ✅ |
| 拼一拼（选声母 + 选韵母、拼读、四声例字） | ✅ |
| 玩一玩（每轮 10 题，听音选声调，答对计数） | ✅ |
| 家长教辅（HOW_TO + 拼读口诀） | ✅ |
| 竖屏布局、44px 点按区 | ✅ |
| 声音（Android） | ✅ `MediaPlayer`，复用桌面端 343 个录音 |
| 声音（iOS） | ⚠️ 还没接 `AVAudioPlayer`，静音 |
| Android APK | ✅ 已验证 `arm64-v8a` + `x86_64` debug 包 |
| iOS | ⚠️ 入口已写，未在 macOS 上验证 |

## 目录

```
mobile/
  Cargo.toml          # 根 workspace 的成员；Android 依赖与桌面端互不干扰
  src/lib.rs          # 模块组织，拼音数据依赖 ../shared 的 pinyin-core
  src/app.rs          # 三页界面（只依赖 gpui 的 div API）
  src/theme.rs        # 和桌面端同一套暖色
  src/audio.rs        # rust-embed 嵌入录音 + JNI 调 MediaPlayer 播放
  src/entry.rs        # android_main / iOS 回调
  vendor/xattr/       # Android 上 xattr 编不过的补丁
  vendor/gpui-mobile/ # 上游副本，只改了按键分发（音量键）
  build-android.ps1   # 一条命令出 APK
  android/            # NativeActivity 的 gradle 工程
```

## 为什么有两个 `gpui`

| 目标 | 依赖 | 作用 |
| --- | --- | --- |
| Android / iOS | `gpui`（Zed git）+ `gpui-mobile` | 真平台 + 渲染 |
| Windows / macOS | `gpui-pre`（crates.io） | 只做类型检查，跑不起来 |

`gpui-mobile` 还没发正式版（crates.io 上的 0.1.0 只有 `momentum`），只能走源码；
它固定要求 Zed 这个 rev 的 `gpui`，两边必须是同一份，否则类型不匹配。

这份源码放在 `vendor/gpui-mobile/`（上游 checkout `c7cab3a` 的拷贝），因为上游把音量键
当已处理事件吞掉了，必须改它的输入分发，详见下面「踩过的坑」。

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

模拟器（Android 16 / x86_64 / NVIDIA 直通）上跑起来了，三页都验证过，点四声卡片能出声。

### 之前一打开就崩的原因

```
#00 pc ... vulkan.ranchu.so (vk_common_SetDebugUtilsObjectNameEXT+283)
#01 ... (我们的 .so)
```

`wgpu` 在 **debug 构建**下默认打开 `InstanceFlags::DEBUG`，会给每个 Vulkan 对象
调 `vkSetDebugUtilsObjectNameEXT` 起名字；模拟器的 `vulkan.ranchu.so` 在这个函数里
直接段错误 —— 跟界面代码无关。

修法：`Cargo.toml` 里给依赖关掉 debug assertions（release 本来就没有这个 flag）：

```toml
[profile.dev.package."*"]
debug-assertions = false
```

顺带一提：`gpui_wgpu` 把 backends 写死成 `VULKAN | GL`，`WGPU_BACKEND` 之类
环境变量在这里不起作用，所以只能从 debug flag 下手。

### 状态栏

Android 15+ 强制 edge-to-edge，NativeActivity 的 surface 是全屏的，标题会被时钟压住。
`gpui-mobile` 的 `AndroidWindow::safe_area_insets_logical()` 应用层拿不到，
所以启动时用 JNI 读系统的 `status_bar_height` 资源（`src/insets.rs`），
界面根节点加 `.pt()`。

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
4. **debug `.so` 有 460MB**：`llvm-strip --strip-unneeded` 后 ~35–45MB（含嵌入录音），
   单 ABI APK ~39MB、双 ABI ~81MB；release 包会小很多。
5. **`rust-embed` 在 debug 下不嵌入**：默认 debug 构建运行时读磁盘，
   在 Android 上那个 Windows 路径不存在，会一直「没有录音」。必须开 `debug-embed`。
6. **前台时音量键没反应**：NativeActivity 的输入事件由 native 层收尾，`gpui-mobile`
   对所有按键都回 `Handled`，系统就不再调音量（切到桌面就正常）。
   `vendor/gpui-mobile/src/android/jni.rs` 里把 VOLUME_UP/DOWN/MUTE 改成回 `Unhandled`，
   事件交回 Java 层，音量条和 stream 选择都恢复系统默认行为。

## 声音怎么接的

- 录音用 `rust-embed` 从 `../assets/audio` 编进 `.so`，第一次播放时解压到
  `<应用私有目录>/audio-v1/`，文件名把 `/` 换成 `-`（`base/a.mp3` → `base-a.mp3`）。
- 播放走 JNI 调 `android.media.MediaPlayer`（`new → setDataSource → prepare → start`），
  不需要额外的 Java 类，所以 APK 里 `hasCode=false`。
- 播放在独立线程，`getDuration()` + 轮询 25ms 等播完；新请求会把 generation 加一，
  正在播的那条立刻 `stop()` + `release()`，不会两个音叠在一起。
- 界面用 `has_letter` / `has_tone` 判断有没有录音，没有的按钮不标可点。

验证方式（模拟器上跑出来的日志）：

```
MediaPlayerService: onAudioDeviceUpdate deviceIds: 2
AudioTrack: stop(22): called with 24035 frames delivered
```

## 下一步

- iOS 声音：`AVAudioPlayer`，资源路径规则不用改。
- 「玩一玩」页（听音选字）还没做，桌面端有，可以照着 `pages/quiz.rs` 移植。
- 真机验证手势滚动（gpui-mobile 把触摸映射成鼠标事件，`.on_click` 可用）。
