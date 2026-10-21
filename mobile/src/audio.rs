//! 手机端的声音。
//!
//! 桌面端用 Windows MCI + SAPI 播放嵌入的 mp3；手机端要换成：
//!
//! - Android：`MediaPlayer`（走 JNI）或 `AAudio` / `oboe`；
//! - iOS：`AVAudioPlayer`。
//!
//! 录音资源本身和桌面端是同一套（`assets/audio`），命名规则也一样：
//! `base/<拼音>.mp3`（`ü` 写作 `v`）和 `tone/<音节><声调>.mp3`。
//!
//! 这一版先把接口留好，等音频层接上再替换实现，界面代码不用动。

/// 播放一个字母的标准读音（`pinyin` 用 ü 写法，如 "ü"、"zh"）。
pub fn play_letter(_pinyin: &str) {}

/// 播放某个音节某一调的录音（`syllable` 不带声调，如 "ba"；`tone` 取 1..=4）。
pub fn play_tone(_syllable: &str, _tone: u8) {}

/// 是否有这个音的示范录音。
pub fn has_letter(_pinyin: &str) -> bool {
    false
}

/// 是否有这个音节这一调的录音。
pub fn has_tone(_syllable: &str, _tone: u8) -> bool {
    false
}

/// 录音文件在 APK 资源里的相对路径，接播放器时直接用它。
pub fn letter_clip(pinyin: &str) -> String {
    format!("audio/base/{}.mp3", pinyin.replace('ü', "v"))
}

pub fn tone_clip(syllable: &str, tone: u8) -> String {
    format!(
        "audio/tone/{}{}.mp3",
        syllable.replace('ü', "v"),
        tone.clamp(1, 4)
    )
}
