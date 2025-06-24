use windows::Win32::UI::Accessibility::STICKYKEYS_FLAGS;
use windows::Win32::UI::WindowsAndMessaging::{SPI_SETSTICKYKEYS, SystemParametersInfoW};

use windows::Win32::UI::Accessibility::STICKYKEYS;

pub fn disable_sticky_keys() {
    let mut sk = STICKYKEYS {
        cbSize: std::mem::size_of::<STICKYKEYS>() as u32,
        dwFlags: STICKYKEYS_FLAGS(0), // disables Sticky Keys
    };

    unsafe {
        let res = SystemParametersInfoW(
            SPI_SETSTICKYKEYS,
            sk.cbSize,
            Some(&mut sk as *mut _ as *mut _),
            windows::Win32::UI::WindowsAndMessaging::SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        );

        if !res.is_ok() {
            log::error!("Failed to apply Sticky Keys settings");
        }
    }
}
