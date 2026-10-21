//! 手机端界面：认一认、拼一拼、玩一玩、家长教辅。
//!
//! 布局按竖屏手机来：一屏一个主任务，点按目标 ≥ 44px，尽量少滚动。

// 手机端这份 `gpui` 由 gpui-mobile 提供；桌面端 `gpui` 来自 gpui-pre
// （包名不同，但 lib 名都叫 gpui），所以这里的 import 两边通用。
use gpui::*;

use crate::{audio, pinyin, theme};

const TONE_NAMES: [&str; 4] = ["一声", "二声", "三声", "四声"];
const TAB_NAMES: [&str; 4] = ["认一认", "拼一拼", "玩一玩", "家长"];

pub struct PinyinApp {
    /// 0 = 认一认，1 = 拼一拼，2 = 玩一玩，3 = 家长教辅
    tab: usize,
    /// 认一认里的分类：声母 / 韵母 / 整体认读
    learn_tab: usize,
    learn_index: usize,
    blend_initial: &'static str,
    blend_final: &'static str,
    quiz: QuizState,
    scroll: ScrollHandle,
}

impl PinyinApp {
    pub fn new() -> Self {
        Self {
            tab: 0,
            learn_tab: 0,
            learn_index: 0,
            blend_initial: "b",
            blend_final: "a",
            quiz: QuizState::new(),
            scroll: ScrollHandle::new(),
        }
    }
}

// ── 玩一玩 ──────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct Question {
    blend: &'static pinyin::Blend,
    /// 正确声调在 `blend.tones` 里的下标
    tone: usize,
    /// 打乱后的四个选项（同一个音节的四个声调）
    options: [&'static str; 4],
    correct: usize,
}

struct QuizState {
    total: usize,
    index: usize,
    score: usize,
    question: Option<Question>,
    chosen: Option<usize>,
    wrong: Option<usize>,
    finished: bool,
    rng: pinyin::Rng,
}

impl QuizState {
    fn new() -> Self {
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
        state.question = state.pick();
        state
    }

    fn pick(&mut self) -> Option<Question> {
        let pool = pinyin::quiz_blends();
        if pool.is_empty() {
            return None;
        }
        let blend = pool[self.rng.below(pool.len())];
        let tone = self.rng.below(blend.tones.len());
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

    fn advance(&mut self) {
        if self.index + 1 >= self.total {
            self.finished = true;
            self.chosen = None;
            self.wrong = None;
            return;
        }
        self.index += 1;
        self.chosen = None;
        self.wrong = None;
        self.question = self.pick();
    }

    fn restart(&mut self) {
        self.index = 0;
        self.score = 0;
        self.chosen = None;
        self.wrong = None;
        self.finished = false;
        self.question = self.pick();
    }

    /// 当前题目要播的音：(音节, 声调, 汉字)
    fn audio(&self) -> Option<(&'static str, u8, &'static str)> {
        let question = self.question?;
        let (marked, hanzi) = question.blend.tones[question.tone];
        Some((question.blend.plain, pinyin::tone_number(marked).max(1), hanzi))
    }
}

impl Render for PinyinApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        col()
            .size_full()
            .bg(rgb(theme::BG))
            .px(px(16.))
            // 状态栏高度 + 一点呼吸空间
            .pt(px(crate::insets::top() + 12.))
            .pb(px(10.))
            .gap(px(12.))
            .child(self.header())
            .child(self.tab_bar(cx))
            .child(match self.tab {
                0 => self.learn(cx),
                1 => self.blend(cx),
                2 => self.quiz(cx),
                _ => self.guide(),
            })
    }
}

