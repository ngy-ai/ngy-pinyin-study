//! 拼音数据：声母、韵母、整体认读音节、拼读组合，以及一个小巧的随机数发生器。
//!
//! `reading` 字段是给语音朗读用的汉字（声母用呼读音，如 b → 玻 bō），
//! 这样系统语音合成可以直接读出正确发音。

/// 一个可学习字母（声母 / 韵母 / 整体认读音节）。
pub struct Letter {
    /// 显示的拼音，如 "zh"
    pub text: &'static str,
    /// 朗读用汉字（呼读音），如 "知"
    pub reading: &'static str,
    /// 例词
    pub word: &'static str,
    /// 例词拼音
    pub word_pinyin: &'static str,
}

pub const INITIALS: &[Letter] = &[
    Letter {
        text: "b",
        reading: "玻",
        word: "爸爸",
        word_pinyin: "bà ba",
    },
    Letter {
        text: "p",
        reading: "坡",
        word: "皮球",
        word_pinyin: "pí qiú",
    },
    Letter {
        text: "m",
        reading: "摸",
        word: "妈妈",
        word_pinyin: "mā ma",
    },
    Letter {
        text: "f",
        reading: "佛",
        word: "房子",
        word_pinyin: "fáng zi",
    },
    Letter {
        text: "d",
        reading: "得",
        word: "大象",
        word_pinyin: "dà xiàng",
    },
    Letter {
        text: "t",
        reading: "特",
        word: "兔子",
        word_pinyin: "tù zi",
    },
    Letter {
        text: "n",
        reading: "讷",
        word: "牛奶",
        word_pinyin: "niú nǎi",
    },
    Letter {
        text: "l",
        reading: "勒",
        word: "老虎",
        word_pinyin: "lǎo hǔ",
    },
    Letter {
        text: "g",
        reading: "哥",
        word: "哥哥",
        word_pinyin: "gē ge",
    },
    Letter {
        text: "k",
        reading: "科",
        word: "蝌蚪",
        word_pinyin: "kē dǒu",
    },
    Letter {
        text: "h",
        reading: "喝",
        word: "花朵",
        word_pinyin: "huā duǒ",
    },
    Letter {
        text: "j",
        reading: "基",
        word: "鸡蛋",
        word_pinyin: "jī dàn",
    },
    Letter {
        text: "q",
        reading: "欺",
        word: "气球",
        word_pinyin: "qì qiú",
    },
    Letter {
        text: "x",
        reading: "希",
        word: "西瓜",
        word_pinyin: "xī guā",
    },
    Letter {
        text: "zh",
        reading: "知",
        word: "蜘蛛",
        word_pinyin: "zhī zhū",
    },
    Letter {
        text: "ch",
        reading: "吃",
        word: "汽车",
        word_pinyin: "qì chē",
    },
    Letter {
        text: "sh",
        reading: "诗",
        word: "狮子",
        word_pinyin: "shī zi",
    },
    Letter {
        text: "r",
        reading: "日",
        word: "日历",
        word_pinyin: "rì lì",
    },
    Letter {
        text: "z",
        reading: "资",
        word: "紫色",
        word_pinyin: "zǐ sè",
    },
    Letter {
        text: "c",
        reading: "次",
        word: "草莓",
        word_pinyin: "cǎo méi",
    },
    Letter {
        text: "s",
        reading: "思",
        word: "森林",
        word_pinyin: "sēn lín",
    },
    Letter {
        text: "y",
        reading: "衣",
        word: "衣服",
        word_pinyin: "yī fu",
    },
    Letter {
        text: "w",
        reading: "乌",
        word: "乌鸦",
        word_pinyin: "wū yā",
    },
];

