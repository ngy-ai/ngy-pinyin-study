//! 手机端入口：Android 走 NativeActivity 的 `android_main`，iOS 走 UIKit 回调。
//!
//! 平台对象由 gpui-mobile 创建，这里只负责把它交给 GPUI 并开一个全屏窗口。

use gpui::{App, AppContext as _, WindowOptions};

use crate::app::PinyinApp;

/// 手机上没有标题栏，`window_bounds: None` 就是全屏。
fn open_app(cx: &mut App) {
    let opened = cx.open_window(
        WindowOptions {
            window_bounds: None,
            ..Default::default()
        },
        |_, cx| cx.new(|_| PinyinApp::new()),
    );

    if opened.is_err() {
        log::error!("open_window failed");
    }

    cx.activate(true);
}

#[cfg(target_os = "android")]
mod android {
    use super::*;
    use gpui::Application;
    use gpui_mobile::android::jni;

    /// `android-activity` 加载 .so 后在新线程调用，直到应用退出才返回。
    #[unsafe(no_mangle)]
    fn android_main(app: android_activity::AndroidApp) {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Info)
                .with_tag("pinyin-study"),
        );
        jni::install_panic_hook();
        log::info!("android_main: 启动");

        let _platform = jni::init_platform(&app);

        let Some(shared) = jni::shared_platform() else {
            log::error!("shared_platform() 为空，无法启动 GPUI");
            return;
        };

        // run 会一直阻塞在 Android 事件循环上，等 surface 就绪后才回调 open_app。
        Application::with_platform(shared.into_rc()).run(open_app);
    }
}

#[cfg(target_os = "ios")]
mod ios {
    use super::*;
    use gpui_mobile::ios::ffi;

    /// Xcode 里由 `main.m` 调用：注册根视图回调。
    #[unsafe(no_mangle)]
    pub extern "C" fn gpui_ios_register_app() {
        let _ = log::set_logger(&NSLOG);
        log::set_max_level(log::LevelFilter::Info);
        ffi::set_app_callback(Box::new(open_app));
    }

    struct NsLog;

    impl log::Log for NsLog {
        fn enabled(&self, _: &log::Metadata) -> bool {
            true
        }

        fn log(&self, record: &log::Record) {
            eprintln!("[{}] {}", record.level(), record.args());
        }

        fn flush(&self) {}
    }

    static NSLOG: NsLog = NsLog;
}
