//! 认一认：一张一张卡片地认识声母、韵母和整体认读音节。

use crate::{AppView, audio, pinyin, ui};
use gpui_kit::assets::IconName;
use gpui_kit::component::{button::Button, button::ButtonVariants, h_flex, v_flex};
use gpui_kit::*;

pub struct LearnState {
    /// 当前分类（声母 / 韵母 / 整体认读）
    pub tab: usize,
    /// 当前卡片
    pub index: usize,
}

impl LearnState {
    pub fn new() -> Self {
        Self { tab: 0, index: 0 }
    }
}

/// 播放一个字母的标准读音；没有录音时退回朗读呼读音或例词。
fn speak(letter: &pinyin::Letter) {
    let fallback = if letter.reading.is_empty() {
        letter.word
    } else {
        letter.reading
    };
    audio::play_letter(letter.text, fallback);
}

/// 四声的名字。
const TONE_NAMES: [&str; 4] = ["一声", "二声", "三声", "四声"];

/// 卡片下方的四声练习：四个带调的音节，点一下听标准录音。
fn tone_practice(
    tab: usize,
    letter: &pinyin::Letter,
    palette: &'static ui::Tint,
    cx: &mut Context<AppView>,
) -> Option<AnyElement> {
    // 显示永远写韵母本身；录音用同音的音节（i 的录音其实是 yī、ui 是 wēi）
    let demo = pinyin::tone_demo(tab, letter.text)?;
    let shown = letter.text;

    Some(
        h_flex()
            .gap_2()
            .items_center()
            .child(
                div()
                    .flex_none()
                    .text_size(px(13.))
                    .text_color(rgb(ui::MUTED))
                    .child("四声"),
            )
            .children((1..=4u8).map(|tone| {
                let marked = pinyin::with_tone(shown, tone);
                let label = TONE_NAMES[(tone - 1) as usize];
                let playable = audio::has_tone(demo, tone);
                let chip = v_flex()
                    .id(format!("tone-{tone}"))
                    .cursor_pointer()
                    .items_center()
                    .justify_center()
                    .gap_0p5()
                    .min_w(px(72.))
                    .px_3()
                    .py_1p5()
                    .rounded_xl()
                    .bg(rgb(palette.soft))
                    .hover(|style| style.border_1().border_color(rgb(palette.strong)))
                    .child(
                        div()
                            .text_size(px(22.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(if playable { palette.strong } else { ui::MUTED }))
                            .child(marked.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(rgb(ui::MUTED))
                            .child(label),
                    );
                if playable {
                    chip.on_click(cx.listener(move |_this, _: &ClickEvent, _window, _cx| {
                        audio::play_tone(demo, tone, pinyin::with_tone(demo, tone));
                    }))
                } else {
                    chip
                }
            }))
            .into_any_element(),
    )
}

impl AppView {
    pub fn render_learn(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let tab = self.pages.learn.tab;
        let (tab_name, letters) = pinyin::TABS[tab];
        let count = letters.len();
        let index = self.pages.learn.index.min(count.saturating_sub(1));
        let letter = &letters[index];
        let tint = ui::tint(index);

        let tabs = h_flex()
            .gap_2()
            .children(pinyin::TABS.iter().enumerate().map(|(i, (name, _))| {
                let selected = i == tab;
                let palette = ui::tint(i + 1);
                h_flex()
                    .id(format!("learn-tab-{i}"))
                    .cursor_pointer()
                    .items_center()
                    .justify_center()
                    .px_4()
                    .py_2()
                    .rounded_full()
                    .border_1()
                    .border_color(rgb(if selected { palette.strong } else { ui::LINE }))
                    .bg(rgb(if selected { palette.strong } else { ui::PAPER }))
                    .text_color(rgb(if selected { 0xFFFFFF } else { ui::MUTED }))
                    .text_size(px(14.))
                    .font_weight(FontWeight::BOLD)
                    .child(*name)
                    .hover(|style| style.opacity(0.85))
                    .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                        if this.pages.learn.tab != i {
                            this.pages.learn.tab = i;
                            this.pages.learn.index = 0;
                            speak(&pinyin::TABS[i].1[0]);
                            cx.notify();
                        }
                    }))
            }));

        let card = ui::card()
            .flex_1()
            .justify_center()
            .p_6()
            .gap_5()
            .child(
                h_flex()
                    .items_center()
                    .gap_6()
                    .child(
                        h_flex()
                            .id("learn-big-letter")
                            .cursor_pointer()
                            .items_center()
                            .gap_5()
                            .rounded_2xl()
                            .child(
                                div()
                                    .min_w(px(108.))
                                    .text_size(px(104.))
                                    .line_height(px(112.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(tint.strong))
                                    .child(letter.text),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_size(px(13.))
                                            .text_color(rgb(ui::MUTED))
                                            .child("读作"),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(22.))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(ui::INK))
                                            .child(if letter.reading.is_empty() {
                                                letter.word
                                            } else {
                                                letter.reading
                                            }),
                                    ),
                            )
                            .on_click(cx.listener(move |_this, _: &ClickEvent, _window, _cx| {
                                speak(&pinyin::TABS[tab].1[index]);
                            })),
                    )
                    .child(div().w(px(1.)).h(px(64.)).bg(rgb(ui::LINE)))
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(px(13.))
                                    .text_color(rgb(ui::MUTED))
                                    .child("例词"),
                            )
                            .child(
                                div()
                                    .text_size(px(32.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(ui::INK))
                                    .child(letter.word),
                            )
                            .child(
                                div()
                                    .text_size(px(15.))
                                    .text_color(rgb(tint.strong))
                                    .child(letter.word_pinyin),
                            ),
                    )
                    .child(div().flex_1())
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .rounded_full()
                            .bg(rgb(tint.soft))
                            .text_size(px(13.))
                            .text_color(rgb(tint.strong))
                            .child(format!("{} / {}", index + 1, count)),
                    ),
            )
            .child(
                h_flex()
                    .gap_3()
                    .items_center()
                    .child(
                        Button::new("learn-say")
                            .primary()
                            .label("读一读")
                            .icon(IconName::Volume2)
                            .on_click(cx.listener(|this, _: &ClickEvent, _window, _cx| {
                                let tab = this.pages.learn.tab;
                                let index = this.pages.learn.index;
                                speak(&pinyin::TABS[tab].1[index]);
                            })),
                    )
                    .child(
                        Button::new("learn-prev")
                            .ghost()
                            .icon(IconName::ChevronLeft)
                            .label("上一个")
                            .on_click(cx.listener(|this, _: &ClickEvent, _window, cx| {
                                this.step_letter(-1);
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("learn-next")
                            .label("下一个")
                            .icon(IconName::ChevronRight)
                            .on_click(cx.listener(|this, _: &ClickEvent, _window, cx| {
                                this.step_letter(1);
                                cx.notify();
                            })),
                    ),
            )
            .children(tone_practice(tab, letter, tint, cx))
            .children((tab == 2).then(|| {
                h_flex()
                    .gap_2()
                    .items_start()
                    .pt_3()
                    .border_t_1()
                    .border_color(rgb(ui::LINE))
                    .child(
                        div()
                            .flex_none()
                            .text_size(px(12.))
                            .text_color(rgb(ui::MUTED))
                            .child("小贴士"),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(rgb(ui::MUTED))
                            .child("整体认读要一口气读出来，不用把声母韵母分开拼。"),
                    )
            }))
            .children(pinyin::tip(tab, letter.text).map(|tip| {
                h_flex()
                    .gap_2()
                    .items_start()
                    .pt_3()
                    .border_t_1()
                    .border_color(rgb(ui::LINE))
                    .child(
                        div()
                            .flex_none()
                            .text_size(px(12.))
                            .text_color(rgb(ui::MUTED))
                            .child("发音要领"),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(rgb(ui::MUTED))
                            .child(tip.mouth),
                    )
            }));

        let chips = h_flex()
            .flex_wrap()
            .gap_2()
            .children(letters.iter().enumerate().map(|(i, item)| {
                let selected = i == index;
                let palette = ui::tint(i);
                h_flex()
                    .id(format!("learn-chip-{tab}-{i}"))
                    .cursor_pointer()
                    .items_center()
                    .justify_center()
                    .min_w(px(62.))
                    .h(px(54.))
                    .px_3()
                    .rounded_2xl()
                    .border_1()
                    .border_color(rgb(if selected {
                        palette.strong
                    } else {
                        palette.soft
                    }))
                    .bg(rgb(if selected {
                        palette.strong
                    } else {
                        palette.soft
                    }))
                    .text_size(px(21.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(if selected { 0xFFFFFF } else { palette.strong }))
                    .child(item.text)
                    .hover(|style| style.opacity(0.85))
                    .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                        this.pages.learn.index = i;
                        speak(&pinyin::TABS[tab].1[i]);
                        cx.notify();
                    }))
            }));

        v_flex()
            .flex_1()
            .min_h(px(0.))
            .gap_4()
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .items_center()
                    .child(tabs)
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(rgb(ui::MUTED))
                            .child(format!("共 {count} 个{tab_name}")),
                    ),
            )
            .child(card)
            .child(
                v_flex()
                    .flex_none()
                    .gap_2()
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(rgb(ui::MUTED))
                            .child("点一点，换个字母"),
                    )
                    .child(chips),
            )
            .into_any_element()
    }

    /// 前后切换卡片并朗读。
    fn step_letter(&mut self, delta: i32) {
        let tab = self.pages.learn.tab;
        let letters = pinyin::TABS[tab].1;
        let count = letters.len() as i32;
        let index = self.pages.learn.index as i32;
        let next = (index + delta).rem_euclid(count);
        self.pages.learn.index = next as usize;
        speak(&letters[next as usize]);
    }
}
