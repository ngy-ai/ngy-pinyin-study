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
        // 拼合表里的组合都能拼出来；只有收进了例字的才有四声例字
        let plain = blend
            .map(|b| b.plain.to_string())
            .unwrap_or_else(|| pinyin::syllable(initial, final_));
        let has_audio = (1..=4u8).any(|tone| audio::has_tone(&plain, tone));

        // 固定每行 12 个：窗口再窄也不会把后面的音挤到看不见
        const PER_ROW: usize = 12;

        let initials_rows = pinyin::blend_initials()
            .chunks(PER_ROW)
            .map(|row| {
                h_flex()
                    .w_full()
                    .gap_2()
                    .children(row.iter().map(|item| {
                        let item: &'static str = *item;
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
                    }))
            })
            .collect::<Vec<_>>();

        let finals_rows = finals
            .chunks(PER_ROW)
            .enumerate()
            .map(|(row_index, row)| {
                h_flex()
                    .w_full()
                    .gap_2()
                    .children(row.iter().enumerate().map(|(i, item)| {
                        let item: &'static str = *item;
                        let selected = item == final_;
                        let palette = ui::tint(row_index * PER_ROW + i + 2);
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
                    }))
            })
            .collect::<Vec<_>>();

        let equation = h_flex()
            .gap_4()
            .items_center()
            .child(tile(initial, ui::tint(0), 68., 30.))
            .child(
                div()
                    .text_size(px(26.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(ui::MUTED))
                    .child("+"),
            )
            .child(tile(final_, ui::tint(3), 68., 30.))
            .child(
                div()
                    .text_size(px(26.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(ui::MUTED))
                    .child("="),
            )
            .child({
                let palette = ui::tint(2);
                h_flex()
                    .items_center()
                    .justify_center()
                    .h(px(68.))
                    .min_w(px(150.))
                    .px_5()
                    .rounded_3xl()
                    .bg(rgb(palette.strong))
                    .child(
                        div()
                            .text_size(px(36.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child(plain.clone()),
                    )
            })
            .child(div().flex_1())
            .child(
                Button::new("blend-say")
                    .primary()
                    .icon(IconName::Volume2)
                    .label("连起来读")
                    .on_click(cx.listener({
                        let plain = plain.clone();
                        move |this, _: &ClickEvent, _window, _cx| {
                            if !audio::has_tone(&plain, 1) {
                                return;
                            }
                            // 连起来读就是这一整个音节，用第一声的录音
                            let hanzi = pinyin::find_blend(this.pages.blend.initial, this.pages.blend.final_)
                                .and_then(|blend| blend.tones.first())
                                .map(|(_, hanzi)| *hanzi)
                                .unwrap_or("啊");
                            audio::play_tone(&plain, 1, hanzi);
                        }
                    })),
            );

        // 四声：有例字的音节只显示真实存在的声调，没有例字的组合显示带调拼音
        let tones: Vec<AnyElement> = (1..=4u8)
            .filter_map(|tone| {
                let hanzi = blend.and_then(|blend| {
                    blend
                        .tones
                        .iter()
                        .find(|(pinyin_text, _)| pinyin::tone_number(pinyin_text) == tone)
                        .map(|(_, hanzi)| *hanzi)
                });
                if blend.is_some() && hanzi.is_none() {
                    return None;
                }
                let marked = pinyin::with_tone(&plain, tone);
                let palette = ui::tint((tone - 1) as usize);
                let playable = audio::has_tone(&plain, tone);
                let card = v_flex()
                    .id(format!("tone-{tone}"))
                    .cursor_pointer()
                    .flex_none()
                    .w(px(140.))
                    .items_center()
                    .justify_center()
                    .gap_1()
                    .h(px(92.))
                    .rounded_2xl()
                    .border_1()
                    .border_color(rgb(palette.soft))
                    .bg(rgb(palette.soft))
                    .hover(|style| style.border_color(rgb(palette.strong)))
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(rgb(ui::MUTED))
                            .child(TONE_NAMES[(tone - 1) as usize]),
                    )
                    .child(
                        div()
                            .text_size(px(28.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(palette.strong))
                            .child(marked),
                    )
                    .children(hanzi.map(|hanzi| {
                        div()
                            .text_size(px(16.))
                            .text_color(rgb(ui::INK))
                            .child(hanzi)
                    }));
                if playable {
                    let plain = plain.clone();
                    let fallback = hanzi.unwrap_or("");
                    Some(
                        card.on_click(cx.listener(move |_this, _: &ClickEvent, _window, _cx| {
                            audio::play_tone(&plain, tone, fallback);
                        }))
                        .into_any_element(),
                    )
                } else {
                    Some(card.into_any_element())
                }
            })
            .collect();

        v_flex()
            .id("blend-page")
            .flex_1()
            .w_full()
            .min_h(px(0.))
            .gap_2()
            .child(
                v_flex()
                    .w_full()
                    .gap_2()
                    .child(section("第一步，选一个声母"))
                    .child(v_flex().w_full().gap_2().children(initials_rows)),
            )
            .child(
                v_flex()
                    .w_full()
                    .gap_2()
                    .child(section("第二步，选一个韵母"))
                    .child(v_flex().w_full().gap_2().children(finals_rows)),
            )
            .child(
                ui::card()
                    .w_full()
                    .p_3()
                    .gap_2()
                    .child(equation)
                    .child(div().h(px(1.)).w_full().bg(rgb(ui::LINE)))
                    .child(
                        v_flex()
                            .w_full()
                            .gap_2()
                            .child(section(if has_audio {
                                "点一点，听听四个声调"
                            } else {
                                "这个音节还没有录音和例字，先照着拼音读一读"
                            }))
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
        .min_w(px(48.))
        .h(px(40.))
        .px_2()
        .rounded_2xl()
        .border_1()
        .text_size(px(18.))
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
