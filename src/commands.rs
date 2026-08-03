use std::{
    ffi::OsStr,
    io,
    process::{Command, Output},
};

use base64::{Engine, engine::general_purpose};

fn asd() {
    // Example PowerShell command to get the Windows version
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "Get-ComputerInfo | Select-Object -Property WindowsVersion",
        ])
        .output()
        .expect("Failed to execute PowerShell command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("PowerShell output:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("PowerShell error:\n{}", stderr);
    }
}

#[cfg(not(target_os = "windows"))]
pub fn execute_unix_command<I, S>(args: I) -> io::Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let script_content = args
        .into_iter()
        .map(|s| s.as_ref().to_string_lossy().into_owned())
        .collect::<Vec<String>>()
        .join("\n");

    Command::new("sh").arg("-c").arg(script_content).output()
}

#[cfg(not(target_os = "windows"))]
pub fn execute_unix_as_admin<I, S>(args: I) -> io::Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let script_content = args
        .into_iter()
        .map(|s| s.as_ref().to_string_lossy().into_owned())
        .collect::<Vec<String>>()
        .join("\n");

    #[cfg(target_os = "macos")]
    {
        let osa_script = format!(
            "do shell script \"{}\" with administrator privileges",
            script_content.replace('\\', "\\\\").replace('"', "\\\"")
        );

        Command::new("osascript").arg("-e").arg(osa_script).output()
    }

    #[cfg(not(target_os = "macos"))]
    {
        // On Linux, use pkexec if a graphical environment is detected, otherwise fall back to sudo
        let use_pkexec =
            std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok();

        if use_pkexec {
            Command::new("pkexec")
                .arg("sh")
                .arg("-c")
                .arg(&script_content)
                .output()
        } else {
            Command::new("sudo")
                .arg("sh")
                .arg("-c")
                .arg(&script_content)
                .output()
        }
    }
}

#[cfg(target_os = "windows")]
pub fn execute_powershell_command<I, S>(args: I) -> io::Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let script_content = args
        .into_iter()
        .map(|s| s.as_ref().to_string_lossy().into_owned())
        .collect::<Vec<String>>()
        .join("\n");

    let script_bytes_utf16le: Vec<u8> = script_content
        .encode_utf16()
        .flat_map(|u| u.to_le_bytes().to_vec())
        .collect();

    let encoded_script = general_purpose::STANDARD.encode(&script_bytes_utf16le);
    // Not sure why, but -EncodedCommand is needed
    Command::new("powershell")
        .args(&["-NoProfile", "-EncodedCommand", &encoded_script])
        .output()
}

#[cfg(target_os = "windows")]
pub fn execute_powershell_as_admin<I, S>(args: I) -> io::Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let script_content = args
        .into_iter()
        .map(|s| s.as_ref().to_string_lossy().into_owned())
        .collect::<Vec<String>>()
        .join("\n"); // Join with newlines to form a proper script

    let script_bytes_utf16le: Vec<u8> = script_content
        .encode_utf16()
        .flat_map(|u| u.to_le_bytes().to_vec())
        .collect();

    let encoded_script = general_purpose::STANDARD.encode(&script_bytes_utf16le);
    // Not sure why, but -EncodedCommand is needed
    let full_command = format!(
        "Start-Process powershell -Verb RunAs -ArgumentList '-NoProfile', '-EncodedCommand', '{}'",
        encoded_script
    );

    Command::new("powershell")
        .args(&["-NoProfile", "-Command", &full_command])
        .output()
}
