pub use self::platform::*;

#[cfg(target_os = "windows")]
#[path="windows/mod.rs"]
mod platform;
#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
#[path="linux/mod.rs"]
mod platform;
#[cfg(target_os = "macos")]
#[path="macos/mod.rs"]
mod platform;
#[cfg(target_os = "android")]
#[path="android/mod.rs"]
mod platform;
#[cfg(target_os = "ios")]
#[path="ios/mod.rs"]
mod platform;
