use std::{
    ffi::{OsStr, OsString},
    io,
    process::{Command, Output},
};

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
    let pre_args = &["-NoProfile", "-Command"];
    let mut all_args: Vec<OsString> = pre_args.iter().map(|s| OsString::from(*s)).collect();
    all_args.extend(args.into_iter().map(|s| s.as_ref().to_os_string()));

    Command::new("powershell").args(all_args).output()
}

pub fn test_cmd() -> Output {
    let result =
        execute_powershell_command(&["Get-ComputerInfo | Select-Object -Property WindowsVersion"])
            .unwrap_or_else(|e| panic!("{e}"));
    result
}
