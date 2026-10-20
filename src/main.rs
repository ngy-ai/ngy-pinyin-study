//! 拼音乐园 —— 给幼儿用的拼音学习小应用（gpui-kit）。
//!
//! 三个页面：认一认（看卡片学字母）、拼一拼（声母 + 韵母拼读）、玩一玩（听音选拼音）。

mod audio;
mod pages;
mod pinyin;
mod ui;

use gpui_kit::assets::IconName;
use gpui_kit::component::{
    Icon, Sizable as _, WindowExt as _, button::Button, button::ButtonVariants, h_flex,
    switch::Switch, v_flex,
};
use gpui_kit::*;

use pages::AppPages;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Learn,
    Blend,
    Quiz,
    Guide,
}

impl Page {
    fn label(self) -> &'static str {
        match self {
            Page::Learn => "认一认",
            Page::Blend => "拼一拼",
            Page::Quiz => "玩一玩",
            Page::Guide => "家长教辅",
        }
    }

    fn hint(self) -> &'static str {
        match self {
            Page::Learn => "认识声母和韵母",
            Page::Blend => "把声母和韵母拼起来",
            Page::Quiz => "听一听，选一选",
            Page::Guide => "给爸爸妈妈的教法",
        }
    }

    fn icon(self) -> IconName {
        match self {
            Page::Learn => IconName::GraduationCap,
            Page::Blend => IconName::Puzzle,
            Page::Quiz => IconName::Gamepad2,
            Page::Guide => IconName::BookOpenText,
        }
    }

    fn id(self) -> &'static str {
        match self {
            Page::Learn => "nav-learn",
            Page::Blend => "nav-blend",
            Page::Quiz => "nav-quiz",
            Page::Guide => "nav-guide",
        }
    }

    fn tint(self) -> usize {
        match self {
            Page::Learn => 2,
            Page::Blend => 3,
            Page::Quiz => 4,
            Page::Guide => 5,
        }
    }

    const ALL: [Page; 4] = [Page::Learn, Page::Blend, Page::Quiz, Page::Guide];
}

pub struct AppView {
    pub page: Page,
    /// 获得的小星星
    pub stars: u32,
    /// 声音开关
    pub sound_on: bool,
    /// 语速 -10..10
    pub rate: i32,
    /// 认一认 / 拼一拼 / 玩一玩 的状态
    pub pages: AppPages,
}