pub const FINALS: &[Letter] = &[
    Letter {
        text: "a",
        reading: "啊",
        word: "阿姨",
        word_pinyin: "ā yí",
    },
    Letter {
        text: "o",
        reading: "哦",
        word: "喔喔",
        word_pinyin: "ō ō",
    },
    Letter {
        text: "e",
        reading: "鹅",
        word: "白鹅",
        word_pinyin: "bái é",
    },
    Letter {
        text: "i",
        reading: "衣",
        word: "衣服",
        word_pinyin: "yī fu",
    },
    Letter {
        text: "u",
        reading: "乌",
        word: "乌鸦",
        word_pinyin: "wū yā",
    },
    Letter {
        text: "ü",
        reading: "鱼",
        word: "金鱼",
        word_pinyin: "jīn yú",
    },
    Letter {
        text: "ai",
        reading: "爱",
        word: "爱心",
        word_pinyin: "ài xīn",
    },
    Letter {
        text: "ei",
        reading: "诶",
        word: "黑白",
        word_pinyin: "hēi bái",
    },
    Letter {
        text: "ui",
        reading: "围",
        word: "喝水",
        word_pinyin: "hē shuǐ",
    },
    Letter {
        text: "ao",
        reading: "袄",
        word: "小鸟",
        word_pinyin: "xiǎo niǎo",
    },
    Letter {
        text: "ou",
        reading: "欧",
        word: "海鸥",
        word_pinyin: "hǎi ōu",
    },
    Letter {
        text: "iu",
        reading: "优",
        word: "皮球",
        word_pinyin: "pí qiú",
    },
    Letter {
        text: "ie",
        reading: "椰",
        word: "树叶",
        word_pinyin: "shù yè",
    },
    Letter {
        text: "üe",
        reading: "月",
        word: "月亮",
        word_pinyin: "yuè liang",
    },
    Letter {
        text: "an",
        reading: "安",
        word: "蓝色",
        word_pinyin: "lán sè",
    },
    Letter {
        text: "en",
        reading: "恩",
        word: "谢谢",
        word_pinyin: "xiè xie",
    },
    Letter {
        text: "in",
        reading: "音",
        word: "音乐",
        word_pinyin: "yīn yuè",
    },
    Letter {
        text: "un",
        reading: "温",
        word: "春天",
        word_pinyin: "chūn tiān",
    },
    Letter {
        text: "ün",
        reading: "云",
        word: "白云",
        word_pinyin: "bái yún",
    },
    Letter {
        text: "ang",
        reading: "昂",
        word: "山羊",
        word_pinyin: "shān yáng",
    },
    Letter {
        text: "eng",
        reading: "鞥",
        word: "台灯",
        word_pinyin: "tái dēng",
    },
    Letter {
        text: "ing",
        reading: "英",
        word: "星星",
        word_pinyin: "xīng xing",
    },
    Letter {
        text: "ong",
        reading: "翁",
        word: "红色",
        word_pinyin: "hóng sè",
    },
];

pub const WHOLE: &[Letter] = &[
    Letter {
        text: "zhi",
        reading: "知",
        word: "一只",
        word_pinyin: "yì zhī",
    },
    Letter {
        text: "chi",
        reading: "吃",
        word: "吃饭",
        word_pinyin: "chī fàn",
    },
    Letter {
        text: "shi",
        reading: "诗",
        word: "十个",
        word_pinyin: "shí gè",
    },
    Letter {
        text: "ri",
        reading: "日",
        word: "日历",
        word_pinyin: "rì lì",
    },
    Letter {
        text: "zi",
        reading: "资",
        word: "写字",
        word_pinyin: "xiě zì",
    },
    Letter {
        text: "ci",
        reading: "次",
        word: "词语",
        word_pinyin: "cí yǔ",
    },
    Letter {
        text: "si",
        reading: "思",
        word: "四个",
        word_pinyin: "sì gè",
    },
    Letter {
        text: "yi",
        reading: "衣",
        word: "一个",
        word_pinyin: "yī gè",
    },
    Letter {
        text: "wu",
        reading: "乌",
        word: "五个",
        word_pinyin: "wǔ gè",
    },
    Letter {
        text: "yu",
        reading: "鱼",
        word: "小鱼",
        word_pinyin: "xiǎo yú",
    },
    Letter {
        text: "ye",
        reading: "椰",
        word: "树叶",
        word_pinyin: "shù yè",
    },
    Letter {
        text: "yue",
        reading: "月",
        word: "月亮",
        word_pinyin: "yuè liang",
    },
    Letter {
        text: "yuan",
        reading: "圆",
        word: "圆圈",
        word_pinyin: "yuán quān",
    },
    Letter {
        text: "yin",
        reading: "音",
        word: "声音",
        word_pinyin: "shēng yīn",
    },
    Letter {
        text: "yun",
        reading: "云",
        word: "白云",
        word_pinyin: "bái yún",
    },
    Letter {
        text: "ying",
        reading: "鹰",
        word: "老鹰",
        word_pinyin: "lǎo yīng",
    },
];

/// 学习页的分类。
pub const TABS: &[(&str, &[Letter])] = &[("声母", INITIALS), ("韵母", FINALS), ("整体认读", WHOLE)];

/// 一个可拼读的音节。
pub struct Blend {
    pub initial: &'static str,
    pub final_: &'static str,
    /// 不带声调的音节，如 "ba"
    pub plain: &'static str,
    /// 四声（或三声）示例：(带调拼音, 汉字)
    pub tones: &'static [(&'static str, &'static str)],
}

macro_rules! blend {
    ($initial:literal, $final:literal, $plain:literal, [$(($p:literal, $h:literal)),* $(,)?]) => {
        Blend { initial: $initial, final_: $final, plain: $plain, tones: &[$(($p, $h)),*] }
    };
}

