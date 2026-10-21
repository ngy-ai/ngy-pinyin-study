//! 手机端的声音。
//!
//! 录音资源跟桌面端是同一套（`assets/audio`），命名规则也一致：
//! `base/<拼音>.mp3`（`ü` 写作 `v`）和 `tone/<音节><声调>.mp3`。
//! 编译期用 rust-embed 嵌进二进制，第一次播放时解压到应用私有目录。
//!
//! - Android：JNI 调 `android.media.MediaPlayer`，不需要额外的 Java 类。
//! - iOS：还没接 `AVAudioPlayer`，先静音。
//! - 播放放在独立线程，界面不会卡；新请求会打断正在播的那一条。

use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;

/// 编译期嵌入的录音资源（相对 mobile/Cargo.toml）。
#[derive(rust_embed::RustEmbed)]
#[folder = "../assets/audio"]
struct Sounds;

#[cfg(target_os = "android")]
pub use imp::Cmd;

#[cfg(not(target_os = "android"))]
pub enum Cmd {
    Clip { id: String },
}

static SENDER: OnceLock<Option<Sender<Cmd>>> = OnceLock::new();
static ENABLED: AtomicBool = AtomicBool::new(true);

/// 播放一个字母的标准读音（`pinyin` 用 ü 写法，如 "ü"、"zh"）。
pub fn play_letter(pinyin: &str) {
    send(Cmd::Clip {
        id: letter_clip(pinyin),
    });
}

/// 播放某个音节某一调的录音（`syllable` 不带声调，如 "ba"；`tone` 取 1..=4）。
pub fn play_tone(syllable: &str, tone: u8) {
    send(Cmd::Clip {
        id: tone_clip(syllable, tone),
    });
}

/// 是否有这个音的示范录音。
pub fn has_letter(pinyin: &str) -> bool {
    Sounds::get(&letter_clip(pinyin)).is_some()
}

/// 是否有这个音节这一调的录音。
pub fn has_tone(syllable: &str, tone: u8) -> bool {
    Sounds::get(&tone_clip(syllable, tone)).is_some()
}

/// 打开 / 关闭声音。
pub fn set_enabled(enabled: bool) {
    ENABLED.store(enabled, Ordering::Relaxed);
    #[cfg(target_os = "android")]
    if !enabled {
        imp::stop();
    }
}

/// 录音在资源里的相对路径。
pub fn letter_clip(pinyin: &str) -> String {
    format!("base/{}.mp3", pinyin.replace('ü', "v"))
}

pub fn tone_clip(syllable: &str, tone: u8) -> String {
    format!(
        "tone/{}{}.mp3",
        syllable.replace('ü', "v"),
        tone.clamp(1, 4)
    )
}

fn send(cmd: Cmd) {
    if !ENABLED.load(Ordering::Relaxed) {
        return;
    }
    let tx = SENDER.get_or_init(|| imp::spawn());
    if let Some(tx) = tx {
        #[cfg(target_os = "android")]
        imp::bump_generation();
        let _ = tx.send(cmd);
    }
}

#[cfg(target_os = "android")]
mod imp {
    use super::Sounds;
    use jni::objects::{JObject, JString, JValue};
    use jni::{JNIEnv, JavaVM};
    // Cmd 定义在本模块里，由父模块 `pub use` 出去
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::mpsc::{Receiver, Sender, channel};
    use std::time::{Duration, Instant};

    pub enum Cmd {
        Clip { id: String },
    }

    /// 每来一个新请求就加一，播放循环看到变化就停掉当前音频。
    static GENERATION: AtomicU64 = AtomicU64::new(0);

    pub fn bump_generation() {
        GENERATION.fetch_add(1, Ordering::Relaxed);
    }

    pub fn stop() {
        bump_generation();
    }

    pub fn spawn() -> Option<Sender<Cmd>> {
        let (tx, rx) = channel();
        std::thread::Builder::new()
            .name("pinyin-audio".into())
            .spawn(move || run(rx))
            .ok()?;
        Some(tx)
    }

    fn run(rx: Receiver<Cmd>) {
        let Some(vm) = vm() else {
            log::warn!("audio: 拿不到 JavaVM，本次会话静音");
            return;
        };
        let Ok(mut env) = vm.attach_current_thread_permanently() else {
            log::warn!("audio: attach 线程失败，本次会话静音");
            return;
        };
        let Some(dir) = cache_dir(&mut env) else {
            log::warn!("audio: 拿不到 getFilesDir，本次会话静音");
            return;
        };

        while let Ok(Cmd::Clip { id }) = rx.recv() {
            play(&mut env, &dir, &id);
        }
    }

