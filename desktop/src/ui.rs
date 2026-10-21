//! 幼儿友好的配色与一些共用的界面零件。

use gpui_kit::assets::IconName;
use gpui_kit::component::{Icon, Sizable as _, h_flex, theme::Theme, v_flex};
use gpui_kit::*;

/// 页面底色（奶油白）
pub const CREAM: u32 = 0xFFF8EF;
/// 正文墨色
pub const INK: u32 = 0x453A30;
/// 次要文字
pub const MUTED: u32 = 0x9A8B7B;
/// 分割线
pub const LINE: u32 = 0xF1E3D2;
/// 卡片白
pub const PAPER: u32 = 0xFFFFFF;
/// 星星黄
pub const STAR: u32 = 0xF7B32B;
/// 答对绿
pub const OK: u32 = 0x35A46B;
/// 答错红
pub const WRONG: u32 = 0xE5634E;

/// 一组柔和的主色 / 背景色。
pub struct Tint {
    /// 深色（文字、图标）
    pub strong: u32,
    /// 浅色（背景、描边）
    pub soft: u32,
}

pub const PALETTE: &[Tint] = &[
    Tint {
        strong: 0xE2653C,
        soft: 0xFFEDE4,
    }, // 蜜桃
    Tint {
        strong: 0xC08A12,
        soft: 0xFFF6DC,
    }, // 柠檬
    Tint {
        strong: 0x2E9A66,
        soft: 0xE3F6EB,
    }, // 薄荷
    Tint {
        strong: 0x2C7BC0,
        soft: 0xE4F1FC,
    }, // 天空
    Tint {
        strong: 0x6B57C6,
        soft: 0xEEE9FD,
    }, // 葡萄
    Tint {
        strong: 0xC34B84,
        soft: 0xFDE8F1,
    }, // 樱花
    Tint {
        strong: 0x2A9A9A,
        soft: 0xE1F6F5,
    }, // 湖水
    Tint {
        strong: 0xC96B2C,
        soft: 0xFFF0E0,
    }, // 橘子
];

pub fn tint(i: usize) -> &'static Tint {
    &PALETTE[i % PALETTE.len()]
}

/// 白色圆角卡片。
pub fn card() -> Div {
    v_flex()
        .bg(rgb(PAPER))
        .rounded_3xl()
        .border_1()
        .border_color(rgb(LINE))
        .shadow_sm()
}

/// 一个带底色和圆角的图标徽标。
pub fn icon_badge(icon: IconName, fg: u32, bg: u32, size: f32) -> Div {
    h_flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(px(size))
        .rounded(px(size * 0.34))
        .bg(rgb(bg))
        .child(
            Icon::new(icon)
                .with_size(px(size * 0.52))
                .text_color(rgb(fg)),
        )
}

/// 首选一款能显示中文的系统字体，找不到就用 GPUI 的默认字体。
pub fn configure_theme(cx: &mut App) {
    let family = pick_font(cx);
    Theme::update(cx, move |theme| {
        theme.font_family = family;
        theme.font_size = px(16.0);
        theme.radius = px(10.0);
        theme.radius_lg = px(18.0);
    });
}

fn pick_font(cx: &App) -> SharedString {
    const CANDIDATES: &[&str] = &[
        "Microsoft YaHei UI",
        "Microsoft YaHei",
        "微软雅黑",
        "PingFang SC",
        "Hiragino Sans GB",
        "Noto Sans CJK SC",
        "Source Han Sans SC",
        "WenQuanYi Micro Hei",
        "SimHei",
        "SimSun",
    ];

    let installed = cx.text_system().all_font_names();
    for want in CANDIDATES {
        if installed.iter().any(|name| name.eq_ignore_ascii_case(want)) {
            return SharedString::from(*want);
        }
    }
    SharedString::from(".SystemUIFont")
}