pub const BLENDS: &[Blend] = &[
    blend!(
        "b",
        "a",
        "ba",
        [("bā", "八"), ("bá", "拔"), ("bǎ", "把"), ("bà", "爸")]
    ),
    blend!(
        "b",
        "i",
        "bi",
        [("bī", "逼"), ("bí", "鼻"), ("bǐ", "笔"), ("bì", "必")]
    ),
    blend!(
        "b",
        "ao",
        "bao",
        [("bāo", "包"), ("báo", "薄"), ("bǎo", "宝"), ("bào", "报")]
    ),
    blend!(
        "p",
        "ao",
        "pao",
        [("pāo", "抛"), ("páo", "袍"), ("pǎo", "跑"), ("pào", "泡")]
    ),
    blend!(
        "m",
        "a",
        "ma",
        [("mā", "妈"), ("má", "麻"), ("mǎ", "马"), ("mà", "骂")]
    ),
    blend!(
        "m",
        "i",
        "mi",
        [("mī", "眯"), ("mí", "迷"), ("mǐ", "米"), ("mì", "蜜")]
    ),
    blend!(
        "m",
        "ao",
        "mao",
        [("māo", "猫"), ("máo", "毛"), ("mào", "帽")]
    ),
    blend!(
        "f",
        "ei",
        "fei",
        [("fēi", "飞"), ("féi", "肥"), ("fěi", "匪"), ("fèi", "费")]
    ),
    blend!(
        "d",
        "a",
        "da",
        [("dā", "搭"), ("dá", "答"), ("dǎ", "打"), ("dà", "大")]
    ),
    blend!(
        "d",
        "i",
        "di",
        [("dī", "低"), ("dí", "敌"), ("dǐ", "底"), ("dì", "弟")]
    ),
    blend!(
        "d",
        "u",
        "du",
        [("dū", "都"), ("dú", "读"), ("dǔ", "堵"), ("dù", "度")]
    ),
    blend!(
        "t",
        "i",
        "ti",
        [("tī", "踢"), ("tí", "提"), ("tǐ", "体"), ("tì", "替")]
    ),
    blend!(
        "t",
        "u",
        "tu",
        [("tū", "秃"), ("tú", "图"), ("tǔ", "土"), ("tù", "兔")]
    ),
    blend!(
        "t",
        "ao",
        "tao",
        [("tāo", "掏"), ("táo", "桃"), ("tǎo", "讨"), ("tào", "套")]
    ),
    blend!("n", "u", "nu", [("nú", "奴"), ("nǔ", "努"), ("nù", "怒")]),
    blend!("l", "u", "lu", [("lú", "炉"), ("lǔ", "卤"), ("lù", "路")]),
    blend!("g", "e", "ge", [("gē", "哥"), ("gé", "格"), ("gè", "个")]),
    blend!("g", "u", "gu", [("gū", "姑"), ("gǔ", "古"), ("gù", "故")]),
    blend!(
        "k",
        "e",
        "ke",
        [("kē", "科"), ("ké", "壳"), ("kě", "可"), ("kè", "课")]
    ),
    blend!("h", "e", "he", [("hē", "喝"), ("hé", "河"), ("hè", "贺")]),
    blend!(
        "h",
        "u",
        "hu",
        [("hū", "呼"), ("hú", "湖"), ("hǔ", "虎"), ("hù", "户")]
    ),
    blend!(
        "j",
        "i",
        "ji",
        [("jī", "鸡"), ("jí", "急"), ("jǐ", "几"), ("jì", "记")]
    ),
    blend!(
        "q",
        "i",
        "qi",
        [("qī", "七"), ("qí", "骑"), ("qǐ", "起"), ("qì", "气")]
    ),
    blend!(
        "x",
        "i",
        "xi",
        [("xī", "西"), ("xí", "席"), ("xǐ", "洗"), ("xì", "细")]
    ),
    blend!(
        "zh",
        "i",
        "zhi",
        [("zhī", "知"), ("zhí", "直"), ("zhǐ", "纸"), ("zhì", "治")]
    ),
    blend!(
        "zh",
        "a",
        "zha",
        [("zhā", "扎"), ("zhá", "闸"), ("zhǎ", "眨"), ("zhà", "炸")]
    ),
    blend!(
        "zh",
        "e",
        "zhe",
        [("zhē", "遮"), ("zhé", "折"), ("zhě", "者"), ("zhè", "这")]
    ),
    blend!(
        "zh",
        "u",
        "zhu",
        [("zhū", "猪"), ("zhú", "竹"), ("zhǔ", "主"), ("zhù", "住")]
    ),
    blend!(
        "zh",
        "ao",
        "zhao",
        [
            ("zhāo", "招"),
            ("zháo", "着"),
            ("zhǎo", "找"),
            ("zhào", "照")
        ]
    ),
    blend!(
        "ch",
        "i",
        "chi",
        [("chī", "吃"), ("chí", "迟"), ("chǐ", "尺"), ("chì", "翅")]
    ),
    blend!(
        "ch",
        "u",
        "chu",
        [("chū", "出"), ("chú", "除"), ("chǔ", "楚"), ("chù", "处")]
    ),
    blend!(
        "sh",
        "i",
        "shi",
        [("shī", "师"), ("shí", "十"), ("shǐ", "史"), ("shì", "是")]
    ),
    blend!(
        "sh",
        "e",
        "she",
        [("shē", "奢"), ("shé", "蛇"), ("shě", "舍"), ("shè", "社")]
    ),
    blend!(
        "sh",
        "u",
        "shu",
        [("shū", "书"), ("shú", "熟"), ("shǔ", "鼠"), ("shù", "树")]
    ),
    blend!(
        "sh",
        "ao",
        "shao",
        [
            ("shāo", "烧"),
            ("sháo", "勺"),
            ("shǎo", "少"),
            ("shào", "哨")
        ]
    ),
    blend!("r", "u", "ru", [("rú", "如"), ("rǔ", "乳"), ("rù", "入")]),
    blend!(
        "z",
        "ao",
        "zao",
        [("zāo", "遭"), ("záo", "凿"), ("zǎo", "早"), ("zào", "造")]
    ),
    blend!("c", "i", "ci", [("cí", "词"), ("cǐ", "此"), ("cì", "次")]),
    blend!("s", "i", "si", [("sī", "丝"), ("sǐ", "死"), ("sì", "四")]),
];

