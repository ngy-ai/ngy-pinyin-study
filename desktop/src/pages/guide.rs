//! 家长教辅：怎么陪孩子练，以及每个音的发音要领和常见错误。

use crate::{AppView, pinyin, ui};
use gpui_kit::assets::IconName;
use gpui_kit::component::{h_flex, v_flex};
use gpui_kit::*;

impl AppView {
    pub fn render_guide(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let how_to = ui::card().p_6().gap_3().child(section_title(
            IconName::Lightbulb,
            ui::tint(1),
            "怎么陪孩子练",
            "比“教多少”更重要的是“每天坚持一点”",
        ));
        let how_to = pinyin::HOW_TO
            .iter()
            .enumerate()
            .fold(how_to, |card, (i, (title, body))| {
                card.child(
                    h_flex()
                        .items_start()
                        .gap_3()
                        .child(
                            div()
                                .flex_none()
                                .size(px(22.))
                                .rounded_full()
                                .bg(rgb(ui::tint(i).soft))
                                .text_size(px(12.))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(ui::tint(i).strong))
                                .child(format!("{}", i + 1))
                                .flex()
                                .items_center()
                                .justify_center(),
                        )
                        .child(
                            v_flex()
                                .gap_0p5()
                                .child(
                                    div()
                                        .text_size(px(14.))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(ui::INK))
                                        .child(*title),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.))
                                        .line_height(px(20.))
                                        .text_color(rgb(ui::MUTED))
                                        .child(*body),
                                ),
                        ),
                )
            });

        let rhyme = ui::card()
            .p_5()
            .bg(rgb(ui::tint(2).soft))
            .border_color(rgb(ui::tint(2).soft))
            .child(
                h_flex()
                    .items_center()
                    .gap_3()
                    .child(ui::icon_badge(
                        IconName::Puzzle,
                        ui::tint(2).strong,
                        ui::PAPER,
                        40.,
                    ))
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(px(13.))
                                    .text_color(rgb(ui::MUTED))
                                    .child("拼读口诀"),
                            )
                            .child(
                                div()
                                    .text_size(px(19.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(ui::tint(2).strong))
                                    .child(pinyin::BLEND_RHYME),
                            ),
                    ),
            );

        let initials = tip_card(
            IconName::GraduationCap,
            ui::tint(0),
            "声母发音要领",
            "声母要读得又轻又短，呼读音只是帮忙带出声音",
            pinyin::INITIAL_TIPS,
            cx,
        );
        let finals = tip_card(
            IconName::AudioLines,
            ui::tint(3),
            "韵母发音要领",
            "韵母要读得响亮，四声可以用手势带着比",
            pinyin::FINAL_TIPS,
            cx,
        );

        v_flex()
            .flex_1()
            .min_h(px(0.))
            .gap_4()
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .items_center()
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(px(22.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(ui::INK))
                                    .child("家长教辅"),
                            )
                            .child(
                                div()
                                    .text_size(px(13.))
                                    .text_color(rgb(ui::MUTED))
                                    .child("给孩子读之前，先看一眼这里的要领"),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(ui::MUTED))
                            .child("点一下拼音可以听标准音"),
                    ),
            )
            .child({
                let scroll = self.pages.guide_scroll.clone();
                v_flex()
                    .id("guide-scroll")
                    .flex_1()
                    .min_h(px(0.))
                    .gap_4()
                    .overflow_y_scroll()
                    .track_scroll(&scroll)
                    .child(how_to)
                    .child(rhyme)
                    .child(initials)
                    .child(finals)
            })
            .into_any_element()
    }
}

fn section_title(
    icon: IconName,
    palette: &'static ui::Tint,
    title: &'static str,
    subtitle: &'static str,
) -> AnyElement {
    h_flex()
        .gap_3()
        .items_center()
        .child(ui::icon_badge(icon, palette.strong, palette.soft, 40.))
        .child(
            v_flex()
                .gap_0p5()
                .child(
                    div()
                        .text_size(px(17.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(ui::INK))
                        .child(title),
                )
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(rgb(ui::MUTED))
                        .child(subtitle),
                ),
        )
        .into_any_element()
}

/// 一张「音 → 要领 / 易错」的速查卡片。
fn tip_card(
    icon: IconName,
    palette: &'static ui::Tint,
    title: &'static str,
    subtitle: &'static str,
    tips: &'static [pinyin::Tip],
    cx: &mut Context<AppView>,
) -> AnyElement {
    let rows = tips.iter().map(|tip| {
        let chip = h_flex()
            .id(format!("guide-tip-{}", tip.text))
            .flex_none()
            .cursor_pointer()
            .items_center()
            .justify_center()
            .w(px(52.))
            .h(px(34.))
            .rounded_xl()
            .bg(rgb(palette.soft))
            .text_size(px(17.))
            .font_weight(FontWeight::BOLD)
            .text_color(rgb(palette.strong))
            .hover(|style| style.border_1().border_color(rgb(palette.strong)))
            .child(tip.text);
        let chip = if crate::audio::has_letter(tip.text) {
            chip.on_click(cx.listener(move |_this, _: &ClickEvent, _window, _cx| {
                crate::audio::play_letter(tip.text, tip.text);
            }))
        } else {
            chip
        };
        h_flex().items_start().gap_3().py_2().child(chip).child(
            v_flex()
                .flex_1()
                .gap_0p5()
                .child(
                    div()
                        .text_size(px(13.))
                        .line_height(px(19.))
                        .text_color(rgb(ui::INK))
                        .child(tip.mouth),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .items_start()
                        .child(
                            div()
                                .flex_none()
                                .text_size(px(11.))
                                .line_height(px(19.))
                                .text_color(rgb(ui::WRONG))
                                .child("易错"),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .line_height(px(19.))
                                .text_color(rgb(ui::MUTED))
                                .child(tip.warn),
                        ),
                ),
        )
    });

    ui::card()
        .p_6()
        .gap_2()
        .child(section_title(icon, palette, title, subtitle))
        .child(div().h(px(4.)))
        .children(rows)
        .into_any_element()
}
