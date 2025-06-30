use std::{
    ffi::OsStr,
    io,
    process::{Command, Output},
};

use base64::{engine::general_purpose, Engine};

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

pub fn test_cmd() -> Output {
    let result =
        execute_powershell_command(&["Get-ComputerInfo | Select-Object -Property WindowsVersion"])
            .unwrap_or_else(|e| panic!("{e}"));
    result
}