/// 声母韵母拼合表：每个声母能拼的全部韵母（按《汉语拼音音节表》）。
///
/// `BLENDS` 只收录了有四声例字的音节，这里保证「第二步选韵母」不会漏掉能拼的组合。
pub const BLEND_FINALS: &[(&str, &[&str])] = &[
    (
        "b",
        &[
            "a", "o", "ai", "ei", "ao", "an", "en", "ang", "eng", "i", "ie", "iao", "ian", "in",
            "ing", "u",
        ],
    ),
    (
        "p",
        &[
            "a", "o", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "i", "ie", "iao", "ian",
            "in", "ing", "u",
        ],
    ),
    (
        "m",
        &[
            "a", "o", "e", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "i", "ie", "iao",
            "iu", "ian", "in", "ing", "u",
        ],
    ),
    ("f", &["a", "o", "ei", "ou", "an", "en", "ang", "eng", "u"]),
    (
        "d",
        &[
            "a", "e", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "ong", "i", "ia", "ie",
            "iao", "iu", "ian", "ing", "u", "uo", "ui", "uan", "un",
        ],
    ),
    (
        "t",
        &[
            "a", "e", "ai", "ao", "ou", "an", "ang", "eng", "ong", "i", "ie", "iao", "ian", "ing",
            "u", "uo", "ui", "uan", "un",
        ],
    ),
    (
        "n",
        &[
            "a", "e", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "ong", "i", "ie", "iao",
            "iu", "ian", "in", "iang", "ing", "u", "uo", "uan", "ü", "üe",
        ],
    ),
    (
        "l",
        &[
            "a", "e", "ai", "ei", "ao", "ou", "an", "ang", "eng", "ong", "i", "ia", "ie", "iao",
            "iu", "ian", "in", "iang", "ing", "u", "uo", "uan", "un", "ü", "üe",
        ],
    ),
    (
        "g",
        &[
            "a", "e", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "ong", "u", "ua", "uo",
            "uai", "ui", "uan", "un", "uang",
        ],
    ),
    (
        "k",
        &[
            "a", "e", "ai", "ao", "ou", "an", "en", "ang", "eng", "ong", "u", "ua", "uo", "uai",
            "ui", "uan", "un", "uang",
        ],
    ),
    (
        "h",
        &[
            "a", "e", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "ong", "u", "ua", "uo",
            "uai", "ui", "uan", "un", "uang",
        ],
    ),
    (
        "j",
        &[
            "i", "ia", "ie", "iao", "iu", "ian", "in", "iang", "ing", "iong", "ü", "üe", "üan",
            "ün",
        ],
    ),
    (
        "q",
        &[
            "i", "ia", "ie", "iao", "iu", "ian", "in", "iang", "ing", "iong", "ü", "üe", "üan",
            "ün",
        ],
    ),
    (
        "x",
        &[
            "i", "ia", "ie", "iao", "iu", "ian", "in", "iang", "ing", "iong", "ü", "üe", "üan",
            "ün",
        ],
    ),
    (
        "zh",
        &[
            "a", "e", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "ong", "i", "u", "ua",
            "uo", "uai", "ui", "uan", "un", "uang",
        ],
    ),
    (
        "ch",
        &[
            "a", "e", "ai", "ao", "ou", "an", "en", "ang", "eng", "ong", "i", "u", "ua", "uo",
            "uai", "ui", "uan", "un", "uang",
        ],
    ),
    (
        "sh",
        &[
            "a", "e", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "i", "u", "ua", "uo",
            "uai", "ui", "uan", "un", "uang",
        ],
    ),
    (
        "r",
        &[
            "e", "ao", "ou", "an", "en", "ang", "eng", "ong", "i", "u", "uo", "ui", "uan", "un",
        ],
    ),
    (
        "z",
        &[
            "a", "e", "ai", "ei", "ao", "ou", "an", "en", "ang", "eng", "ong", "i", "u", "uo",
            "ui", "uan", "un",
        ],
    ),
    (
        "c",
        &[
            "a", "e", "ai", "ao", "ou", "an", "en", "ang", "eng", "ong", "i", "u", "uo", "ui",
            "uan", "un",
        ],
    ),
    (
        "s",
        &[
            "a", "e", "ai", "ao", "ou", "an", "en", "ang", "eng", "ong", "i", "u", "uo", "ui",
            "uan", "un",
        ],
    ),
];

