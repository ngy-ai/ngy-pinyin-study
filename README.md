# 拼音乐园

一个给幼儿（3–6 岁）用的拼音学习桌面应用，家长在旁边陪着一起用。使用 [gpui-kit](https://gpui-kit.com)（Rust + GPUI）构建。

## 功能

| 页面 | 内容 |
| --- | --- |
| 认一认 | 声母 23 个、韵母 24 个、整体认读 16 个；大卡片 + 例词 + **四声练习** + 发音要领 |
| 拼一拼 | 选声母 + 选韵母 → 看音节，四声例字（八 / 拔 / 把 / 爸）逐张点读 |
| 玩一玩 | 每轮 10 题，听读音在四个带声调的拼音里选一个，答对得小星星 |
| 家长教辅 | 怎么陪孩子练、拼读口诀、声母 23 + 韵母 24 的发音要领与常见错误 |

- 认一认的韵母和整体认读卡片下方有 **四声练习**：`ō ó ǒ ò`、`zhī zhí zhǐ zhì`……点一下就是标准录音。
  四声行只写韵母本身（`i` → `ī í ǐ ì`、`ui` → `uī uí uǐ uì`、`ong` → `ōng óng ǒng òng`），
  不加 y / w，也不加声母；录音用同音的零声母音节（`i` → `yi`、`ui` → `wei`、`iu` → `you`、`un` → `wen`），
  声音完全一样。`ong` 用的是单独录的 `ong` 四声（来源见下）。
- 每张卡片底部有一行 **发音要领**，家长扫一眼就知道怎么示范。
- 顶部有星星计数、声音开关和家长设置（朗读语速）。

## 声音从哪里来

**默认播放内置录音**（`assets/audio`，共 3.0 MB，编译时嵌进二进制）：

| 目录 | 内容 | 来源与许可 |
| --- | --- | --- |
| `base/*.mp3` | 声母 23 + 韵母 24 + 整体认读 16 的示范音（呼读音） | [CodeTrainerMan/typingchinese](https://github.com/CodeTrainerMan/typingchinese)，整理自 hanyupinyin.cn（该站未声明开放许可） |
| `tone/*.mp3` | 70 个音节 × 四声，共 280 个（韵母四声、拼读、题目） | [hugolpz/audio-cmn](https://github.com/hugolpz/audio-cmn) 的 1,707 个音节录音（录音：Chen Wang，整理：Hugo Lopez / PLIDAM, INALCO），**CC BY-SA**；其中 `ong` 四声取自 [zispace/hanyu-pinyin-audio](https://github.com/zispace/hanyu-pinyin-audio) 的 `misc/2-所有音节` |

- 录音在首次播放时解压到 `%LOCALAPPDATA%\pinyin-study\audio-v1\`，用 Windows MCI 播放。
- **没有录音时自动退回系统语音合成**（SAPI5），所以删掉 `assets/audio` 也能跑，只是音质变成机器音。
- ⚠️ `base/` 那一组的上游（hanyupinyin.cn）没有开放许可。自用没问题；**要公开发布就换掉它**，
  或者向站方取得授权。`tone/` 那组是 CC BY-SA：发布时请保留「Chen Wang 录音 / Hugo Lopez 整理」的署名，
  音频文件本身也要继续以 CC BY-SA 提供。

命名规则：`base/<拼音>.mp3`（ü 写作 `v`）、`tone/<音节><声调 1-4>.mp3`。换音源只要遵守这个规则，代码不用改。

## 运行

```bash
cargo run
# 跑测试：标调规则 + 每个音是否都有录音
cargo test
```

首次构建会编译 GPUI 依赖，需要几分钟。

### 环境要求

- Rust 1.92+（MSVC 工具链），Windows 10+
- Visual Studio 2022 生成工具（含 C++ 桌面开发）和 CMake
- 想听机器人声兜底才需要中文语音包（Windows「设置 → 时间和语言 → 语音 → 管理语音」）

## 代码结构

```
src/
├── main.rs        # 入口、窗口、页面路由、顶栏与左侧导航
├── pinyin.rs      # 拼音数据、标调规则、家长教辅文案、抽题用的小随机数
├── audio.rs       # 录音播放（MCI）+ 语音合成兜底（SAPI5），后台线程
├── ui.rs          # 幼儿友好的配色与通用小部件
└── pages/
    ├── learn.rs   # 认一认（含四声练习）
    ├── blend.rs   # 拼一拼
    ├── quiz.rs    # 玩一玩
    └── guide.rs   # 家长教辅
```

- 页面状态集中放在 `AppView` / `AppPages` 里，每个页面都是 `impl AppView` 上的渲染方法，避免多 Entity 来回传状态。
- 标调（`pinyin::with_tone`）按「有 a/o/e 标 a/o/e，否则标靠后的 i/u」的规则算，不手写 96 个带调字符串。
- 拼读表只收录四声（或三声）都有常用例字的音节，保证给出的例字都是真实存在的词。
