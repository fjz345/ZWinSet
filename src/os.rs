#[cfg(target_os = "windows")]
use crate::windows::does_windows_program_exist;

pub fn does_program_exist(program_name: &str) -> bool {
    #[cfg(target_os = "windows")]
    return does_windows_program_exist(program_name);
    #[cfg(target_os = "macos")]
    return false;

    panic!();
}