/// 可以对孩子说的鼓励话。
pub const PRAISE: &[&str] = &["太棒了", "真聪明", "好厉害", "答对啦"];

/// 拼读练习里可用的声母顺序。
pub fn blend_initials() -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    for b in BLENDS {
        if !out.contains(&b.initial) {
            out.push(b.initial);
        }
    }
    out
}

/// 指定声母可拼的全部韵母（按拼合表，而不是只看有没有例字）。
pub fn blend_finals(initial: &str) -> Vec<&'static str> {
    if let Some((_, finals)) = BLEND_FINALS.iter().find(|(i, _)| *i == initial) {
        return finals.to_vec();
    }
    BLENDS
        .iter()
        .filter(|b| b.initial == initial)
        .map(|b| b.final_)
        .collect()
}

/// 声母 + 韵母拼出来的音节。`j / q / x` 遇到 `ü` 要去掉两点（jü → ju）。
pub fn syllable(initial: &str, final_: &str) -> String {
    if matches!(initial, "j" | "q" | "x")
        && let Some(rest) = final_.strip_prefix('ü')
    {
        return format!("{initial}u{rest}");
    }
    format!("{initial}{final_}")
}

pub fn find_blend(initial: &str, final_: &str) -> Option<&'static Blend> {
    BLENDS
        .iter()
        .find(|b| b.initial == initial && b.final_ == final_)
}

/// 四声齐全的音节，用于出题。
pub fn quiz_blends() -> Vec<&'static Blend> {
    BLENDS.iter().filter(|b| b.tones.len() == 4).collect()
}

// ---------------------------------------------------------------------------
// 声调
// ---------------------------------------------------------------------------

/// 六个带调元音，下标 0..=3 对应一、二、三、四声。
const TONED_VOWELS: [[char; 4]; 6] = [
    ['ā', 'á', 'ǎ', 'à'],
    ['ō', 'ó', 'ǒ', 'ò'],
    ['ē', 'é', 'ě', 'è'],
    ['ī', 'í', 'ǐ', 'ì'],
    ['ū', 'ú', 'ǔ', 'ù'],
    ['ǖ', 'ǘ', 'ǚ', 'ǜ'],
];

fn toned(base: char, tone: u8) -> char {
    let index = (tone.clamp(1, 4) - 1) as usize;
    match base {
        'a' => TONED_VOWELS[0][index],
        'o' => TONED_VOWELS[1][index],
        'e' => TONED_VOWELS[2][index],
        'i' => TONED_VOWELS[3][index],
        'u' => TONED_VOWELS[4][index],
        'ü' => TONED_VOWELS[5][index],
        _ => base,
    }
}

/// 标调位置：有 a/o/e 就标在它上面，否则标在靠后的 i/u 上（iu → iù、ui → uì）。
fn mark_index(chars: &[char]) -> Option<usize> {
    for vowel in ['a', 'o', 'e', 'ü'] {
        if let Some(index) = chars.iter().position(|&c| c == vowel) {
            return Some(index);
        }
    }
    if chars.contains(&'i') && chars.contains(&'u') {
        return chars.iter().rposition(|&c| c == 'i' || c == 'u');
    }
    chars
        .iter()
        .position(|&c| c == 'i')
        .or_else(|| chars.iter().position(|&c| c == 'u'))
}

/// 给不带调的拼音加上声调符号：`("ma", 3)` → `mǎ`。
pub fn with_tone(syllable: &str, tone: u8) -> String {
    let chars: Vec<char> = syllable.chars().collect();
    let Some(index) = mark_index(&chars) else {
        return syllable.to_string();
    };
    let mut out: String = chars[..index].iter().collect();
    out.push(toned(chars[index], tone));
    out.extend(chars[index + 1..].iter());
    out
}

/// 读出带调拼音是第几声，认不出来时返回 0。
pub fn tone_number(pinyin: &str) -> u8 {
    for c in pinyin.chars() {
        for marks in TONED_VOWELS.iter() {
            if let Some(index) = marks.iter().position(|&m| m == c) {
                return index as u8 + 1;
            }
        }
    }
    0
}

/// 韵母四声「取哪段录音」。
///
/// 韵母单独念的时候要写成零声母的形式：`i` 念作 `yi`、`ui` 念作 `wei`、`iu` 念作 `you`、
/// `un` 念作 `wen`……声音和韵母本身完全一样，只是拼音写法不同，所以录音用它、显示仍然写韵母。
pub fn final_tone_demo(final_: &str) -> Option<&'static str> {
    Some(match final_ {
        "a" => "a",
        "o" => "o",
        "e" => "e",
        "i" => "yi",
        "u" => "wu",
        "ü" => "yu",
        "ai" => "ai",
        "ei" => "ei",
        "ui" => "wei",
        "ao" => "ao",
        "ou" => "ou",
        "iu" => "you",
        "ie" => "ye",
        "üe" => "yue",
        "er" => "er",
        "an" => "an",
        "en" => "en",
        "in" => "yin",
        "un" => "wen",
        "ün" => "yun",
        "ang" => "ang",
        "eng" => "eng",
        "ing" => "ying",
        "ong" => "ong",
        _ => return None,
    })
}

