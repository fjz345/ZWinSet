use crate::jobs::job::Job;
#[cfg(target_os = "linux")]
pub use crate::jobs::linux_jobs::LinuxJobExecutor as CurrentPlatformJob;

#[cfg(target_os = "windows")]
pub use crate::jobs::windows_jobs::WindowsJobExecutor as CurrentPlatformJob;

use crate::jobs::macos_jobs::MACOS_JOBS;
// pub fn get_platform_jobs() -> impl Iterator<Item = Job> {
//     panic!();
//     //CurrentPlatformJob::all_jobs().into_iter()
// }

pub fn get_all_jobs() -> impl Iterator<Item = Job> {
    #[cfg(target_os = "windows")]
    {
        WINDOWS_JOBS.iter().cloned()
    }

    #[cfg(target_os = "macos")]
    {
        MACOS_JOBS.iter().cloned()
    }

    #[cfg(target_os = "linux")]
    {
        LINUX_JOBS.iter().cloned()
    }
}
