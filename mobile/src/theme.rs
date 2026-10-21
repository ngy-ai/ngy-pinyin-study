//! 手机端配色：和桌面端同一套暖色，只是把卡片做小、圆角做大。

pub const BG: u32 = 0xFDF6EC;
pub const PAPER: u32 = 0xFFFFFF;
pub const INK: u32 = 0x3D3229;
pub const MUTED: u32 = 0x8C7B6B;
pub const LINE: u32 = 0xF0E6DA;

/// 四组主色（声母 / 韵母 / 拼读 / 教辅），和桌面端 ui::tint 对应。
pub const ACCENT: [u32; 4] = [0xE2703A, 0x2E8B57, 0x3A6EA5, 0x7B5EA7];

/// 对应主色的浅底。
pub const SOFT: [u32; 4] = [0xFDEDE4, 0xE8F5EC, 0xE8F0F8, 0xF0EAF8];

pub fn accent(i: usize) -> u32 {
    ACCENT[i % ACCENT.len()]
}

pub fn soft(i: usize) -> u32 {
    SOFT[i % SOFT.len()]
}
