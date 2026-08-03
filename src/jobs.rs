pub mod job;
pub mod platform_jobs;

#[cfg(target_os = "linux")]
pub mod linux_jobs;
#[cfg(target_os = "macos")]
pub mod macos_jobs;
#[cfg(target_os = "windows")]
pub mod windows_jobs;