    fn vm() -> Option<JavaVM> {
        let ctx = ndk_context::android_context();
        unsafe { JavaVM::from_raw(ctx.vm().cast()).ok() }
    }

    /// `<应用私有目录>/audio-v1`，首次用到时创建。
    fn cache_dir(env: &mut JNIEnv) -> Option<PathBuf> {
        let ctx = ndk_context::android_context();
        // 这个引用归 android-activity 管，不能删，所以只借用不改所有权
        let activity = unsafe { JObject::from_raw(ctx.context().cast()) };

        let file = env
            .call_method(&activity, "getFilesDir", "()Ljava/io/File;", &[])
            .ok()?
            .l()
            .ok()?;
        let path = env
            .call_method(&file, "getAbsolutePath", "()Ljava/lang/String;", &[])
            .ok()?
            .l()
            .ok()?;
        let base: String = env.get_string(&JString::from(path)).ok()?.into();

        let dir = PathBuf::from(base).join("audio-v1");
        std::fs::create_dir_all(&dir).ok()?;
        Some(dir)
    }

    /// 把嵌入的 mp3 落到磁盘（已存在且大小一致就跳过），返回路径。
    fn materialize(dir: &Path, id: &str) -> Option<PathBuf> {
        let file = Sounds::get(id)?;
        let path = dir.join(id.replace('/', "-"));
        if let Ok(meta) = std::fs::metadata(&path)
            && meta.len() == file.data.len() as u64
        {
            return Some(path);
        }
        std::fs::write(&path, file.data.as_ref()).ok()?;
        Some(path)
    }

    fn play(env: &mut JNIEnv, dir: &Path, id: &str) {
        let Some(path) = materialize(dir, id) else {
            log::warn!("audio: 没有嵌入的录音 {id}");
            return;
        };
        let player = match env.new_object("android/media/MediaPlayer", "()V", &[]) {
            Ok(player) => player,
            Err(err) => {
                log::warn!("audio: 创建 MediaPlayer 失败: {err}");
                return;
            }
        };

        let jpath = match env.new_string(path.to_string_lossy().to_string()) {
            Ok(s) => s,
            Err(err) => {
                log::warn!("audio: 构造路径字符串失败: {err}");
                return;
            }
        };
        if let Err(err) = env.call_method(
            &player,
            "setDataSource",
            "(Ljava/lang/String;)V",
            &[JValue::Object(&jpath)],
        ) {
            log::warn!("audio: setDataSource 失败 {id}: {err}");
            let _ = env.exception_clear();
            let _ = env.call_method(&player, "release", "()V", &[]);
            return;
        }
        if let Err(err) = env.call_method(&player, "prepare", "()V", &[]) {
            log::warn!("audio: prepare 失败 {id}: {err}");
            let _ = env.exception_clear();
            let _ = env.call_method(&player, "release", "()V", &[]);
            return;
        }

        let generation = GENERATION.load(Ordering::Relaxed);
        if let Err(err) = env.call_method(&player, "start", "()V", &[]) {
            log::warn!("audio: start 失败 {id}: {err}");
            let _ = env.exception_clear();
            let _ = env.call_method(&player, "release", "()V", &[]);
            return;
        }

        // MediaPlayer 没有回调就得自己等：按 duration 睡，期间被新请求打断就提前停
        let ms = env
            .call_method(&player, "getDuration", "()I", &[])
            .ok()
            .and_then(|v| v.i().ok())
            .unwrap_or(800)
            .max(200) as u64;
        let deadline = Instant::now() + Duration::from_millis(ms + 80);
        while Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(25));
            if GENERATION.load(Ordering::Relaxed) != generation {
                break;
            }
        }

        let _ = env.call_method(&player, "stop", "()V", &[]);
        let _ = env.call_method(&player, "release", "()V", &[]);
    }
}

#[cfg(not(target_os = "android"))]
mod imp {
    use super::Cmd;
    use std::sync::mpsc::Sender;

    pub fn spawn() -> Option<Sender<Cmd>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ü_is_spelled_v_in_file_names() {
        assert_eq!(letter_clip("ü"), "base/v.mp3");
        assert_eq!(letter_clip("zh"), "base/zh.mp3");
        assert_eq!(tone_clip("nüe", 4), "tone/nve4.mp3");
        assert_eq!(tone_clip("ba", 9), "tone/ba4.mp3");
    }

    #[test]
    fn every_letter_and_tone_has_a_clip() {
        // 桌面端有同样的断言，这里保证手机端嵌入的资源没丢
        assert!(has_letter("a"));
        assert!(has_letter("zh"));
        assert!(has_letter("ü"));
        assert!(has_tone("ba", 1));
        assert!(has_tone("bao", 2));
    }
}
