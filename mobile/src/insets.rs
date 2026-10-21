//! 顶部安全区（状态栏高度）。
//!
//! Android 15+ 强制 edge-to-edge，NativeActivity 的 surface 是全屏的，
//! 界面会画到状态栏下面（时钟压住标题）。gpui-mobile 的
//! `AndroidWindow::safe_area_insets_logical()` 拿不到，所以在启动时用 JNI 问一次
//! `getRootWindowInsets()`，把结果存成逻辑点（dp），界面加一个 `pt()` 就行。

use std::sync::atomic::{AtomicU32, Ordering};

static TOP: AtomicU32 = AtomicU32::new(0);

/// 顶部安全区，逻辑点（dp）。
pub fn top() -> f32 {
    f32::from_bits(TOP.load(Ordering::Relaxed))
}

/// 启动时调用一次；失败就保持 0，界面顶到最上面（和没做时一样）。
#[cfg(target_os = "android")]
pub fn init() {
    match query() {
        Some(top) => {
            log::info!("insets: 顶部安全区 {top}dp");
            TOP.store(top.to_bits(), Ordering::Relaxed);
        }
        None => log::warn!("insets: 拿不到状态栏高度，顶部不留白"),
    }
}

#[cfg(not(target_os = "android"))]
pub fn init() {}

#[cfg(target_os = "android")]
fn query() -> Option<f32> {
    use jni::objects::{JObject, JValue};
    use jni::JavaVM;

    let ctx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()).ok()? };
    let mut env = vm.attach_current_thread_permanently().ok()?;
    let activity = unsafe { JObject::from_raw(ctx.context().cast()) };

    // 用系统的 status_bar_height 资源，比 getRootWindowInsets 稳：
    // 后者在窗口还没 attach 时是 null（启动那一刻正好拿不到）。
    let res = env
        .call_method(
            &activity,
            "getResources",
            "()Landroid/content/res/Resources;",
            &[],
        )
        .ok()?
        .l()
        .ok()?;

    let name = env.new_string("status_bar_height").ok()?;
    let dimen = env.new_string("dimen").ok()?;
    let pkg = env.new_string("android").ok()?;
    let id = env
        .call_method(
            &res,
            "getIdentifier",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)I",
            &[
                JValue::Object(&name),
                JValue::Object(&dimen),
                JValue::Object(&pkg),
            ],
        )
        .ok()?
        .i()
        .ok()?;
    if id <= 0 {
        return None;
    }

    let top_px = env
        .call_method(&res, "getDimensionPixelSize", "(I)I", &[JValue::Int(id)])
        .ok()?
        .i()
        .ok()?;
    if top_px <= 0 {
        return None;
    }

    let metrics = env
        .call_method(
            &res,
            "getDisplayMetrics",
            "()Landroid/util/DisplayMetrics;",
            &[],
        )
        .ok()?
        .l()
        .ok()?;
    let density: f32 = env.get_field(&metrics, "density", "F").ok()?.f().ok()?;
    if density <= 0.0 {
        return None;
    }

    Some(top_px as f32 / density)
}
