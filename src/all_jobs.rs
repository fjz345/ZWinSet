use crate::jobs::{InstallApplicationCtx, Job, JobCategory, PowerShellCtx};

pub static ALL_JOBS: &[Job] = &[
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Dev Test",
        category: JobCategory::Etc,
        list_of_commands: &[
            "Start-Sleep -Milliseconds 200",
            "Start-Sleep -Milliseconds 200",
            "Start-Sleep -Milliseconds 200",
            "Start-Sleep -Milliseconds 200",
            "Start-Sleep -Milliseconds 200",
            "Start-Sleep -Milliseconds 200",
            "Start-Sleep -Milliseconds 200",
            "Start-Sleep -Milliseconds 200",
            "Start-Sleep -Milliseconds 200",
            "Start-Sleep -Milliseconds 200",
        ],
        name: "Test",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "InstallDiscord",
        category: JobCategory::Application,
        list_of_commands: &["asd", "ASd"],
        name: "Discord",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Windows mouse acceleration is on by default",
        category: JobCategory::Windows,
        list_of_commands: &[r#"
# Set registry values
Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseSpeed" -Value "0"
Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseThreshold1" -Value "0"
Set-ItemProperty -Path "HKCU:\Control Panel\Mouse" -Name "MouseThreshold2" -Value "0"

# Define constants
$SPI_SETMOUSE = 0x0004
$SPIF_UPDATEINIFILE = 0x01
$SPIF_SENDCHANGE = 0x02

# Parameters: [threshold1, threshold2, acceleration]
$mouseParams = @(0, 0, 0)

# Load native method once
Add-Type @"
using System;
using System.Runtime.InteropServices;
public class NativeMethods {
    [DllImport("user32.dll", SetLastError = true)]
    public static extern bool SystemParametersInfo(
        uint uiAction, uint uiParam, int[] pvParam, uint fWinIni);
}
"@

# Apply mouse settings
[NativeMethods]::SystemParametersInfo($SPI_SETMOUSE, 0, $mouseParams, $SPIF_UPDATEINIFILE -bor $SPIF_SENDCHANGE)
            "#],
        name: "Mouse Acceleration",
        require_admin: false,
    }),
    Job::InstallApplication(InstallApplicationCtx {
        explination: "Install Steam",
        category: JobCategory::Application,
        application_name: "Steam",
        name: "Steam",
        require_admin: false,
    }),
];
