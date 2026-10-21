//! `xattr` 的 Android 兼容空实现。
//!
//! 为什么要这一层：
//!
//! `gpui` 无条件依赖 `http_client`，`http_client` 又依赖 `async-tar`，
//! `async-tar` 在 `cfg(unix)` 下调 `xattr::set` 设置扩展属性。Android 也属于 unix，
//! 但 bionic 的 `libc` 没有 `ENOATTR`，`xattr 0.2.3` 直接编译失败：
//!
//! ```text
//! error[E0425]: cannot find value `ENOATTR` in crate `::libc`
//! ```
//!
//! 我们的界面对 tar 解包只用不到（没有下载、没有安装包解压），
//! 所以这里提供一个同签名的空实现，让链接和类型检查都能过。
//! `set` 直接返回 `Ok(())`，因为「设置扩展属性」失败与否不影响 UI。

use std::ffi::{OsStr, OsString};
use std::io;
use std::path::Path;

pub fn get<P: AsRef<Path>, N: AsRef<OsStr>>(_path: P, _name: N) -> io::Result<Option<Vec<u8>>> {
    Ok(None)
}

pub fn set<P: AsRef<Path>, N: AsRef<OsStr>, V: AsRef<[u8]>>(
    _path: P,
    _name: N,
    _value: V,
) -> io::Result<()> {
    Ok(())
}

pub fn list<P: AsRef<Path>>(_path: P) -> io::Result<Vec<OsString>> {
    Ok(Vec::new())
}

pub fn remove<P: AsRef<Path>, N: AsRef<OsStr>>(_path: P, _name: N) -> io::Result<()> {
    Ok(())
}
