use crate::jobs::{
    InstallApplicationCtx, Job, JobCategory, PowerShellCtx, PowerShellRegKeyCtx, RegKey, RegKeyType,
};

pub static ALL_JOBS: &[Job] = &[
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Dev Test, 2 sec duration",
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
        explination: "Install Google Chrome (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            r#"$chromeInstaller = "$env:TEMP\chrome_installer.exe""#,
            r#"$chromeInstaller = "$env:TEMP\chrome_installer.exe";Invoke-WebRequest -Uri "https://dl.google.com/chrome/install/latest/chrome_installer.exe" -OutFile $chromeInstaller"#,
            r#"$chromeInstaller = "$env:TEMP\chrome_installer.exe"Start-Process -FilePath $chromeInstaller -ArgumentList "/silent", "/install" -NoNewWindow -Wait"#,
            r#"$chromeInstaller = "$env:TEMP\chrome_installer.exe"Remove-Item $chromeInstaller"#,
        ],
        name: "Chrome",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Steam (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            r#"$steamInstaller = "$env:TEMP\steam_installer.exe""#,
            r#"$steamInstaller = "$env:TEMP\steam_installer.exe";Invoke-WebRequest -Uri "https://cdn.cloudflare.steamstatic.com/client/installer/SteamSetup.exe" -OutFile $steamInstaller"#,
            r#"$steamInstaller = "$env:TEMP\steam_installer.exe";Start-Process -FilePath $steamInstaller -ArgumentList "/S" -NoNewWindow -Wait"#,
            r#"$steamInstaller = "$env:TEMP\steam_installer.exe";Remove-Item $steamInstaller"#,
        ],
        name: "Steam",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Discord (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            r#"$discordInstaller = "$env:TEMP\DiscordSetup.exe""#,
            r#"$discordInstaller = "$env:TEMP\DiscordSetup.exe";Invoke-WebRequest -Uri "https://discord.com/api/download?platform=win" -OutFile $discordInstaller"#,
            r#"$discordInstaller = "$env:TEMP\DiscordSetup.exe";Start-Process -FilePath $discordInstaller -ArgumentList "/S" -NoNewWindow -Wait"#,
            r#"$discordInstaller = "$env:TEMP\DiscordSetup.exe";Remove-Item $discordInstaller"#,
        ],
        name: "Discord",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Nvidia App (Drivers) (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            r#"irm https://github.com/emilwojcik93/Install-NvidiaApp/releases/latest/download/Install-NvidiaApp.ps1 | iex"#,
            r#"Install-NvidiaApp.ps1 -Edition Public -SilentInstall -SkipCheck"#,
        ],
        name: "Nvidia App",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Nvidia Broadcast (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            r#"$broadcastInstaller = "$env:TEMP\NvidiaBroadcastInstaller.exe""#,
            r#"$broadcastInstaller = "$env:TEMP\NvidiaBroadcastInstaller.exe";Invoke-WebRequest -Uri "https://developer.nvidia.com/compute/broadcast-sdk/redist/nvidia_broadcast_app_latest.exe" -OutFile $broadcastInstaller"#,
            r#"$broadcastInstaller = "$env:TEMP\NvidiaBroadcastInstaller.exe";Start-Process -FilePath $broadcastInstaller -ArgumentList "/silent" -Wait"#,
            r#"$broadcastInstaller = "$env:TEMP\NvidiaBroadcastInstaller.exe";Remove-Item $broadcastInstaller"#,
        ],
        name: "Nvidia Broadcast",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Logitech Hub (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            r#"$logitechInstaller = "$env:TEMP\lghub_installer.exe""#,
            r#"$logitechInstaller = "$env:TEMP\lghub_installer.exe";Invoke-WebRequest -Uri "https://downloads.logitech.com/pub/gaming/lghub_installer.exe" -OutFile $logitechInstaller"#,
            r#"$logitechInstaller = "$env:TEMP\lghub_installer.exe";Start-Process -FilePath $logitechInstaller -ArgumentList "--silent" -Wait"#,
            r#"$logitechInstaller = "$env:TEMP\lghub_installer.exe";Remove-Item $logitechInstaller"#,
        ],
        name: "Logitech Hub",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Visual Code (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            r#"$vsCodeInstaller = "$env:TEMP\VSCodeUserSetup.exe""#,
            r#"$vsCodeInstaller = "$env:TEMP\VSCodeUserSetup.exe";Invoke-WebRequest -Uri "https://update.code.visualstudio.com/latest/win32-x64-user/stable" -OutFile $vsCodeInstaller"#,
            r#"$vsCodeInstaller = "$env:TEMP\VSCodeUserSetup.exe";Start-Process -FilePath $vsCodeInstaller -ArgumentList "/silent", "/mergetasks=!runcode" -Wait"#,
            r#"$vsCodeInstaller = "$env:TEMP\VSCodeUserSetup.exe";Remove-Item $vsCodeInstaller"#,
        ],
        name: "Visual Code",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Mullvad VPN (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            r#"$mullvadInstaller = "$env:TEMP\MullvadVPN.exe""#,
            r#"$mullvadInstaller = "$env:TEMP\MullvadVPN.exe";Invoke-WebRequest -Uri "https://mullvad.net/download/app/win/latest/" -OutFile $mullvadInstaller"#,
            r#"$mullvadInstaller = "$env:TEMP\MullvadVPN.exe";Start-Process -FilePath $mullvadInstaller -ArgumentList "/S" -Wait"#,
            r#"$mullvadInstaller = "$env:TEMP\MullvadVPN.exe";Remove-Item $mullvadInstaller"#,
        ],
        name: "Mullvad",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Helix (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            r#"$helixZip = "$env:TEMP\helix.zip";$installDir = "$env:LOCALAPPDATA\Programs\helix""#,
            r#"$helixZip = "$env:TEMP\helix.zip";$installDir = "$env:LOCALAPPDATA\Programs\helix";Invoke-WebRequest -Uri "https://downloads.logitech.com/pub/gaming/lghub_installer.exe" -OutFile $logitechInstaller"#,
            r#"$helixZip = "$env:TEMP\helix.zip";$installDir = "$env:LOCALAPPDATA\Programs\helix";Invoke-WebRequest -Uri "https://github.com/helix-editor/helix/releases/latest/download/helix-windows.zip" -OutFile $helixZip"#,
            r#"$helixZip = "$env:TEMP\helix.zip";$installDir = "$env:LOCALAPPDATA\Programs\helix";if (-Not (Test-Path $installDir)) {
                New-Item -ItemType Directory -Path $installDir | Out-Null
            }"#,
            r#"$helixZip = "$env:TEMP\helix.zip";$installDir = "$env:LOCALAPPDATA\Programs\helix";Add-Type -AssemblyName System.IO.Compression.FileSystem
