use std::env;
use std::path::PathBuf;

use windows::Win32::UI::Accessibility::STICKYKEYS_FLAGS;
use windows::Win32::UI::WindowsAndMessaging::{SPI_SETSTICKYKEYS, SystemParametersInfoW};

use windows::Win32::UI::Accessibility::STICKYKEYS;

use crate::commands::execute_powershell_command;

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

pub fn clear_recent_files() {
    const COMMANDS: &[&'static str] = &[
        r#"Remove-Item -Path "$env:APPDATA\Microsoft\Windows\Recent\*" -Recurse -Force -ErrorAction SilentlyContinue"#,
        r#"Remove-Item -Path "$env:APPDATA\Microsoft\Windows\Recent\AutomaticDestinations\*" -Recurse -Force -ErrorAction SilentlyContinue"#,
        r#"Remove-Item -Path "$env:APPDATA\Microsoft\Windows\Recent\CustomDestinations\*" -Recurse -Force -ErrorAction SilentlyContinue"#,
    ];
    match execute_powershell_command(&COMMANDS[..]) {
        Ok(_) => {
            log::info!("Cleared Recent Files")
        }
        Err(e) => log::error!("{e}"),
    }
}

pub fn restart_explorer() {
    const COMMANDS: &[&'static str] = &[
        r#"Stop-Process -Name explorer -Force"#,
        r#"Start-Process -FilePath "explorer.exe" -ArgumentList "/n,/e,/select,::{20D04FE0-3AEA-1069-A2D8-08002B30309D}" -WindowStyle Hidden"#,
    ];
    match execute_powershell_command(&COMMANDS[..]) {
        Ok(_) => {
            log::info!("Restarted explorer")
        }
        Err(e) => log::error!("{e}"),
    }
}

pub fn does_program_registry_exist(program_name: &str) -> bool {
    let paths = [
        r#"HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*"#,
        r#"HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\*"#,
        r#"HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\*"#,
    ];

    for path in paths {
        let cmd = format!(
            r#"Get-ItemProperty -Path "{}" | Where-Object {{ $_.DisplayName -like '*{}*' }} | Select-Object -First 1"#,
            path, program_name
        );

        if let Ok(output) = execute_powershell_command(&[&cmd]) {
            if output.status.success() {
                if !String::from_utf8_lossy(&output.stdout).trim().is_empty() {
                    return true;
                }
            }
        }
    }

    false
}

/// Checks if the given path exists on the current system using PowerShell.
pub fn does_path_exist(path: &str) -> bool {
    let cmd = format!(r#"Test-Path -Path "{}""#, path);

    match execute_powershell_command(&[&cmd]) {
        Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout)
            .trim()
            .eq_ignore_ascii_case("True"),
        _ => false,
    }
}

/// Checks if a given program path exists on any available drive (C:\, D:\, etc.).
pub fn does_program_path_exist_on_any_drive(program_path: &str) -> bool {
    // List of common Windows drive letters
    let drives = [
        "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
        "U", "V", "W", "X", "Y", "Z",
    ];

    for drive in drives.iter() {
        let full_path = format!(r"{}:\{}", drive, program_path.trim_start_matches('\\'));
        if does_path_exist(&full_path) {
            return true;
        }
    }

    false
}

pub fn does_program_exist(program_name: &str) -> bool {
    if does_program_registry_exist(program_name) {
        return true;
    }
    let common_dirs = [
        env::var("ProgramFiles").ok(),
        env::var("ProgramFiles(x86)").ok(),
        env::var("LocalAppData").ok(),
        env::var("AppData").ok(),
    ];

    for dir in common_dirs.iter().flatten() {
        let path = PathBuf::from(dir).join(program_name);
        if does_program_path_exist_on_any_drive(path.to_str().unwrap_or_default()) {
            return true;
        }
    }

    // Optionally, you can also check "C:\ProgramData"
    if does_program_path_exist_on_any_drive(&format!(r"ProgramData\{}", program_name)) {
        return true;
    }

    false
}
