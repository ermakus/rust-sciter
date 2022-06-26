#[cfg(all(target_os = "windows" , target_arch = "x86"))]
mod video_wx86;

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub use self::video_wx86::*;

#[cfg(not(all(target_os = "windows", target_arch = "x86")))]
mod video_common;

#[cfg(not(all(target_os = "windows", target_arch = "x86")))]
pub use self::video_common::*;