[System.IO.Compression.ZipFile]::ExtractToDirectory($helixZip, $installDir, $true)"#,
            r#"Remove-Item $helixZip"#,
            r#"$helixZip = "$env:TEMP\helix.zip";$installDir = "$env:LOCALAPPDATA\Programs\helix";$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
 if (-Not $userPath.Split(';') -contains $installDir) {
     [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
 }"#,
        ],
        name: "Helix",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Battlenet (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            r#"$bnetInstaller = "$env:TEMP\BattleNet-Setup.exe""#,
            r#"$bnetInstaller = "$env:TEMP\BattleNet-Setup.exe";Invoke-WebRequest -Uri "https://www.battle.net/download/getInstaller?os=win&installer=Battle.net-Setup.exe" -OutFile $bnetInstaller"#,
            r#"$bnetInstaller = "$env:TEMP\BattleNet-Setup.exe";Start-Process -FilePath $bnetInstaller -ArgumentList "/SILENT" -Wait"#,
            r#"$bnetInstaller = "$env:TEMP\BattleNet-Setup.exe";Remove-Item $bnetInstaller"#,
        ],
        name: "Battlenet",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install PowerToys (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            r#"$powertoysInstaller = "$env:TEMP\PowerToysSetup.exe""#,
            r#"$powertoysInstaller = "$env:TEMP\PowerToysSetup.exe";Invoke-WebRequest -Uri "https://github.com/microsoft/PowerToys/releases/latest/download/PowerToysSetup-x64.exe" -OutFile $powertoysInstaller"#,
            r#"$powertoysInstaller = "$env:TEMP\PowerToysSetup.exe";Start-Process -FilePath $powertoysInstaller -ArgumentList "/silent" -Wait"#,
            r#"$powertoysInstaller = "$env:TEMP\PowerToysSetup.exe";Remove-Item $powertoysInstaller"#,
        ],
        name: "PowerToys",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install WSL (Not Tested Yet)",
        category: JobCategory::Windows,
        list_of_commands: &[
            r#"if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
                Write-Host "❗ Please run this script as Administrator." -ForegroundColor Red
                return
            }"#,
            r#"wsl --install --quiet"#,
        ],
        name: "WSL",
        require_admin: true,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Notepad++ (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            r#"$nppInstaller = "$env:TEMP\npp_installer.exe""#,
            r#"$nppInstaller = "$env:TEMP\npp_installer.exe";Invoke-WebRequest -Uri "https://github.com/notepad-plus-plus/notepad-plus-plus/releases/latest/download/npp.8.6.8.Installer.x64.exe" -OutFile $nppInstaller"#,
            r#"$nppInstaller = "$env:TEMP\npp_installer.exe";Start-Process -FilePath $nppInstaller -ArgumentList "/S" -Wait"#,
            r#"$nppInstaller = "$env:TEMP\npp_installer.exe";Remove-Item $nppInstaller"#,
        ],
        name: "Notepad++",
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        name: "PowerMode High Performance",
        explination: "Windows PowerMode High Performance (Not Tested Yet)",
        category: JobCategory::Windows,
        list_of_commands: &[
            r#"$highPerf = powercfg -L | Select-String -Pattern "High performance" | ForEach-Object {($_ -split ' ')[3]}"#,
            r#"$highPerf = powercfg -L | Select-String -Pattern "High performance" | ForEach-Object {($_ -split ' ')[3]};if ($highPerf) {
    Write-Host "👉 Setting power plan to High performance ($highPerf)..."
    powercfg -S $highPerf
    Write-Host "✅ Power mode set to High performance."
} else {
    Write-Host "❗ High performance plan not found. Creating it..."
    powercfg -duplicatescheme SCHEME_MIN
    $newHighPerf = powercfg -L | Select-String -Pattern "High performance" | ForEach-Object {
        ($_ -split ' ')[3]
    }
    if ($newHighPerf) {
        powercfg -S $newHighPerf
        Write-Host "✅ Power mode set to High performance."
    } else {
        Write-Host "❌ Failed to set High performance plan."
    }
}"#,
        ],
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        name: "Display timeout 30min",
        explination: "When plugged in, turn off my screen after 30 minutes",
        category: JobCategory::Windows,
        list_of_commands: &[r#"
function Set-DisplayTimeout {
    param (
        [int]$timeoutSeconds
    )

    # Get the current active power scheme GUID
    $output = powercfg /getactivescheme
    if ($output -match 'Power Scheme GUID:\s+([a-f0-9\-]+)') {
        $activeScheme = $matches[1]
    } else {
        Write-Error "❌ Could not determine active power scheme GUID."
        return
    }

    # GUID for display timeout setting
    $subGroup = "7516b95f-f776-4464-8c53-06167f40cc99"  # Video settings subgroup
    $setting = "3c0bc021-c8a8-4e07-a973-6b14cbcb2b7e"   # Turn off display after

    # Set timeout for AC (plugged in)
    powercfg /setacvalueindex $activeScheme $subGroup $setting $timeoutSeconds

    # Set timeout for DC (battery)
    powercfg /setdcvalueindex $activeScheme $subGroup $setting $timeoutSeconds

    # Apply the changes
    powercfg /setactive $activeScheme

    Write-Host "✅ Screen turn-off timeout set to $timeoutSeconds seconds."
}

# Example: Set display timeout to 1800 seconds (30 minutes)
Set-DisplayTimeout -timeoutSeconds 1800
"#],
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        name: "Never Sleep",
        explination: "When plugged in, put my device to sleep after Never",
        category: JobCategory::Windows,
        list_of_commands: &[r#"function Set-SleepTimeout {
    param (
        [int]$timeoutSeconds
    )

    # Get the current active power scheme GUID
    $output = powercfg /getactivescheme
    if ($output -match 'Power Scheme GUID:\s+([a-f0-9\-]+)') {
        $activeScheme = $matches[1]
    } else {
        Write-Error "❌ Could not determine active power scheme GUID."
        return
    }

    # GUID for sleep settings subgroup
    $subGroup = "238C9FA8-0AAD-41ED-83F4-97BE242C8F20"  # Sleep settings subgroup
    $setting = "29F6C1DB-86DA-48C5-9FDB-F2B67B1F44DA"   # Sleep after timeout

    # Set sleep timeout for AC (plugged in)
    powercfg /setacvalueindex $activeScheme $subGroup $setting $timeoutSeconds

    # Apply the changes
    powercfg /setactive $activeScheme

    Write-Host "✅ Sleep timeout (plugged in) set to $timeoutSeconds seconds."
}

# Example: Set sleep timeout to never (0 seconds)
Set-SleepTimeout -timeoutSeconds 0"#],
        require_admin: false,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        name: "Remove Cortana",
        explination: "Remove Cortana (Not Tested)",
        category: JobCategory::Windows,
        list_of_commands: &[
            r#"Get-AppxPackage -AllUsers -Name Microsoft.549981C3F5F10 | Remove-AppxPackage"#,
            r#"Write-Host "✅ Cortana has been removed for all users.""#,
        ],
        require_admin: false, // Removes from all user if true
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "EnableVoiceTyping 0",
        explination: r#"RegKey HKCU:\Software\Microsoft\Input\Settings EnableVoiceTyping 0"#,
        category: JobCategory::Windows,

        require_admin: false,
        reg_keys: &[RegKey {
            path: r#"HKCU:\Software\Microsoft\Input\Settings"#,
            name: r#"EnableVoiceTyping"#,
            value: r#"0"#,
            key_type: RegKeyType::DWORD,
        }],
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "Sticky Keys",
        explination: "Disable Sticky Keys (Spam Shift) (Not Tested Yet)",
        category: JobCategory::Windows,
        reg_keys: &[
            RegKey {
                path: r#"HKCU:\Control Panel\Accessibility\StickyKeys"#,
                name: r#"Flags"#,
                value: r#"506"#,
                key_type: RegKeyType::DWORD,
            },
            RegKey {
                path: r#"HKCU:\Control Panel\Accessibility\StickyKeys"#,
                name: r#"HotkeyFlags"#,
                value: r#"0"#,
                key_type: RegKeyType::DWORD,
            },
            RegKey {
                path: r#"HKCU:\Control Panel\Accessibility\StickyKeys"#,
                name: r#"PopupSetting"#,
                value: r#"0"#,
                key_type: RegKeyType::DWORD,
            },
        ],
        require_admin: false,
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "Developer Mode",
        explination: "Windows Developer Mode (Not Tested Yet)",
        category: JobCategory::Windows,
        reg_keys: &[RegKey {
            path: r#"HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock"#,
            name: r#"AllowDevelopmentWithoutDevLicense"#,
            value: r#"1"#,
            key_type: RegKeyType::DWORD,
        }],
        require_admin: true,
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "Toolbar Search",
        explination: "Hides the searchbra at the toolbar (Not Tested Yet)",
        category: JobCategory::Windows,
        reg_keys: &[RegKey {
            path: r#"HKCU\Software\Microsoft\Windows\CurrentVersion\Search"#,
            name: r#"SearchboxTaskbarMode"#,
            value: r#"0"#,
            key_type: RegKeyType::DWORD,
        }],
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
];
