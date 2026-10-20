//! 玩一玩：听一听，从四个带声调的拼音里选出正确的那个。

use crate::{AppView, audio, pinyin, ui};
use gpui_kit::assets::IconName;
use gpui_kit::component::{
    Icon, Sizable as _, button::Button, button::ButtonVariants, h_flex, progress::Progress, v_flex,
};
use gpui_kit::*;

#[derive(Clone, Copy)]
pub struct Question {
    pub blend: &'static pinyin::Blend,
    /// 正确声调在 `blend.tones` 中的下标
    pub tone: usize,
    /// 打乱后的四个选项
    pub options: [&'static str; 4],
    /// 正确选项在 `options` 中的下标
    pub correct: usize,
}

pub struct QuizState {
    pub total: usize,
    /// 当前第几题（从 0 开始）
    pub index: usize,
    pub score: usize,
    pub question: Option<Question>,
    /// 选对后锁定
    pub chosen: Option<usize>,
    /// 最近一次选错的选项，用来标红
    pub wrong: Option<usize>,
    pub finished: bool,
    rng: pinyin::Rng,
}

impl QuizState {
    pub fn new() -> Self {
        let mut state = Self {
            total: 10,
            index: 0,
            score: 0,
            question: None,
            chosen: None,
            wrong: None,
            finished: false,
            rng: pinyin::Rng::from_time(),
        };
        let question = state.pick();
        state.question = question;
        state
    }

    fn pick(&mut self) -> Option<Question> {
        let pool = pinyin::quiz_blends();
        if pool.is_empty() {
            return None;
        }
        let blend = pool[self.rng.below(pool.len())];
        let tone = self.rng.below(4);
        let mut order = [0usize, 1, 2, 3];
        self.rng.shuffle(&mut order);
        let options = [
            blend.tones[order[0]].0,
            blend.tones[order[1]].0,
            blend.tones[order[2]].0,
            blend.tones[order[3]].0,
        ];
        let correct = order.iter().position(|&t| t == tone).unwrap_or(0);
        Some(Question {
            blend,
            tone,
            options,
            correct,
        })
    }

    /// 进入下一题；最后一题答完后进入结算。
    pub fn advance(&mut self) {
        if self.index + 1 >= self.total {
            self.finished = true;
            self.chosen = None;
            self.wrong = None;
            return;
        }
        self.index += 1;
        self.chosen = None;
        self.wrong = None;
        let question = self.pick();
        self.question = question;
    }

    pub fn restart(&mut self) {
        self.index = 0;
        self.score = 0;
        self.chosen = None;
        self.wrong = None;
        self.finished = false;
        let question = self.pick();
        self.question = question;
    }
}

impl AppView {
    pub fn render_quiz(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let quiz = &self.pages.quiz;
        let progress = if quiz.finished {
            100.0
        } else {
            quiz.index as f32 / quiz.total as f32 * 100.0
        };

        let top = h_flex()
            .w_full()
            .gap_4()
            .items_center()
            .child(
                div()
                    .flex_none()
                    .text_size(px(14.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(ui::INK))
                    .child(format!(
                        "第 {} / {} 题",
                        (quiz.index + 1).min(quiz.total),
                        quiz.total
                    )),
            )
            .child(
                div().flex_1().child(
                    Progress::new("quiz-progress")
                        .value(progress)
                        .color(rgb(ui::tint(2).strong))
                        .with_size(px(12.)),
                ),
            )
            .child(
                h_flex()
                    .flex_none()
                    .gap_1p5()
                    .items_center()
                    .child(
                        Icon::new(IconName::Star)
                            .with_size(px(16.))
                            .text_color(rgb(ui::STAR)),
                    )
                    .child(
                        div()
                            .text_size(px(14.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(ui::MUTED))
                            .child(format!("答对 {} 题", quiz.score)),
                    ),
            );

        if quiz.finished {
            return v_flex()
                .flex_1()
                .min_h(px(0.))
                .gap_4()
                .child(top)
                .child(
                    ui::card()
                        .flex_1()
                        .items_center()
                        .justify_center()
                        .p_8()
                        .gap_4()
                        .child(ui::icon_badge(
                            IconName::PartyPopper,
                            ui::tint(1).strong,
                            ui::tint(1).soft,
                            72.,
                        ))
                        .child(
                            div()
                                .text_size(px(30.))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(ui::INK))
                                .child("全部答完啦！"),
                        )
                        .child(
                            div()
                                .text_size(px(16.))
                                .text_color(rgb(ui::MUTED))
                                .child(format!(
                                    "这一轮答对 {} 题，共收获 {} 颗小星星",
                                    quiz.score, self.stars
                                )),
                        )
                        .child(
                            Button::new("quiz-restart")
                                .primary()
                                .label("再玩一次")
                                .icon(IconName::RotateCcw)
                                .on_click(cx.listener(|this, _: &ClickEvent, _window, cx| {
                                    this.pages.quiz.restart();
                                    cx.notify();
                                })),
                        ),
                )
                .into_any_element();
        }

        let Some(question) = quiz.question else {
            return v_flex()
                .child(div().child("还没有准备好题目"))
                .into_any_element();
        };

        let chosen = quiz.chosen;
        let wrong = quiz.wrong;
        let hanzi = question.blend.tones[question.tone].1;
        let pinyin_text = question.blend.tones[question.tone].0;
        let plain = question.blend.plain;
        let tone_no = pinyin::tone_number(pinyin_text).max(1);

        let speaker = {
            let palette = ui::tint(2);
            h_flex()
                .id("quiz-speak")
                .flex_none()
                .cursor_pointer()
                .items_center()
                .justify_center()
                .size(px(84.))
                .rounded_full()
                .bg(rgb(palette.strong))
                .hover(|style| style.opacity(0.88))
                .child(
                    Icon::new(IconName::Volume2)
                        .with_size(px(34.))
                        .text_color(rgb(0xFFFFFF)),
                )
                .on_click(cx.listener(move |_this, _: &ClickEvent, _window, _cx| {
                    audio::play_tone(plain, tone_no, hanzi);
                }))
        };

        let options = h_flex().gap_3().children((0..4).map(|i| {
            let is_correct = i == question.correct;
            let is_chosen = chosen == Some(i);
            let is_wrong = wrong == Some(i);

            let (bg, border, fg) = if is_chosen {
                (0xE3F7EC, ui::OK, 0x1E7A4B)
            } else if is_wrong {
                (0xFCE9E5, ui::WRONG, 0xB03A28)
            } else if chosen.is_some() && is_correct {
                (0xE3F7EC, ui::OK, 0x1E7A4B)
            } else {
                (ui::PAPER, ui::LINE, ui::INK)
            };

            v_flex()
                .id(format!("quiz-option-{i}"))
                .cursor_pointer()
                .flex_1()
                .items_center()
                .justify_center()
                .gap_1()
                .h(px(116.))
                .rounded_3xl()
                .border_2()
                .border_color(rgb(border))
                .bg(rgb(bg))
                .hover(|style| style.border_color(rgb(ui::tint(2).strong)))
                .child(
                    div()
                        .text_size(px(36.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(fg))
                        .child(question.options[i]),
                )
                .children(if is_chosen || (chosen.is_some() && is_correct) {
                    Some(
                        Icon::new(IconName::Check)
                            .with_size(px(16.))
                            .text_color(rgb(ui::OK)),
                    )
                } else if is_wrong {
                    Some(
                        Icon::new(IconName::X)
                            .with_size(px(16.))
                            .text_color(rgb(ui::WRONG)),
                    )
                } else {
                    None
                })
                .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                    let Some(question) = this.pages.quiz.question else {
                        return;
                    };
                    if this.pages.quiz.chosen.is_some() {
                        return;
                    }
                    if i == question.correct {
                        this.pages.quiz.chosen = Some(i);
                        this.pages.quiz.wrong = None;
                        this.pages.quiz.score += 1;
                        this.stars += 1;
                        let praise = pinyin::PRAISE[this.pages.quiz.index % pinyin::PRAISE.len()];
                        audio::speak(praise);
                    } else {
                        this.pages.quiz.wrong = Some(i);
                        audio::speak("再试试");
                    }
                    cx.notify();
                }))
        }));

