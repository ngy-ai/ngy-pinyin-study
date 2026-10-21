//! 拼音乐园手机端（gpui-mobile）。
//!
//! - 拼音数据直接复用桌面端的 `src/pinyin.rs`，不复制一份；
//! - 界面只依赖 `gpui`（手机端那套由 gpui-mobile 提供 Platform 实现）；
//! - Windows / macOS 上编译时 `gpui` 换成 gpui-pre，只做类型检查，不产生可运行窗口。

pub mod app;
pub mod audio;
pub mod theme;

#[path = "../../src/pinyin.rs"]
pub mod pinyin;

#[cfg(any(target_os = "android", target_os = "ios"))]
mod entry;
