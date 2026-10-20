//! 声音：优先播放内置的标准普通话录音，缺失时退回系统语音合成。
//!
//! - 录音放在 `assets/audio`，编译期嵌进二进制；首次播放时解压到本地缓存目录，
//!   再用 Windows MCI 播放（wav / mp3 都支持）。
//! - 没有对应录音（或播放失败）时，用 SAPI5 朗读兜底。
//! - 所有播放请求都投递到同一个后台线程，不会阻塞界面，新请求会打断旧内容。
//! - 非 Windows 平台静默降级为「无声音」，其余功能不受影响。

use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;

/// 编译期嵌入的录音资源。
#[derive(rust_embed::RustEmbed)]
#[folder = "assets/audio"]
struct Sounds;

/// 发给播放线程的指令。
pub enum Cmd {
    /// 播放内置录音；拿不到录音时朗读 `fallback`
    Clip { id: String, fallback: String },
    /// 语音合成
    Say(String),
    /// 语速
    Rate(i32),
    /// 立刻静音
    Stop,
}

static SENDER: OnceLock<Option<Sender<Cmd>>> = OnceLock::new();
static ENABLED: AtomicBool = AtomicBool::new(true);

/// 启动播放线程（只启动一次，可重复调用）。
pub fn init() {
    SENDER.get_or_init(imp::spawn);
}

/// 打开 / 关闭声音。
pub fn set_enabled(enabled: bool) {
    ENABLED.store(enabled, Ordering::Relaxed);
    if !enabled && let Some(Some(tx)) = SENDER.get() {
        let _ = tx.send(Cmd::Stop);
    }
}

/// 语速，取值 -10（最慢）到 10（最快），0 为正常。
pub fn set_rate(rate: i32) {
    if let Some(Some(tx)) = SENDER.get() {
        let _ = tx.send(Cmd::Rate(rate));
    }
}

/// 朗读一段文本。
pub fn speak(text: impl Into<String>) {
    let text = text.into();
    if text.trim().is_empty() {
        return;
    }
    send(Cmd::Say(text));
}

/// 播放字母 / 音节的示范音（`pinyin` 用 ü 写法，如 "ü"、"zh"）。
pub fn play_letter(pinyin: &str, fallback: impl Into<String>) {
    let name = pinyin.replace('ü', "v");
    send(Cmd::Clip {
        id: format!("base/{name}.mp3"),
        fallback: fallback.into(),
    });
}

/// 播放某个音节某一调的标准录音（`syllable` 不带声调，如 "ma"；`tone` 取 1..=4）。
pub fn play_tone(syllable: &str, tone: u8, fallback: impl Into<String>) {
    send(Cmd::Clip {
        id: tone_clip(syllable, tone),
        fallback: fallback.into(),
    });
}

/// 是否有这个音的示范录音。
pub fn has_letter(pinyin: &str) -> bool {
    Sounds::get(&format!("base/{}.mp3", pinyin.replace('ü', "v"))).is_some()
}

/// 是否有这个音节这一调的录音。
pub fn has_tone(syllable: &str, tone: u8) -> bool {
    Sounds::get(&tone_clip(syllable, tone)).is_some()
}

fn tone_clip(syllable: &str, tone: u8) -> String {
    format!("tone/{syllable}{}.mp3", tone.clamp(1, 4))
}

fn send(cmd: Cmd) {
    if !ENABLED.load(Ordering::Relaxed) {
        return;
    }
    if let Some(Some(tx)) = SENDER.get() {
        let _ = tx.send(cmd);
    }
}

#[cfg(windows)]
mod imp {
    use super::{Cmd, Sounds};
    use std::path::{Path, PathBuf};
    use std::sync::mpsc::{Receiver, Sender, channel};
    use windows::Win32::Media::Multimedia::mciSendStringW;
    use windows::Win32::Media::Speech::{
        ISpVoice, SPF_ASYNC, SPF_IS_NOT_XML, SPF_PURGEBEFORESPEAK,
    };
    use windows::Win32::System::Com::{
        CLSCTX_ALL, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
    };
    use windows::core::{GUID, IUnknown, PCWSTR};