impl AppView {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            page: Page::Learn,
            stars: 0,
            sound_on: true,
            rate: -1,
            pages: AppPages::new(),
        }
    }

    fn header(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let view = cx.entity();
        let sound_on = self.sound_on;

        h_flex()
            .w_full()
            .flex_none()
            .px_6()
            .pt_5()
            .pb_3()
            .items_center()
            .justify_between()
            .child(
                h_flex()
                    .gap_3()
                    .items_center()
                    .child(ui::icon_badge(
                        IconName::Music,
                        ui::tint(0).strong,
                        ui::tint(0).soft,
                        46.,
                    ))
                    .child(
                        v_flex()
                            .child(
                                div()
                                    .text_size(px(21.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(ui::INK))
                                    .child("拼音乐园"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.))
                                    .text_color(rgb(ui::MUTED))
                                    .child("和小朋友一起学拼音"),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        h_flex()
                            .gap_1p5()
                            .items_center()
                            .px_3()
                            .py_1p5()
                            .rounded_full()
                            .bg(rgb(0xFFF4D8))
                            .child(
                                Icon::new(IconName::Star)
                                    .with_size(px(18.))
                                    .text_color(rgb(ui::STAR)),
                            )
                            .child(
                                div()
                                    .text_size(px(16.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x8A6415))
                                    .child(self.stars.to_string()),
                            ),
                    )
                    .child(
                        self.round_icon_button(
                            "sound-toggle",
                            if sound_on {
                                IconName::Volume2
                            } else {
                                IconName::VolumeX
                            },
                            sound_on,
                            cx.listener(move |this, _: &ClickEvent, _window, cx| {
                                this.sound_on = !this.sound_on;
                                audio::set_enabled(this.sound_on);
                                cx.notify();
                            }),
                        )
                        .into_any_element(),
                    )
                    .child(
                        self.round_icon_button(
                            "open-settings",
                            IconName::Settings,
                            false,
                            cx.listener({
                                let view = view.clone();
                                move |_this, _: &ClickEvent, window, cx| {
                                    AppView::open_settings(window, cx, view.clone());
                                }
                            }),
                        )
                        .into_any_element(),
                    ),
            )
            .into_any_element()
    }

    fn round_icon_button(
        &self,
        id: &'static str,
        icon: IconName,
        active: bool,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Stateful<Div> {
        let (fg, bg) = if active {
            (ui::tint(2).strong, ui::tint(2).soft)
        } else {
            (ui::MUTED, 0xFDF3E6)
        };

        h_flex()
            .id(id)
            .flex_none()
            .cursor_pointer()
            .items_center()
            .justify_center()
            .size(px(44.))
            .rounded_full()
            .bg(rgb(bg))
            .hover(|style| style.bg(rgb(0xF7E8D4)))
            .child(Icon::new(icon).with_size(px(20.)).text_color(rgb(fg)))
            .on_click(handler)
    }

    fn rail(&mut self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .flex_none()
            .w(px(214.))
            .px_4()
            .pb_5()
            .gap_2()
            .children(Page::ALL.into_iter().map(|page| self.nav_item(page, cx)))
            .child(div().flex_1())
            .child(
                v_flex()
                    .gap_1()
                    .p_3()
                    .rounded_2xl()
                    .bg(rgb(0xFFF1DF))
                    .child(
                        h_flex()
                            .gap_1p5()
                            .items_center()
                            .child(
                                Icon::new(IconName::Lightbulb)
                                    .with_size(px(15.))
                                    .text_color(rgb(ui::tint(1).strong)),
                            )
                            .child(
                                div()
                                    .text_size(px(13.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(ui::tint(7).strong))
                                    .child("小提示"),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .line_height(px(18.))
                            .text_color(rgb(ui::MUTED))
                            .child("点一点大卡片，就能听到读音哦"),
                    ),
            )
            .into_any_element()
    }

    fn nav_item(&mut self, page: Page, cx: &mut Context<Self>) -> AnyElement {
        let selected = self.page == page;
        let tint = ui::tint(page.tint());
        let (bg, border, fg) = if selected {
            (tint.soft, tint.soft, tint.strong)
        } else {
            (ui::PAPER, ui::PAPER, ui::INK)
        };

        h_flex()
            .id(page.id())
            .cursor_pointer()
            .items_center()
            .gap_3()
            .px_3()
            .py_2p5()
            .rounded_2xl()
            .border_1()
            .border_color(rgb(border))
            .bg(rgb(bg))
            .hover(|style| style.bg(rgb(0xF8EEE2)))
            .child(ui::icon_badge(
                page.icon(),
                if selected { 0xFFFFFF } else { tint.strong },
                if selected { tint.strong } else { tint.soft },
                38.,
            ))
            .child(
                v_flex()
                    .child(
                        div()
                            .text_size(px(16.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(fg))
                            .child(page.label()),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(rgb(ui::MUTED))
                            .child(page.hint()),
                    ),
            )
            .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                if this.page != page {
                    this.page = page;
                    cx.notify();
                }
            }))
            .into_any_element()
    }

    /// 家长设置：声音开关、朗读语速。
    fn open_settings(window: &mut Window, cx: &mut App, view: Entity<AppView>) {
        window.open_dialog(cx, move |dialog, _window, cx| {
            let sound_on = view.read(cx).sound_on;
            let rate = view.read(cx).rate;

            let speed_button =
                |label: &'static str, value: i32, selected: bool, target: Entity<AppView>| {
                    let mut button = Button::new(format!("speed-{value}")).label(label);
                    if selected {
                        button = button.primary();
                    }
                    button.on_click(move |_, _window, cx| {
                        audio::set_rate(value);
                        target.update(cx, |this, cx| {
                            this.rate = value;
                            cx.notify();
                        });
                    })
                };

            let sound_view = view.clone();
            let slow_view = view.clone();
            let normal_view = view.clone();
            let fast_view = view.clone();

            dialog
                .title("家长设置")
                .w(px(420.))
                .child(
                    v_flex()
                        .gap_4()
                        .pb_2()
                        .child(
                            h_flex()
                                .justify_between()
                                .items_center()
                                .child(
                                    v_flex()
                                        .child(
                                            div()
                                                .text_size(px(14.))
                                                .font_weight(FontWeight::BOLD)
                                                .child("声音"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.))
                                                .text_color(rgb(ui::MUTED))
                                                .child("用系统语音朗读拼音"),
                                        ),
                                )
                                .child(
                                    Switch::new("sound-switch")
                                        .checked(sound_on)
                                        .on_change(move |checked, _window, cx| {
                                            audio::set_enabled(*checked);
                                            sound_view.update(cx, |this, cx| {
                                                this.sound_on = *checked;
                                                cx.notify();
                                            });
                                        }),
                                ),
                        )
                        .child(
                            v_flex()
                                .gap_2()
                                .child(
                                    div()
                                        .text_size(px(14.))
                                        .font_weight(FontWeight::BOLD)
                                        .child("朗读语速"),
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(speed_button("慢一点", -4, rate == -4, slow_view))
                                        .child(speed_button("合适", -1, rate == -1, normal_view))
                                        .child(speed_button("快一点", 3, rate == 3, fast_view)),
                                ),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .line_height(px(18.))
                                .text_color(rgb(ui::MUTED))
                                .child("如果听不到声音，请在 Windows「设置 - 时间和语言 - 语音」中安装中文语音包。"),
                        ),
                )
                .footer(
                    h_flex().w_full().justify_end().child(
                        Button::new("settings-close")
                            .label("好的")
                            .primary()
                            .on_click(move |_, window, cx| {
                                window.close_dialog(cx);
                            }),
                    ),
                )
        });
    }

    fn body(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self.page {
            Page::Learn => self.render_learn(cx),
            Page::Blend => self.render_blend(cx),
            Page::Quiz => self.render_quiz(window, cx),
            Page::Guide => self.render_guide(cx),
        }
    }
}

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let header = self.header(cx);
        let rail = self.rail(cx);
        let body = self.body(window, cx);

        v_flex()
            .size_full()
            .bg(rgb(ui::CREAM))
            .text_color(rgb(ui::INK))
            .child(header)
            .child(
                h_flex()
                    .flex_1()
                    .w_full()
                    .min_h(px(0.))
                    .items_stretch()
                    .child(rail)
                    .child(
                        v_flex()
                            .flex_1()
                            .min_w(px(0.))
                            .min_h(px(0.))
                            .px_6()
                            .pb_6()
                            .overflow_hidden()
                            .child(body),
                    ),
            )
    }
}

fn main() {
    application().with_assets(assets::AllAssets).run(|cx| {
        init(cx);
        ui::configure_theme(cx);
        audio::init();

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1120.), px(700.)),
                cx,
            ))),
            window_min_size: Some(size(px(900.), px(600.))),
            ..Default::default()
        };

        open_window(options, cx, |window, cx| {
            window.set_window_title("拼音乐园");
            cx.new(AppView::new)
        })
        .expect("无法打开窗口");
    });
}
