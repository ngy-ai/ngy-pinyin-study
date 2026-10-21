//! 拼音乐园手机端（gpui-mobile）。
//!
//! - 拼音数据来自 `shared/` 的 `pinyin-core`，和桌面端是同一个 crate；
//! - 界面只依赖 `gpui`（手机端那套由 gpui-mobile 提供 Platform 实现）；
//! - Windows / macOS 上编译时 `gpui` 换成 gpui-pre，只做类型检查，不产生可运行窗口。

pub mod app;
pub mod audio;
pub mod insets;
pub mod theme;

// 别名保持 pinyin，界面里的 `pinyin::…` 不用改。
pub use pinyin_core as pinyin;

#[cfg(any(target_os = "android", target_os = "ios"))]
mod entry;