    /// SpVoice 的 CLSID：{96749377-3391-11D2-9EE3-00C04F797396}
    const CLSID_SP_VOICE: GUID = GUID::from_u128(0x96749377_3391_11d2_9ee3_00c04f797396);
    /// MCI 播放用的设备别名。
    const ALIAS: &str = "pinyin-study-clip";

    pub fn spawn() -> Option<Sender<Cmd>> {
        let (tx, rx) = channel::<Cmd>();
        std::thread::Builder::new()
            .name("pinyin-audio".into())
            .spawn(move || unsafe { run(rx) })
            .ok()?;
        Some(tx)
    }

    unsafe fn run(rx: Receiver<Cmd>) {
        let _ = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
        let voice: Option<ISpVoice> =
            unsafe { CoCreateInstance(&CLSID_SP_VOICE, None::<&IUnknown>, CLSCTX_ALL) }.ok();
        if let Some(voice) = &voice {
            unsafe {
                let _ = voice.SetRate(-1);
                let _ = voice.SetVolume(100);
            }
        }

        while let Ok(cmd) = rx.recv() {
            match cmd {
                Cmd::Rate(rate) => {
                    if let Some(voice) = &voice {
                        unsafe {
                            let _ = voice.SetRate(rate.clamp(-10, 10));
                        }
                    }
                }
                Cmd::Stop => {
                    stop_clip();
                    if let Some(voice) = &voice {
                        say(voice, "");
                    }
                }
                Cmd::Say(text) => {
                    stop_clip();
                    if let Some(voice) = &voice {
                        say(voice, &text);
                    }
                }
                Cmd::Clip { id, fallback } => {
                    let played = match cache_clip(&id) {
                        Some(path) => play_clip(&path),
                        None => false,
                    };
                    if played {
                        // 录音开始播放，把正在朗读的合成语音打断
                        if let Some(voice) = &voice {
                            say(voice, "");
                        }
                    } else if let Some(voice) = &voice {
                        say(voice, &fallback);
                    }
                }
            }
        }
    }

    fn say(voice: &ISpVoice, text: &str) {
        let mut wide: Vec<u16> = text.encode_utf16().collect();
        wide.push(0);
        let flags = (SPF_ASYNC.0 | SPF_PURGEBEFORESPEAK.0 | SPF_IS_NOT_XML.0) as u32;
        unsafe {
            let _ = voice.Speak(PCWSTR(wide.as_ptr()), flags, None);
        }
    }

    /// 把嵌入的录音写到本地缓存，返回可播放的路径。
    fn cache_clip(id: &str) -> Option<PathBuf> {
        let file = Sounds::get(id)?;
        let path = cache_dir().join(id.replace('/', "-"));
        if let Ok(meta) = std::fs::metadata(&path)
            && meta.len() == file.data.len() as u64
        {
            return Some(path);
        }
        std::fs::create_dir_all(path.parent()?).ok()?;
        std::fs::write(&path, &file.data).ok()?;
        Some(path)
    }

    fn cache_dir() -> PathBuf {
        let base = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        base.join("pinyin-study").join("audio-v1")
    }

    unsafe fn mci(cmd: &str) -> u32 {
        let mut wide: Vec<u16> = cmd.encode_utf16().collect();
        wide.push(0);
        unsafe { mciSendStringW(PCWSTR(wide.as_ptr()), None, None) }
    }

    fn stop_clip() {
        unsafe {
            mci(&format!("close {ALIAS}"));
        }
    }

    fn play_clip(path: &Path) -> bool {
        let type_ = match path.extension().and_then(|e| e.to_str()) {
            Some("mp3") => "mpegvideo",
            _ => "waveaudio",
        };
        unsafe {
            mci(&format!("close {ALIAS}"));
            let open = format!("open \"{}\" type {type_} alias {ALIAS}", path.display());
            let mut rc = mci(&open);
            if rc != 0 {
                // 个别系统不认 type 关键字，退一步让 MCI 自己按扩展名判断
                let open = format!("open \"{}\" alias {ALIAS}", path.display());
                rc = mci(&open);
            }
            rc == 0 && mci(&format!("play {ALIAS}")) == 0
        }
    }
}

#[cfg(not(windows))]
mod imp {
    use super::Cmd;
    use std::sync::mpsc::Sender;

    pub fn spawn() -> Option<Sender<Cmd>> {
        None
    }
}
