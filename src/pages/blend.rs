//! 拼一拼：声母 + 韵母 → 音节，再听四声和例字。

use crate::{AppView, audio, pinyin, ui};
use gpui_kit::assets::IconName;
use gpui_kit::component::{button::Button, button::ButtonVariants, h_flex, v_flex};
use gpui_kit::*;

pub struct BlendState {
    pub initial: &'static str,
    pub final_: &'static str,
}

impl BlendState {
    pub fn new() -> Self {
        Self {
            initial: "b",
            final_: "a",
        }
    }

    /// 换声母时，如果原来的韵母拼不了，就换成第一个能拼的韵母。
    fn set_initial(&mut self, initial: &'static str) {
        self.initial = initial;
        let finals = pinyin::blend_finals(initial);
        if !finals.contains(&self.final_) {
            self.final_ = finals.first().copied().unwrap_or("a");
        }
    }
}

const TONE_NAMES: [&str; 4] = ["一声", "二声", "三声", "四声"];

impl AppView {
    pub fn render_blend(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let initial = self.pages.blend.initial;
        let final_ = self.pages.blend.final_;
        let finals = pinyin::blend_finals(initial);
        let blend = pinyin::find_blend(initial, final_);

        let initials_row =
            h_flex()
                .flex_wrap()
                .gap_2()
                .children(pinyin::blend_initials().into_iter().map(|item| {
                    let selected = item == initial;
                    let index = pinyin::BLENDS
                        .iter()
                        .position(|b| b.initial == item)
                        .unwrap_or(0);
                    let palette = ui::tint(index);
                    chip(format!("blend-initial-{item}"), item.to_string())
                        .bg(rgb(if selected {
                            palette.strong
                        } else {
                            palette.soft
                        }))
                        .border_color(rgb(if selected {
                            palette.strong
                        } else {
                            palette.soft
                        }))
                        .text_color(rgb(if selected { 0xFFFFFF } else { palette.strong }))
                        .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                            this.pages.blend.set_initial(item);
                            cx.notify();
                        }))
                }));

        let finals_row = h_flex()
            .flex_wrap()
            .gap_2()
            .children(finals.iter().enumerate().map(|(i, item)| {
                let item: &'static str = *item;
                let selected = item == final_;
                let palette = ui::tint(i + 2);
                chip(format!("blend-final-{item}"), item.to_string())
                    .bg(rgb(if selected {
                        palette.strong
                    } else {
                        palette.soft
                    }))
                    .border_color(rgb(if selected {
                        palette.strong
                    } else {
                        palette.soft
                    }))
                    .text_color(rgb(if selected { 0xFFFFFF } else { palette.strong }))
                    .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                        this.pages.blend.final_ = item;
                        cx.notify();
                    }))
            }));

        let equation = h_flex()
            .gap_4()
            .items_center()
            .child(tile(initial, ui::tint(0), 84., 34.))
            .child(
                div()
                    .text_size(px(26.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(ui::MUTED))
                    .child("+"),
            )
            .child(tile(final_, ui::tint(3), 84., 34.))
            .child(
                div()
                    .text_size(px(26.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(ui::MUTED))
                    .child("="),
            )
            .child({
                let plain = blend.map(|b| b.plain).unwrap_or("?");
                let palette = ui::tint(2);
                h_flex()
                    .items_center()
                    .justify_center()
                    .h(px(96.))
                    .min_w(px(150.))
                    .px_5()
                    .rounded_3xl()
                    .bg(rgb(palette.strong))
                    .child(
                        div()
                            .text_size(px(44.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(plain),
                    )
            })
            .child(div().flex_1())
            .child(
                Button::new("blend-say")
                    .primary()
                    .icon(IconName::Volume2)
                    .label("连起来读")
                    .on_click(cx.listener(move |this, _: &ClickEvent, _window, _cx| {
                        let blend =
                            pinyin::find_blend(this.pages.blend.initial, this.pages.blend.final_);
                        if let Some(blend) = blend {
                            // 连起来读就是这一整个音节，用第一声的录音
                            let hanzi = blend.tones.first().map(|(_, h)| *h).unwrap_or("啊");
                            audio::play_tone(blend.plain, 1, hanzi);
                        }
                    })),
            );

        let tones = blend
            .map(|b| b.tones)
            .unwrap_or(&[("?", "?")])
            .iter()
            .enumerate()
            .map(|(i, (pinyin_text, hanzi))| {
                let palette = ui::tint(i);
                let tone = TONE_NAMES.get(i).copied().unwrap_or("");
                v_flex()
                    .id(format!("tone-{i}"))
                    .cursor_pointer()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .gap_1()
                    .h(px(112.))
                    .rounded_2xl()
                    .border_1()
                    .border_color(rgb(palette.soft))
                    .bg(rgb(palette.soft))
                    .hover(|style| style.border_color(rgb(palette.strong)))
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(rgb(ui::MUTED))
                            .child(tone),
                    )
                    .child(
                        div()
                            .text_size(px(32.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(palette.strong))
                            .child(*pinyin_text),
                    )
                    .child(
                        div()
                            .text_size(px(18.))
                            .text_color(rgb(ui::INK))
                            .child(*hanzi),
                    )
                    .on_click(cx.listener(move |_this, _: &ClickEvent, _window, _cx| {
                        if let Some(blend) = blend {
                            let number = pinyin::tone_number(pinyin_text).max(1);
                            audio::play_tone(blend.plain, number, *hanzi);
                        } else {
                            audio::speak(*hanzi);
                        }
                    }))
            });

        v_flex()
            .flex_1()
            .min_h(px(0.))
            .gap_4()
            .child(
                v_flex()
                    .gap_2()
                    .child(section("第一步，选一个声母"))
                    .child(initials_row),
            )
            .child(
                v_flex()
                    .gap_2()
                    .child(section("第二步，选一个韵母"))
                    .child(finals_row),
            )
            .child(
                ui::card()
                    .flex_1()
                    .justify_center()
                    .max_h(px(340.))
                    .p_6()
                    .gap_4()
                    .child(equation)
                    .child(div().h(px(1.)).w_full().bg(rgb(ui::LINE)))
                    .child(
                        v_flex()
                            .gap_2()
                            .child(section("点一点，听听四个声调"))
                            .child(h_flex().gap_3().children(tones)),
                    ),
            )
            .into_any_element()
    }
}

fn section(text: &'static str) -> Div {
    div()
        .text_size(px(13.))
        .font_weight(FontWeight::BOLD)
        .text_color(rgb(ui::MUTED))
        .child(text)
}

fn chip(id: String, text: String) -> Stateful<Div> {
    h_flex()
        .id(id)
        .cursor_pointer()
        .items_center()
        .justify_center()
        .min_w(px(56.))
        .h(px(50.))
        .px_3()
        .rounded_2xl()
        .border_1()
        .text_size(px(20.))
        .font_weight(FontWeight::BOLD)
        .hover(|style| style.opacity(0.85))
        .child(text)
}

fn tile(text: &'static str, palette: &'static ui::Tint, size: f32, font: f32) -> Div {
    h_flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(px(size))
        .rounded_3xl()
        .border_1()
        .border_color(rgb(palette.soft))
        .bg(rgb(palette.soft))
        .child(
            div()
                .text_size(px(font))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(palette.strong))
                .child(text),
        )
}
