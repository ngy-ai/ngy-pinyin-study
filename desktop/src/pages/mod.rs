//! 四个页面：认一认、拼一拼、玩一玩，以及给大人的家长教辅。
//!
//! 每个页面都是 `impl AppView` 上的一组渲染 / 交互方法，
//! 状态集中放在 [`AppPages`] 里，避免在多个 Entity 之间来回传状态。

pub mod blend;
pub mod guide;
pub mod learn;
pub mod quiz;

use blend::BlendState;
use gpui_kit::*;
use learn::LearnState;
use quiz::QuizState;

pub struct AppPages {
    pub learn: LearnState,
    pub blend: BlendState,
    pub quiz: QuizState,
    /// 家长教辅页面的滚动位置
    pub guide_scroll: ScrollHandle,
}

impl AppPages {
    pub fn new() -> Self {
        Self {
            learn: LearnState::new(),
            blend: BlendState::new(),
            quiz: QuizState::new(),
            guide_scroll: ScrollHandle::new(),
        }
    }
}