        let feedback = if let Some(_) = chosen {
            h_flex()
                .gap_3()
                .items_center()
                .child(
                    Icon::new(IconName::CheckCheck)
                        .with_size(px(20.))
                        .text_color(rgb(ui::OK)),
                )
                .child(
                    div()
                        .text_size(px(16.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(ui::OK))
                        .child(format!("读作 {pinyin_text}，真棒！")),
                )
                .child(div().flex_1())
                .child(
                    Button::new("quiz-next")
                        .primary()
                        .label(if quiz.index + 1 >= quiz.total {
                            "看看结果"
                        } else {
                            "下一题"
                        })
                        .icon(IconName::ChevronRight)
                        .on_click(cx.listener(|this, _: &ClickEvent, _window, cx| {
                            this.pages.quiz.advance();
                            if let Some(question) = this.pages.quiz.question {
                                let (marked, hanzi) = question.blend.tones[question.tone];
                                let tone = pinyin::tone_number(marked).max(1);
                                audio::play_tone(question.blend.plain, tone, hanzi);
                            }
                            cx.notify();
                        })),
                )
        } else if wrong.is_some() {
            h_flex()
                .gap_2()
                .items_center()
                .child(
                    Icon::new(IconName::X)
                        .with_size(px(18.))
                        .text_color(rgb(ui::WRONG)),
                )
                .child(
                    div()
                        .text_size(px(15.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(ui::WRONG))
                        .child("差一点点，再听一次吧"),
                )
        } else {
            h_flex().gap_2().items_center().child(
                div()
                    .text_size(px(15.))
                    .text_color(rgb(ui::MUTED))
                    .child("先点一下大喇叭，听清楚声调再选"),
            )
        };

        v_flex()
            .flex_1()
            .min_h(px(0.))
            .gap_4()
            .child(top)
            .child(
                ui::card()
                    .flex_1()
                    .justify_center()
                    .max_h(px(400.))
                    .p_7()
                    .gap_6()
                    .child(
                        h_flex()
                            .gap_5()
                            .items_center()
                            .child(
                                h_flex()
                                    .flex_none()
                                    .items_center()
                                    .justify_center()
                                    .size(px(96.))
                                    .rounded_3xl()
                                    .bg(rgb(ui::tint(1).soft))
                                    .child(
                                        div()
                                            .text_size(px(52.))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(ui::tint(7).strong))
                                            .child(hanzi),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_size(px(20.))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(ui::INK))
                                            .child("听一听，选出正确的拼音"),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(13.))
                                            .text_color(rgb(ui::MUTED))
                                            .child("点一下喇叭可以再听一次"),
                                    ),
                            )
                            .child(div().flex_1())
                            .child(speaker),
                    )
                    .child(options)
                    .child(feedback),
            )
            .into_any_element()
    }
}
