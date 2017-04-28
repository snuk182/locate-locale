#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(any(target_os = "windows", target_os = "android")))]
mod unix;

#[cfg(target_os = "windows")]
pub use windows::{user, system};

#[cfg(not(any(target_os = "windows", target_os = "android")))]
pub use unix::{user, system};