impl PinyinApp {
    fn header(&self) -> AnyElement {
        row()
            .items_center()
            .gap(px(12.))
            .child(
                div()
                    .size(px(40.))
                    .rounded_full()
                    .bg(rgb(theme::soft(0)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(20.))
                    .child("♪"),
            )
            .child(
                col()
                    .gap(px(2.))
                    .child(
                        div()
                            .text_size(px(20.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(theme::INK))
                            .child("拼音乐园"),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(theme::MUTED))
                            .child("和小朋友一起学拼音"),
                    ),
            )
            .child(div().flex_1())
            .into_any_element()
    }

    fn tab_bar(&self, cx: &mut Context<Self>) -> AnyElement {
        let mut bar = row().gap(px(8.));
        for (i, name) in TAB_NAMES.iter().enumerate() {
            let selected = i == self.tab;
            bar = bar.child(
                div()
                    .id(format!("tab-{i}"))
                    .flex_1()
                    .h(px(44.))
                    .rounded_full()
                    .border_1()
                    .border_color(rgb(if selected {
                        theme::accent(1)
                    } else {
                        theme::LINE
                    }))
                    .bg(rgb(if selected {
                        theme::accent(1)
                    } else {
                        theme::PAPER
                    }))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(15.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(if selected { 0xFFFFFF } else { theme::MUTED }))
                    .child(*name)
                    .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                        this.tab = i;
                        // 进玩一玩就直接把题目念出来，不用先找喇叭
                        if i == 2
                            && let Some((plain, tone, _)) = this.quiz.audio()
                        {
                            audio::play_tone(plain, tone);
                        }
                        cx.notify();
                    })),
            );
        }
        bar.into_any_element()
    }

    // ── 认一认 ──────────────────────────────────────────────────────────────

    fn learn(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let tab = self.learn_tab;
        let (_, letters) = pinyin::TABS[tab];
        let count = letters.len();
        if self.learn_index >= count {
            self.learn_index = 0;
        }
        let index = self.learn_index;
        let letter = &letters[index];
        let tint = theme::accent(index);

        let mut practice = row().gap(px(8.));
        if let Some(demo) = pinyin::tone_demo(tab, letter.text) {
            for tone in 1..=4u8 {
                let labeled = pinyin::with_tone(letter.text, tone);
                let playable = audio::has_tone(demo, tone);
                practice = practice.child(
                    col()
                        .id(format!("learn-tone-{tone}"))
                        .flex_1()
                        .h(px(54.))
                        .rounded_xl()
                        .bg(rgb(theme::soft(3)))
                        .items_center()
                        .justify_center()
                        .gap(px(2.))
                        .child(
                            div()
                                .text_size(px(18.))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(if playable {
                                    theme::accent(3)
                                } else {
                                    theme::MUTED
                                }))
                                .child(labeled),
                        )
                        .child(
                            div()
                                .text_size(px(10.))
                                .text_color(rgb(theme::MUTED))
                                .child(TONE_NAMES[(tone - 1) as usize]),
                        )
                        .on_click(cx.listener(move |_this, _: &ClickEvent, _window, _cx| {
                            audio::play_tone(demo, tone);
                        })),
                );
            }
        }
        let has_practice = pinyin::tone_demo(tab, letter.text).is_some();

        let reading = if letter.reading.is_empty() {
            letter.word
        } else {
            letter.reading
        };

        let card = col()
            .bg(rgb(theme::PAPER))
            .rounded_2xl()
            .p(px(18.))
            .gap(px(12.))
            .child(
                row()
                    .items_center()
                    .gap(px(16.))
                    .child(
                        div()
                            .min_w(px(84.))
                            .text_size(px(64.))
                            .line_height(px(72.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(tint))
                            .child(letter.text),
                    )
                    .child(
                        col()
                            .gap(px(2.))
                            .child(
                                div()
                                    .text_size(px(11.))
                                    .text_color(rgb(theme::MUTED))
                                    .child("读作"),
                            )
                            .child(
                                div()
                                    .text_size(px(20.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(theme::INK))
                                    .child(reading),
                            ),
                    ),
            )
            .child(
                row()
                    .items_center()
                    .gap(px(12.))
                    .pt(px(10.))
                    .border_t_1()
                    .border_color(rgb(theme::LINE))
                    .child(
                        col()
                            .gap(px(2.))
                            .child(
                                div()
                                    .text_size(px(11.))
                                    .text_color(rgb(theme::MUTED))
                                    .child("例词"),
                            )
                            .child(
                                div()
                                    .text_size(px(22.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(theme::INK))
                                    .child(letter.word),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(rgb(tint))
                            .child(letter.word_pinyin),
                    )
                    .child(div().flex_1())
                    .child(
                        div()
                            .px(px(10.))
                            .py(px(4.))
                            .rounded_full()
                            .bg(rgb(theme::soft(index)))
                            .text_size(px(12.))
                            .text_color(rgb(tint))
                            .child(format!("{} / {}", index + 1, count)),
                    ),
            )
            .children(has_practice.then(|| practice.into_any_element()));

        let nav = row()
            .gap(px(10.))
            .child(
                div()
                    .id("learn-play")
                    .flex_1()
                    .h(px(48.))
                    .rounded_xl()
                    .bg(rgb(tint))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(15.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child("读一读")
                    .on_click(cx.listener(|this, _: &ClickEvent, _window, _cx| {
                        let (_, letters) = pinyin::TABS[this.learn_tab];
                        if let Some(letter) = letters.get(this.learn_index) {
                            audio::play_letter(letter.text);
                        }
                    })),
            )
            .child(
                div()
                    .id("learn-prev")
                    .px(px(16.))
                    .h(px(48.))
                    .rounded_xl()
                    .bg(rgb(theme::PAPER))
                    .border_1()
                    .border_color(rgb(theme::LINE))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(14.))
                    .text_color(rgb(theme::MUTED))
                    .child("上一个")
                    .on_click(cx.listener(|this, _: &ClickEvent, _window, cx| {
                        let (_, letters) = pinyin::TABS[this.learn_tab];
                        let count = letters.len() as i32;
                        this.learn_index = ((this.learn_index as i32 - 1).rem_euclid(count)) as usize;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .id("learn-next")
                    .px(px(16.))
                    .h(px(48.))
                    .rounded_xl()
                    .bg(rgb(theme::PAPER))
                    .border_1()
                    .border_color(rgb(theme::LINE))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(14.))
                    .text_color(rgb(theme::MUTED))
                    .child("下一个")
                    .on_click(cx.listener(|this, _: &ClickEvent, _window, cx| {
                        let (_, letters) = pinyin::TABS[this.learn_tab];
                        let count = letters.len() as i32;
                        this.learn_index = ((this.learn_index as i32 + 1).rem_euclid(count)) as usize;
                        cx.notify();
                    })),
            );

        let mut chips = row().flex_wrap().gap(px(8.));
        for (i, item) in letters.iter().enumerate() {
            let selected = i == index;
            let palette = theme::accent(i);
            chips = chips.child(
                div()
                    .id(format!("letter-{i}"))
                    .min_w(px(52.))
                    .h(px(44.))
                    .px(px(10.))
                    .rounded_xl()
                    .border_1()
                    .border_color(rgb(if selected {
                        palette
                    } else {
                        theme::soft(i)
                    }))
                    .bg(rgb(if selected {
                        palette
                    } else {
                        theme::soft(i)
                    }))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(18.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(if selected { 0xFFFFFF } else { palette }))
                    .child(item.text)
                    .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                        this.learn_index = i;
                        let (_, letters) = pinyin::TABS[this.learn_tab];
                        if let Some(letter) = letters.get(i) {
                            audio::play_letter(letter.text);
                        }
                        cx.notify();
                    })),
            );
        }

        let mut kinds = row().gap(px(8.));
        for (i, (name, letters)) in pinyin::TABS.iter().enumerate() {
            let selected = i == tab;
            let palette = theme::accent(1);
            kinds = kinds.child(
                div()
                    .id(format!("learn-kind-{i}"))
                    .flex_1()
                    .h(px(38.))
                    .rounded_full()
                    .border_1()
                    .border_color(rgb(if selected {
                        palette
                    } else {
                        theme::LINE
                    }))
                    .bg(rgb(if selected {
                        palette
                    } else {
                        theme::PAPER
                    }))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(13.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(if selected { 0xFFFFFF } else { theme::MUTED }))
                    .child(format!("{name} {}", letters.len()))
                    .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                        this.learn_tab = i;
                        this.learn_index = 0;
                        this.scroll.set_offset(point(px(0.), px(0.)));
                        if let Some(letter) = pinyin::TABS[i].1.first() {
                            audio::play_letter(letter.text);
                        }
                        cx.notify();
                    })),
            );
        }

        col()
            .id("learn-scroll")
            .flex_1()
            .min_h(px(0.))
            .gap(px(12.))
            .overflow_y_scroll()
            .track_scroll(&self.scroll)
            .child(kinds)
            .child(card)
            .child(nav)
            .child(chips)
            .into_any_element()
    }

    // ── 拼一拼 ──────────────────────────────────────────────────────────────

    fn blend(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let initial = self.blend_initial;
        let finals = pinyin::blend_finals(initial);
        if !finals.contains(&self.blend_final)
            && let Some(first) = finals.first()
        {
            self.blend_final = *first;
        }
        let final_ = self.blend_final;
        let blend = pinyin::find_blend(initial, final_);
        let plain = blend
            .map(|b| b.plain.to_string())
            .unwrap_or_else(|| pinyin::syllable(initial, final_));
        let has_audio = (1..=4u8).any(|tone| audio::has_tone(&plain, tone));

        let mut initial_chips = row().flex_wrap().gap(px(8.));
        for item in pinyin::blend_initials() {
            let selected = item == initial;
            let palette = theme::accent(2);
            initial_chips = initial_chips.child(
                div()
                    .id(format!("initial-{item}"))
                    .min_w(px(46.))
                    .h(px(44.))
                    .px(px(8.))
                    .rounded_xl()
                    .bg(rgb(if selected {
                        palette
                    } else {
                        theme::soft(2)
                    }))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(18.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(if selected { 0xFFFFFF } else { palette }))
                    .child(item)
                    .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                        this.blend_initial = item;
                        let list = pinyin::blend_finals(item);
                        if !list.contains(&this.blend_final)
                            && let Some(first) = list.first()
                        {
                            this.blend_final = *first;
                        }
                        cx.notify();
                    })),
            );
        }

        let mut final_chips = row().flex_wrap().gap(px(8.));
        for &item in &finals {
            let selected = item == final_;
            let palette = theme::accent(0);
            final_chips = final_chips.child(
                div()
                    .id(format!("final-{item}"))
                    .min_w(px(46.))
                    .h(px(44.))
                    .px(px(8.))
                    .rounded_xl()
                    .bg(rgb(if selected {
                        palette
                    } else {
                        theme::soft(0)
                    }))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(18.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(if selected { 0xFFFFFF } else { palette }))
                    .child(item)
                    .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                        this.blend_final = item;
                        cx.notify();
                    })),
            );
        }

        let equation = row()
            .items_center()
            .gap(px(10.))
            .child(tile(initial, theme::soft(0), theme::accent(0)))
            .child(
                div()
                    .text_size(px(20.))
                    .text_color(rgb(theme::MUTED))
                    .child("+"),
            )
            .child(tile(final_, theme::soft(2), theme::accent(2)))
            .child(
                div()
                    .text_size(px(20.))
                    .text_color(rgb(theme::MUTED))
                    .child("="),
            )
            .child(
                div()
                    .flex_1()
                    .h(px(64.))
                    .rounded_2xl()
                    .bg(rgb(theme::accent(1)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(26.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xFFFFFF))
                    .child(plain.clone()),
            );

        let mut tones = row().gap(px(8.));
        let mut tone_count = 0;
        if let Some(blend) = blend {
            for (marked, hanzi) in blend.tones {
                let tone = pinyin::tone_number(marked).max(1);
                let played = plain.clone();
                tones = tones.child(
                    col()
                        .id(format!("blend-tone-{tone}"))
                        .flex_1()
                        .py(px(8.))
                        .rounded_xl()
                        .bg(rgb(theme::soft(3)))
                        .items_center()
                        .gap(px(2.))
                        .child(
                            div()
                                .text_size(px(10.))
                                .text_color(rgb(theme::MUTED))
                                .child(TONE_NAMES[(tone - 1) as usize]),
                        )
                        .child(
                            div()
                                .text_size(px(17.))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(theme::accent(3)))
                                .child(*marked),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(rgb(theme::INK))
                                .child(*hanzi),
                        )
                        .on_click(cx.listener(move |_this, _: &ClickEvent, _window, _cx| {
                            audio::play_tone(&played, tone);
                        })),
                );
                tone_count += 1;
            }
        }

        let hint = if tone_count == 0 {
            "这个组合没有常用例字，先照着拼音读一读"
        } else if has_audio {
            "点一点，听听四个声调"
        } else {
            "这个音节还没有录音，先照着拼音读一读"
        };

        col()
            .id("blend-scroll")
            .flex_1()
            .min_h(px(0.))
            .gap(px(10.))
            .overflow_y_scroll()
            .child(label("第一步，选一个声母"))
            .child(initial_chips)
            .child(label("第二步，选一个韵母"))
            .child(final_chips)
            .child(
                col()
                    .bg(rgb(theme::PAPER))
                    .rounded_2xl()
                    .p(px(14.))
                    .gap(px(10.))
                    .child(equation)
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(rgb(theme::MUTED))
                            .child(hint),
                    )
                    .children((tone_count > 0).then(|| tones.into_any_element())),
            )
            .into_any_element()
    }

    // ── 家长教辅 ────────────────────────────────────────────────────────────

    fn guide(&self) -> AnyElement {
        let mut list = col().w_full().gap(px(10.));
        for (i, (title, body)) in pinyin::HOW_TO.iter().enumerate() {
            list = list.child(
                row()
                    .w_full()
                    .items_start()
                    .gap(px(10.))
                    .child(
                        div()
                            .size(px(22.))
                            .rounded_full()
                            .bg(rgb(theme::soft(i)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(11.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(theme::accent(i)))
                            .child(format!("{}", i + 1)),
                    )
                    .child(
                        col()
                            .flex_1()
                            .min_w(px(0.))
                            .gap(px(2.))
                            .child(
                                div()
                                    .w_full()
                                    .text_size(px(14.))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(theme::INK))
                                    .child(*title),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .text_size(px(12.))
                                    .line_height(px(18.))
                                    .text_color(rgb(theme::MUTED))
                                    .child(*body),
                            ),
                    ),
            );
        }

        col()
            .id("guide-scroll")
            .flex_1()
            .w_full()
            .min_h(px(0.))
            .gap(px(10.))
            .overflow_y_scroll()
            .child(
                div()
                    .text_size(px(16.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(theme::INK))
                    .child("怎么陪孩子练"),
            )
            .child(list)
            .child(
                col()
                    .bg(rgb(theme::soft(2)))
                    .rounded_2xl()
                    .p(px(12.))
                    .gap(px(4.))
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(rgb(theme::MUTED))
                            .child("拼读口诀"),
                    )
                    .child(
                        div()
                            .text_size(px(15.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(theme::accent(2)))
                            .child(pinyin::BLEND_RHYME),
                    ),
            )
            .into_any_element()
    }

    // ── 玩一玩 ──────────────────────────────────────────────────────────────

    fn quiz(&mut self, cx: &mut Context<Self>) -> AnyElement {
        if self.quiz.finished {
            return self.quiz_result(cx);
        }
        let Some(question) = self.quiz.question else {
            return col()
                .child(div().text_color(rgb(theme::MUTED)).child("题目还没有准备好"))
                .into_any_element();
        };

        let chosen = self.quiz.chosen;
        let wrong = self.quiz.wrong;
        let (marked, hanzi) = question.blend.tones[question.tone];
        let pinyin_text = marked;
        let plain = question.blend.plain;
        let tone_no = pinyin::tone_number(marked).max(1);

        let top = row()
            .items_center()
            .gap(px(8.))
            .child(
                div()
                    .text_size(px(13.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(theme::MUTED))
                    .child(format!(
                        "第 {} / {} 题",
                        (self.quiz.index + 1).min(self.quiz.total),
                        self.quiz.total
                    )),
            )
            .child(div().flex_1())
            .child(
                div()
                    .text_size(px(13.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(theme::accent(1)))
                    .child(format!("答对 {} 题", self.quiz.score)),
            );

        let speaker = div()
            .id("quiz-speak")
            .size(px(60.))
            .rounded_full()
            .bg(rgb(theme::accent(2)))
            .flex()
            .items_center()
            .justify_center()
            .text_size(px(26.))
            .text_color(rgb(0xFFFFFF))
            .child("♪")
            .on_click(cx.listener(move |_this, _: &ClickEvent, _window, _cx| {
                audio::play_tone(plain, tone_no);
            }));

        let prompt = row()
            .items_center()
            .gap(px(12.))
            .child(
                div()
                    .flex_none()
                    .size(px(76.))
                    .rounded_2xl()
                    .bg(rgb(theme::soft(1)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(44.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(theme::accent(1)))
                    .child(hanzi),
            )
            .child(
                col()
                    .flex_1()
                    .min_w(px(0.))
                    .gap(px(3.))
                    .child(
                        div()
                            .w_full()
                            .text_size(px(16.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(theme::INK))
                            .child("听一听，选出正确的拼音"),
                    )
                    .child(
                        div()
                            .w_full()
                            .text_size(px(12.))
                            .text_color(rgb(theme::MUTED))
                            .child("点喇叭可以再听一次"),
                    ),
            )
            .child(speaker);

        let mut options = col().gap(px(8.));
        for line in 0..2 {
            let mut row_ = row().gap(px(8.));
            for slot in 0..2 {
                let i = line * 2 + slot;
                let is_correct = i == question.correct;
                let is_chosen = chosen == Some(i);
                let is_wrong = wrong == Some(i);

                let (bg, border, fg) = if is_chosen || (chosen.is_some() && is_correct) {
                    (0xE3F7EC, 0x2E8B57, 0x1E7A4B)
                } else if is_wrong {
                    (0xFCE9E5, 0xC0503C, 0xB03A28)
                } else {
                    (theme::PAPER, theme::LINE, theme::INK)
                };

                row_ = row_.child(
                    div()
                        .id(format!("quiz-option-{i}"))
                        .flex_1()
                        .h(px(84.))
                        .rounded_2xl()
                        .border_2()
                        .border_color(rgb(border))
                        .bg(rgb(bg))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(30.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(fg))
                        .child(question.options[i])
                        .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                            if this.quiz.chosen.is_some() {
                                return;
                            }
                            let Some(q) = this.quiz.question else {
                                return;
                            };
                            if i == q.correct {
                                this.quiz.chosen = Some(i);
                                this.quiz.wrong = None;
                                this.quiz.score += 1;
                                audio::play_tone(q.blend.plain, pinyin::tone_number(q.blend.tones[q.tone].0).max(1));
                            } else {
                                this.quiz.wrong = Some(i);
                            }
                            cx.notify();
                        })),
                );
            }
            options = options.child(row_);
        }

        let feedback = if chosen.is_some() {
            row()
                .items_center()
                .gap(px(10.))
                .child(
                    div()
                        .flex_1()
                        .min_w(px(0.))
                        .text_size(px(14.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x1E7A4B))
                        .child(format!("读作 {pinyin_text}，真棒！")),
                )
                .child(
                    div()
                        .id("quiz-next")
                        .px(px(18.))
                        .h(px(44.))
                        .rounded_full()
                        .bg(rgb(theme::accent(1)))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_size(px(14.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xFFFFFF))
                        .child(if self.quiz.index + 1 >= self.quiz.total {
                            "看看结果"
                        } else {
                            "下一题"
                        })
                        .on_click(cx.listener(|this, _: &ClickEvent, _window, cx| {
                            this.quiz.advance();
                            if let Some((plain, tone, _)) = this.quiz.audio() {
                                audio::play_tone(plain, tone);
                            }
                            cx.notify();
                        })),
                )
        } else if wrong.is_some() {
            div()
                .text_size(px(14.))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0xB03A28))
                .child("差一点点，再听一次吧")
        } else {
            div()
                .text_size(px(13.))
                .text_color(rgb(theme::MUTED))
                .child("先点一下喇叭，听清楚声调再选")
        };

        col()
            .id("quiz-page")
            .flex_1()
            .min_h(px(0.))
            .gap(px(12.))
            .child(top)
            .child(
                col()
                    .bg(rgb(theme::PAPER))
                    .rounded_2xl()
                    .p(px(16.))
                    .gap(px(14.))
                    .child(prompt)
                    .child(options)
                    .child(feedback),
            )
            .into_any_element()
    }

    fn quiz_result(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let score = self.quiz.score;
        let total = self.quiz.total;
        col()
            .id("quiz-result")
            .flex_1()
            .min_h(px(0.))
            .gap(px(12.))
            .child(
                col()
                    .flex_1()
                    .bg(rgb(theme::PAPER))
                    .rounded_2xl()
                    .p(px(20.))
                    .gap(px(14.))
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(40.))
                            .text_color(rgb(theme::accent(0)))
                            .child("★"),
                    )
                    .child(
                        div()
                            .text_size(px(24.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(theme::INK))
                            .child("全部答完啦！"),
                    )
                    .child(
                        div()
                            .text_size(px(15.))
                            .text_color(rgb(theme::MUTED))
                            .child(format!("这一轮答对 {score} 题，共 {total} 题")),
                    )
                    .child(
                        div()
                            .id("quiz-restart")
                            .px(px(24.))
                            .h(px(48.))
                            .rounded_full()
                            .bg(rgb(theme::accent(1)))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_size(px(15.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("再玩一次")
                            .on_click(cx.listener(|this, _: &ClickEvent, _window, cx| {
                                this.quiz.restart();
                                if let Some((plain, tone, _)) = this.quiz.audio() {
                                    audio::play_tone(plain, tone);
                                }
                                cx.notify();
                            })),
                    ),
            )
            .into_any_element()
    }
}

fn label(text: &'static str) -> AnyElement {
    div()
        .text_size(px(12.))
        .font_weight(FontWeight::BOLD)
        .text_color(rgb(theme::MUTED))
        .child(text)
        .into_any_element()
}

fn tile(text: &'static str, bg: u32, fg: u32) -> AnyElement {
    div()
        .size(px(64.))
        .rounded_2xl()
        .bg(rgb(bg))
        .flex()
        .items_center()
        .justify_center()
        .text_size(px(26.))
        .font_weight(FontWeight::BOLD)
        .text_color(rgb(fg))
        .child(text)
        .into_any_element()
}

fn col() -> Div {
    div().flex().flex_col()
}

fn row() -> Div {
    div().flex().flex_row()
}