/// 认一认里某个字母能不能做「四声练习」，返回用来取录音的音节。
pub fn tone_demo(tab: usize, text: &'static str) -> Option<&'static str> {
    match tab {
        1 => final_tone_demo(text),
        2 => Some(text),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// 家长教辅
// ---------------------------------------------------------------------------

/// 一个音的发音要领与常见错误（给大人看的）。
pub struct Tip {
    pub text: &'static str,
    pub mouth: &'static str,
    pub warn: &'static str,
}

pub const INITIAL_TIPS: &[Tip] = &[
    Tip {
        text: "b",
        mouth: "双唇闭紧，突然放开，气流轻轻冲出（不送气）",
        warn: "别读成拖长的「玻」，也不要和 p 混",
    },
    Tip {
        text: "p",
        mouth: "双唇闭紧，突然放开，一股较强的气流冲出来（送气）",
        warn: "送气不够就会听着像 b",
    },
    Tip {
        text: "m",
        mouth: "双唇闭紧，气流从鼻子出来，声带振动",
        warn: "嘴唇没闭住，尾巴会多出元音",
    },
    Tip {
        text: "f",
        mouth: "上齿轻贴下唇，气流从窄缝里擦出来",
        warn: "用了双唇，就会变成 h",
    },
    Tip {
        text: "d",
        mouth: "舌尖抵住上牙床，突然放开，不送气",
        warn: "舌尖太靠后就像 g",
    },
    Tip {
        text: "t",
        mouth: "舌尖抵住上牙床，突然放开，送气",
        warn: "和 d 只差一口气，要让孩子感受气流",
    },
    Tip {
        text: "n",
        mouth: "舌尖抵住上牙床，气流从鼻腔出来",
        warn: "最容易和 l 混（牛奶 niú nǎi）",
    },
    Tip {
        text: "l",
        mouth: "舌尖抵住上牙床，气流从舌头两边出来",
        warn: "气流跑到鼻子里就变成 n 了",
    },
    Tip {
        text: "g",
        mouth: "舌根抵住软腭，突然放开，不送气",
        warn: "位置太靠前会读成 d",
    },
    Tip {
        text: "k",
        mouth: "舌根抵住软腭，突然放开，送气",
        warn: "送气不足就像 g",
    },
    Tip {
        text: "h",
        mouth: "舌根靠近软腭，气流从缝里擦出来",
        warn: "摩擦太用力会像在清嗓子",
    },
    Tip {
        text: "j",
        mouth: "舌面前部贴住硬腭，不送气",
        warn: "别和 zh、z 混，舌尖要放下",
    },
    Tip {
        text: "q",
        mouth: "舌面姿势同 j，但要送气",
        warn: "送气不足就听着像 j",
    },
    Tip {
        text: "x",
        mouth: "舌面前部靠近硬腭，气流擦出来",
        warn: "舌尖顶住牙齿会变成 s",
    },
    Tip {
        text: "zh",
        mouth: "舌尖上翘顶住硬腭前部，不送气",
        warn: "舌尖放平就变成 z（四是四，十是十）",
    },
    Tip {
        text: "ch",
        mouth: "姿势同 zh，送气",
        warn: "和 c 混是常见问题",
    },
    Tip {
        text: "sh",
        mouth: "舌尖上翘靠近硬腭，气流擦出来",
        warn: "和 s 混时，先练「翘舌」再辨音",
    },
    Tip {
        text: "r",
        mouth: "姿势同 sh，但声带要振动",
        warn: "声带不振动就变成 sh 了",
    },
    Tip {
        text: "z",
        mouth: "舌尖平放抵住上齿背，不送气",
        warn: "舌尖翘起来就变成 zh",
    },
    Tip {
        text: "c",
        mouth: "姿势同 z，送气",
        warn: "和 ch 混时看舌尖是不是平的",
    },
    Tip {
        text: "s",
        mouth: "舌尖靠近上齿背，气流从窄缝擦出",
        warn: "和 sh 混时注意嘴角向两边",
    },
    Tip {
        text: "y",
        mouth: "就是 i，读「衣」，嘴角向两边展开",
        warn: "别拖成很长的「衣——」",
    },
    Tip {
        text: "w",
        mouth: "就是 u，双唇收圆前突，读「乌」",
        warn: "嘴唇不圆就会走音",
    },
];

pub const FINAL_TIPS: &[Tip] = &[
    Tip {
        text: "a",
        mouth: "嘴巴张大，舌头放平，读「啊」",
        warn: "嘴张不够大，声音就闷",
    },
    Tip {
        text: "o",
        mouth: "嘴唇收圆，舌根后缩，读「哦」",
        warn: "容易滑成 ou 或 wo",
    },
    Tip {
        text: "e",
        mouth: "嘴巴半开，嘴角微展，嘴唇不圆，读「鹅」",
        warn: "和 o 只差圆不圆唇",
    },
    Tip {
        text: "i",
        mouth: "嘴角咧开，上下牙齿对齐，读「衣」",
        warn: "单独念 i 时其实读 yi",
    },
    Tip {
        text: "u",
        mouth: "嘴唇收圆前突，读「乌」",
        warn: "嘴唇没圆，声音就散",
    },
    Tip {
        text: "ü",
        mouth: "嘴唇收圆，同时舌面前伸，读「鱼」",
        warn: "读成 u 是幼儿最常见的错误（雨→五）",
    },
    Tip {
        text: "ai",
        mouth: "从 a 滑到 i，一口气连起来",
        warn: "拆成两个音就不成音节了",
    },
    Tip {
        text: "ei",
        mouth: "从 e 滑到 i",
        warn: "和 ai 混时先看口型：ai 嘴张得大",
    },
    Tip {
        text: "ui",
        mouth: "从 u 滑到 i，先圆唇再展唇",
        warn: "容易漏掉前面的 u",
    },
    Tip {
        text: "ao",
        mouth: "从 a 滑到 o，嘴由大到圆",
        warn: "滑成 u 就变成「欧」",
    },
    Tip {
        text: "ou",
        mouth: "从 o 滑到 u",
        warn: "和 ao 混时先确认起点口型",
    },
    Tip {
        text: "iu",
        mouth: "从 i 滑到 u",
        warn: "它是 iou 的缩写，别念成「优」加 u",
    },
    Tip {
        text: "ie",
        mouth: "从 i 快速滑到 e",
        warn: "和 ye 是同一个音，只是写法不同",
    },
    Tip {
        text: "üe",
        mouth: "从 ü 滑到 e，嘴唇一直圆着",
        warn: "漏掉两点就读成 ue 了",
    },
    Tip {
        text: "er",
        mouth: "舌面放平，舌尖卷起来，读「儿」",
        warn: "卷舌不够就成了 e",
    },
    Tip {
        text: "an",
        mouth: "先发 a，舌尖抬起抵上牙床带出鼻音",
        warn: "和 ang 混时看舌尖有没有抵上去",
    },
    Tip {
        text: "en",
        mouth: "先发 e，加前鼻音 n",
        warn: "嘴张太大就滑成 an",
    },
    Tip {
        text: "in",
        mouth: "先发 i，加前鼻音 n",
        warn: "和 ing 混时注意舌根有没有抬起",
    },
    Tip {
        text: "un",
        mouth: "先发 u，加前鼻音 n",
        warn: "别读成 ün（云）",
    },
    Tip {
        text: "ün",
        mouth: "先发 ü，加前鼻音 n",
        warn: "单独念时写作 yun",
    },
    Tip {
        text: "ang",
        mouth: "先发 a，舌根抬起带出后鼻音 ng",
        warn: "和 an 只差一个舌根位置",
    },
    Tip {
        text: "eng",
        mouth: "先发 e，舌根抬起带出后鼻音 ng",
        warn: "和 en 混时听结尾的鼻音长短",
    },
    Tip {
        text: "ing",
        mouth: "先发 i，舌根抬起带出后鼻音 ng",
        warn: "和 in 混时注意结尾共鸣在口腔后部",
    },
    Tip {
        text: "ong",
        mouth: "先发 o 再快速带 u，舌根抬起收 ng",
        warn: "和 eng 混时先看起点口型是不是圆的",
    },
];

/// 某个音（按「认一认」的分类）的发音要领。
pub fn tip(tab: usize, text: &str) -> Option<&'static Tip> {
    let list = match tab {
        0 => INITIAL_TIPS,
        1 => FINAL_TIPS,
        _ => return None,
    };
    list.iter().find(|t| t.text == text)
}

/// 给家长的教学建议：(标题, 说明)。
pub const HOW_TO: &[(&str, &str)] = &[
    (
        "每天十分钟",
        "幼儿注意力短，一次 5–10 分钟、3–5 个音就够，宁少而稳，不要一口气教完。",
    ),
    (
        "先听再读",
        "先点卡片听标准音，孩子跟读两遍，再说例词，最后才让孩子自己读。",
    ),
    (
        "示范，而不是纠错",
        "读错了先慢慢示范一次，让孩子模仿；反复说“不对”，孩子会不敢开口。",
    ),
    (
        "拼读慢一点",
        "声母读得轻短、韵母读得响亮，先慢速分开，再连起来。",
    ),
    (
        "四声可以用手势",
        "一声手平举，二声手上扬，三声手先下后上，四声手往下压，边比边读。",
    ),
    (
        "答错也是练习",
        "玩一玩答错很正常，让孩子再听一遍再选，不要急着替他点。",
    ),
    (
        "小星星做奖励",
        "答对一颗星，攒到十颗给一个小奖励，比一次学很多更能坚持。",
    ),
];

/// 拼读口诀。
pub const BLEND_RHYME: &str = "前音轻短后音重，两音相连猛一碰。";

/// 极简 xorshift 随机数，避免为了抽题引入额外依赖。
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        })
    }

    pub fn from_time() -> Self {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x1234_5678);
        Self::new(nanos)
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next_u64() % n as u64) as usize
        }
    }

    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.below(i + 1);
            items.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tone_mark_goes_on_the_right_vowel() {
        assert_eq!(with_tone("ma", 3), "mǎ");
        assert_eq!(with_tone("bei", 4), "bèi");
        assert_eq!(with_tone("gui", 1), "guī");
        assert_eq!(with_tone("liu", 2), "liú");
        assert_eq!(with_tone("chun", 1), "chūn");
        assert_eq!(with_tone("dong", 1), "dōng");
        assert_eq!(with_tone("yue", 4), "yuè");
        assert_eq!(with_tone("yun", 2), "yún");
        assert_eq!(with_tone("ying", 1), "yīng");
        assert_eq!(with_tone("er", 2), "ér");
    }

    /// 四声练习显示韵母本身，不加 y / w / 声母。
    #[test]
    fn tone_row_shows_the_final_itself() {
        assert_eq!(with_tone("i", 2), "í");
        assert_eq!(with_tone("ü", 3), "ǚ");
        assert_eq!(with_tone("ie", 4), "iè");
        assert_eq!(with_tone("üe", 1), "üē");
        assert_eq!(with_tone("ing", 1), "īng");
        assert_eq!(with_tone("ün", 2), "ǘn");
        assert_eq!(with_tone("ui", 1), "uī");
        assert_eq!(with_tone("iu", 3), "iǔ");
        assert_eq!(with_tone("un", 4), "ùn");
        assert_eq!(with_tone("ong", 1), "ōng");
        // 录音用零声母的写法，声音和韵母本身一样
        assert_eq!(final_tone_demo("i"), Some("yi"));
        assert_eq!(final_tone_demo("ui"), Some("wei"));
        assert_eq!(final_tone_demo("iu"), Some("you"));
        assert_eq!(final_tone_demo("un"), Some("wen"));
        // 声母没有四声
        assert_eq!(tone_demo(0, "b"), None);
    }

    #[test]
    fn tone_number_reads_back() {
        for tone in 1..=4 {
            assert_eq!(tone_number(&with_tone("pao", tone)), tone);
        }
        assert_eq!(tone_number("bā"), 1);
        assert_eq!(tone_number("ba"), 0);
    }

    /// 每个韵母的四声示范都必须有录音，否则界面上会出现点不响的按钮。
    #[test]
    fn every_final_has_four_tones() {
        for letter in FINALS {
            let demo = final_tone_demo(letter.text).expect(letter.text);
            for tone in 1..=4 {
                assert!(crate::audio::has_tone(demo, tone), "{demo}{tone}");
            }
        }
    }

    /// 每个字母的示范音都要有录音。
    #[test]
    fn every_letter_has_a_recording() {
        for (_, letters) in TABS {
            for letter in *letters {
                assert!(crate::audio::has_letter(letter.text), "{}", letter.text);
            }
        }
    }

    /// 拼一拼要列出每个声母能拼的全部韵母，不能只看有没有例字。
    #[test]
    fn every_initial_lists_all_its_finals() {
        for (initial, finals) in BLEND_FINALS {
            assert!(!finals.is_empty(), "{initial}");
            assert_eq!(blend_finals(initial).len(), finals.len(), "{initial}");
        }
        // b 不只有 a / i / ao
        assert!(blend_finals("b").contains(&"u"));
        assert!(blend_finals("b").contains(&"ie"));
        // j / q / x 能和 ü 系列相拼
        assert!(blend_finals("j").contains(&"ü"));
        assert!(blend_finals("j").contains(&"üe"));
        // 有例字的音节必须落在拼合表里
        for blend in BLENDS {
            assert!(
                blend_finals(blend.initial).contains(&blend.final_),
                "{} {}",
                blend.initial,
                blend.final_
            );
        }
    }

    /// 拼出来的音节：j / q / x 遇 ü 去两点，n / l 保留两点。
    #[test]
    fn initials_and_finals_spell_correctly() {
        assert_eq!(syllable("b", "a"), "ba");
        assert_eq!(syllable("zh", "uang"), "zhuang");
        assert_eq!(syllable("j", "ü"), "ju");
        assert_eq!(syllable("j", "üe"), "jue");
        assert_eq!(syllable("q", "üan"), "quan");
        assert_eq!(syllable("x", "ün"), "xun");
        assert_eq!(syllable("n", "ü"), "nü");
        assert_eq!(syllable("l", "üe"), "lüe");
        assert_eq!(with_tone("jue", 4), "juè");
        assert_eq!(with_tone("lüe", 4), "lüè");
    }
}
