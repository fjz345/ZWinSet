use crate::{
    jobs::{
        Job, JobCategory, JobReadyState, PowerShellCtx, PowerShellRegKeyCtx, RegKey, RegKeyType,
        StaticPowerShellCommand,
    },
    windows::{clear_recent_files, disable_sticky_keys, restart_explorer},
};

// Formatting does not work
#[rustfmt::skip] 
pub static ALL_JOBS: &[Job] = &[
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Dev Test, 2 sec duration",
        category: JobCategory::Etc,
        list_of_commands: &[
            StaticPowerShellCommand::new("Start-Sleep -Milliseconds 200"),
            StaticPowerShellCommand::new("Start-Sleep -Milliseconds 200"),
            StaticPowerShellCommand::new("Start-Sleep -Milliseconds 200"),
            StaticPowerShellCommand::new("Start-Sleep -Milliseconds 200"),
            StaticPowerShellCommand::new("Start-Sleep -Milliseconds 200"),
            StaticPowerShellCommand::new("Start-Sleep -Milliseconds 200"),
            StaticPowerShellCommand::new("Start-Sleep -Milliseconds 200"),
            StaticPowerShellCommand::new("Start-Sleep -Milliseconds 200"),
            StaticPowerShellCommand::new("Start-Sleep -Milliseconds 200"),
            StaticPowerShellCommand::new("Start-Sleep -Milliseconds 200"),
        ],
        name: "Test",
        tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Google Chrome (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            StaticPowerShellCommand::new(r#"$chromeInstaller = "$env:TEMP\chrome_installer.exe""#),
            StaticPowerShellCommand::new(
                r#"$chromeInstaller = "$env:TEMP\chrome_installer.exe";Invoke-WebRequest -Uri "https://dl.google.com/chrome/install/latest/chrome_installer.exe" -OutFile $chromeInstaller"#,
            ),
            StaticPowerShellCommand::new(
                r#"$chromeInstaller = "$env:TEMP\chrome_installer.exe"Start-Process -FilePath $chromeInstaller -ArgumentList "/silent", "/install" -NoNewWindow -Wait"#,
            ),
            StaticPowerShellCommand::new(
                r#"$chromeInstaller = "$env:TEMP\chrome_installer.exe"Remove-Item $chromeInstaller"#,
            ),
        ],
        name: "Chrome",tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Steam (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            StaticPowerShellCommand::new(r#"$steamInstaller = "$env:TEMP\steam_installer.exe""#),
            StaticPowerShellCommand::new(
                r#"$steamInstaller = "$env:TEMP\steam_installer.exe";Invoke-WebRequest -Uri "https://cdn.cloudflare.steamstatic.com/client/installer/SteamSetup.exe" -OutFile $steamInstaller"#,
            ),
            StaticPowerShellCommand::new(
                r#"$steamInstaller = "$env:TEMP\steam_installer.exe";Start-Process -FilePath $steamInstaller -ArgumentList "/S" -NoNewWindow -Wait"#,
            ),
            StaticPowerShellCommand::new(
                r#"$steamInstaller = "$env:TEMP\steam_installer.exe";Remove-Item $steamInstaller"#,
            ),
        ],
        name: "Steam",tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Discord (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            StaticPowerShellCommand::new(r#"$discordInstaller = "$env:TEMP\DiscordSetup.exe""#),
            StaticPowerShellCommand::new(
                r#"$discordInstaller = "$env:TEMP\DiscordSetup.exe";Invoke-WebRequest -Uri "https://discord.com/api/download?platform=win" -OutFile $discordInstaller"#,
            ),
            StaticPowerShellCommand::new(
                r#"$discordInstaller = "$env:TEMP\DiscordSetup.exe";Start-Process -FilePath $discordInstaller -ArgumentList "/S" -NoNewWindow -Wait"#,
            ),
            StaticPowerShellCommand::new(
                r#"$discordInstaller = "$env:TEMP\DiscordSetup.exe";Remove-Item $discordInstaller"#,
            ),
        ],
        name: "Discord",tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Nvidia App (Drivers) (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            StaticPowerShellCommand::new(
                r#"irm https://github.com/emilwojcik93/Install-NvidiaApp/releases/latest/download/Install-NvidiaApp.ps1 | iex"#,
            ),
            StaticPowerShellCommand::new(
                r#"Install-NvidiaApp.ps1 -Edition Public -SilentInstall -SkipCheck"#,
            ),
        ],
        name: "Nvidia App",tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Nvidia Broadcast (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            StaticPowerShellCommand::new(
                r#"$broadcastInstaller = "$env:TEMP\NvidiaBroadcastInstaller.exe""#,
            ),
            StaticPowerShellCommand::new(
                r#"$broadcastInstaller = "$env:TEMP\NvidiaBroadcastInstaller.exe";Invoke-WebRequest -Uri "https://developer.nvidia.com/compute/broadcast-sdk/redist/nvidia_broadcast_app_latest.exe" -OutFile $broadcastInstaller"#,
            ),
            StaticPowerShellCommand::new(
                r#"$broadcastInstaller = "$env:TEMP\NvidiaBroadcastInstaller.exe";Start-Process -FilePath $broadcastInstaller -ArgumentList "/silent" -Wait"#,
            ),
            StaticPowerShellCommand::new(
                r#"$broadcastInstaller = "$env:TEMP\NvidiaBroadcastInstaller.exe";Remove-Item $broadcastInstaller"#,
            ),
        ],
        name: "Nvidia Broadcast",  tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Logitech Hub (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            StaticPowerShellCommand::new(r#"$logitechInstaller = "$env:TEMP\lghub_installer.exe""#),
            StaticPowerShellCommand::new(
                r#"$logitechInstaller = "$env:TEMP\lghub_installer.exe";Invoke-WebRequest -Uri "https://downloads.logitech.com/pub/gaming/lghub_installer.exe" -OutFile $logitechInstaller"#,
            ),
            StaticPowerShellCommand::new(
                r#"$logitechInstaller = "$env:TEMP\lghub_installer.exe";Start-Process -FilePath $logitechInstaller -ArgumentList "--silent" -Wait"#,
            ),
            StaticPowerShellCommand::new(
                r#"$logitechInstaller = "$env:TEMP\lghub_installer.exe";Remove-Item $logitechInstaller"#,
            ),
        ],
        name: "Logitech Hub",tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Visual Code (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            StaticPowerShellCommand::new(r#"$vsCodeInstaller = "$env:TEMP\VSCodeUserSetup.exe""#),
            StaticPowerShellCommand::new(
                r#"$vsCodeInstaller = "$env:TEMP\VSCodeUserSetup.exe";Invoke-WebRequest -Uri "https://update.code.visualstudio.com/latest/win32-x64-user/stable" -OutFile $vsCodeInstaller"#,
            ),
            StaticPowerShellCommand::new(
                r#"$vsCodeInstaller = "$env:TEMP\VSCodeUserSetup.exe";Start-Process -FilePath $vsCodeInstaller -ArgumentList "/silent", "/mergetasks=!runcode" -Wait"#,
            ),
            StaticPowerShellCommand::new(
                r#"$vsCodeInstaller = "$env:TEMP\VSCodeUserSetup.exe";Remove-Item $vsCodeInstaller"#,
            ),
        ],
        name: "Visual Code",tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Mullvad VPN (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            StaticPowerShellCommand::new(r#"$mullvadInstaller = "$env:TEMP\MullvadVPN.exe""#),
            StaticPowerShellCommand::new(
                r#"$mullvadInstaller = "$env:TEMP\MullvadVPN.exe";Invoke-WebRequest -Uri "https://mullvad.net/download/app/win/latest/" -OutFile $mullvadInstaller"#,
            ),
            StaticPowerShellCommand::new(
                r#"$mullvadInstaller = "$env:TEMP\MullvadVPN.exe";Start-Process -FilePath $mullvadInstaller -ArgumentList "/S" -Wait"#,
            ),
            StaticPowerShellCommand::new(
                r#"$mullvadInstaller = "$env:TEMP\MullvadVPN.exe";Remove-Item $mullvadInstaller"#,
            ),
        ],
        name: "Mullvad",tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Helix (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            StaticPowerShellCommand::new(
                r#"$helixZip = "$env:TEMP\helix.zip";$installDir = "$env:LOCALAPPDATA\Programs\helix""#,
            ),
            StaticPowerShellCommand::new(
                r#"$helixZip = "$env:TEMP\helix.zip";$installDir = "$env:LOCALAPPDATA\Programs\helix";Invoke-WebRequest -Uri "https://downloads.logitech.com/pub/gaming/lghub_installer.exe" -OutFile $logitechInstaller"#,
            ),
            StaticPowerShellCommand::new(
                r#"$helixZip = "$env:TEMP\helix.zip";$installDir = "$env:LOCALAPPDATA\Programs\helix";Invoke-WebRequest -Uri "https://github.com/helix-editor/helix/releases/latest/download/helix-windows.zip" -OutFile $helixZip"#,
            ),
            StaticPowerShellCommand::new(
                r#"$helixZip = "$env:TEMP\helix.zip";$installDir = "$env:LOCALAPPDATA\Programs\helix";if (-Not (Test-Path $installDir)) {
                New-Item -ItemType Directory -Path $installDir | Out-Null
            }"#,
            ),
            StaticPowerShellCommand::new(
                r#"$helixZip = "$env:TEMP\helix.zip";$installDir = "$env:LOCALAPPDATA\Programs\helix";Add-Type -AssemblyName System.IO.Compression.FileSystem
[System.IO.Compression.ZipFile]::ExtractToDirectory($helixZip, $installDir, $true)"#,
            ),
            StaticPowerShellCommand::new(r#"Remove-Item $helixZip"#),
            StaticPowerShellCommand::new(
                r#"$helixZip = "$env:TEMP\helix.zip";$installDir = "$env:LOCALAPPDATA\Programs\helix";$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
 if (-Not $userPath.Split(';') -contains $installDir) {
     [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
 }"#,
            ),
        ],
        name: "Helix",tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Battlenet (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            StaticPowerShellCommand::new(r#"$bnetInstaller = "$env:TEMP\BattleNet-Setup.exe""#),
            StaticPowerShellCommand::new(
                r#"$bnetInstaller = "$env:TEMP\BattleNet-Setup.exe";Invoke-WebRequest -Uri "https://www.battle.net/download/getInstaller?os=win&installer=Battle.net-Setup.exe" -OutFile $bnetInstaller"#,
            ),
            StaticPowerShellCommand::new(
                r#"$bnetInstaller = "$env:TEMP\BattleNet-Setup.exe";Start-Process -FilePath $bnetInstaller -ArgumentList "/SILENT" -Wait"#,
            ),
            StaticPowerShellCommand::new(
                r#"$bnetInstaller = "$env:TEMP\BattleNet-Setup.exe";Remove-Item $bnetInstaller"#,
            ),
        ],
        name: "Battlenet",tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install PowerToys (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            StaticPowerShellCommand::new(
                r#"$powertoysInstaller = Join-Path $env:TEMP 'PowerToysSetup.exe'; Write-Host 'PowerToys: Downloading...'; curl.exe -L -o $powertoysInstaller 'https://github.com/microsoft/PowerToys/releases/download/v0.81.1/PowerToysSetup-0.81.1-x64.exe')"#,
            ),
            StaticPowerShellCommand::new(
                r#"$powertoysInstaller = Join-Path $env:TEMP 'PowerToysSetup.exe';if ((Get-Item $powertoysInstaller).Length -lt 1024kb) { Write-Host 'Download failed or file too small.'; exit 1 }"#,
            ),
            StaticPowerShellCommand::new(
                r#"$powertoysInstaller = Join-Path $env:TEMP 'PowerToysSetup.exe'; Write-Host 'PowerToys: Installing...'; Start-Process -FilePath $powertoysInstaller -ArgumentList '/silent' -Wait"#,
            ).req_admin(),
            StaticPowerShellCommand::new(
                r#"$powertoysInstaller = Join-Path $env:TEMP 'PowerToysSetup.exe'; Write-Host 'PowerToys: Cleaning up...'; Remove-Item $powertoysInstaller"#,
            ),
        ],
        name: "PowerToys",tested: JobReadyState::VERIFIED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install WSL (Not Tested Yet)",
        category: JobCategory::Windows,
        list_of_commands: &[
            StaticPowerShellCommand::new(
                r#"if (-not ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
                Write-Host "❗ Please run this script as Administrator." -ForegroundColor Red
                return
            }"#,
            ).req_admin(),
            StaticPowerShellCommand::new(r#"wsl --install --quiet"#).req_admin(),
        ],
        name: "WSL",tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Install Notepad++ (Not Tested Yet)",
        category: JobCategory::Application,
        list_of_commands: &[
            StaticPowerShellCommand::new(r#"
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$release = Invoke-RestMethod -Uri "https://api.github.com/repos/notepad-plus-plus/notepad-plus-plus/releases/latest"
$installerAsset = $release.assets | Where-Object { $_.name -like "*.Installer.x64.exe" } | Select-Object -First 1

$nppInstaller = "$env:TEMP\\npp_installer.exe"
Invoke-WebRequest -Uri $installerAsset.browser_download_url -OutFile $nppInstaller -UseBasicParsing

Start-Process -FilePath $nppInstaller -ArgumentList "/S" -Wait

Remove-Item $nppInstaller"#
),
        ],
        name: "Notepad++",tested: JobReadyState::VERIFIED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        name: "PowerMode High Performance",
        explination: "Windows PowerMode High Performance (Not Tested Yet)",
        category: JobCategory::Windows,
        list_of_commands: &[
            StaticPowerShellCommand::new(
                r#"$highPerf = powercfg -L | Select-String -Pattern "High performance" | ForEach-Object {($_ -split ' ')[3]}"#,
            ),
            StaticPowerShellCommand::new(
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
            ),
        ],tested: JobReadyState::VERIFIED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        name: "Display timeout 30min",
        explination: "When plugged in, turn off my screen after 30 minutes",
        category: JobCategory::Windows,
        list_of_commands: &[StaticPowerShellCommand::new(
            r#"
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
"#,
        )],tested: JobReadyState::VERIFIED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        name: "Never Sleep",
        explination: "When plugged in, put my device to sleep after Never",
        category: JobCategory::Windows,
        list_of_commands: &[StaticPowerShellCommand::new(
            r#"function Set-SleepTimeout {
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
Set-SleepTimeout -timeoutSeconds 0"#,
        )],tested: JobReadyState::VERIFIED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        name: "Remove Cortana",
        explination: "Remove Cortana (Not Tested)",
        category: JobCategory::Windows,
        list_of_commands: &[StaticPowerShellCommand::new(
            r#"Get-AppxPackage -Name Microsoft.549981C3F5F10 | Remove-AppxPackage"#,
        )],tested: JobReadyState::VERIFIED,
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
        post_fn: None,tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "Sticky Keys (shift)",
        explination: "Disable Sticky Keys (Spam Shift) (Not Tested Yet)",
        category: JobCategory::Windows,
        reg_keys: &[
            RegKey {
                path: r#"HKCU:\Control Panel\Accessibility\StickyKeys"#,
                name: r#"Flags"#,
                value: r#"506"#, // off
                key_type: RegKeyType::DWORD,
            },
            RegKey {
                path: r#"HKCU:\Control Panel\Accessibility\StickyKeys"#,
                name: r#"HotKeyActive"#,
                value: r#"0"#, // disables Shift hotkey
                key_type: RegKeyType::DWORD,
            },
            RegKey {
                path: r#"HKCU:\Control Panel\Accessibility\StickyKeys"#,
                name: r#"ConfirmActivation"#,
                value: r#"0"#, // disables confirmation popup
                key_type: RegKeyType::DWORD,
            },
            RegKey {
                path: r#"HKCU:\Control Panel\Accessibility\Keyboard Response"#,
                name: r#"HotKeyActive"#,
                value: r#"0"#,
                key_type: RegKeyType::DWORD,
            },
            RegKey {
                path: r#"HKCU:\Control Panel\Accessibility\ToggleKeys"#,
                name: r#"HotKeyActive"#,
                value: r#"0"#,
                key_type: RegKeyType::DWORD,
            },
        ],
        require_admin: false,
        post_fn: Some(disable_sticky_keys),tested: JobReadyState::VERIFIED,
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
    name: "Disable Recent Start Menu Recommended",
    explination: "Disables Recent Items and Frequent Apps in Windows Start Menu",
    category: JobCategory::Windows,
    reg_keys: &[
        RegKey {
            path: r#"HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"#,
            name: r#"Start_TrackDocs"#,
            value: r#"0"#, // disables recently opened documents
            key_type: RegKeyType::DWORD,
        },
        RegKey {
            path: r#"HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced"#,
            name: r#"Start_TrackProgs"#,
            value: r#"0"#, // disables frequently used apps
            key_type: RegKeyType::DWORD,
        },
        RegKey {
            path: r#"HKCU:\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer"#,
            name: r#"NoRecentDocsHistory"#,
            value: r#"1"#,
            key_type: RegKeyType::DWORD,
        },
        RegKey {
            path: r#"HKCU:\Software\Microsoft\Windows\CurrentVersion\Policies\Explorer"#,
            name: r#"NoRecentDocsMenu"#,
            value: r#"1"#,
            key_type: RegKeyType::DWORD,
        },
    ],
    require_admin: false,
    post_fn: Some(||{clear_recent_files();restart_explorer();}),tested: JobReadyState::VERIFIED,
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
        post_fn: None,tested: JobReadyState::VERIFIED,
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "Toolbar Search",
        explination: "Hides the search bra at the toolbar (Not Tested Yet)",
        category: JobCategory::Windows,
        reg_keys: &[RegKey {
            path: r#"HKCU:\Software\Microsoft\Windows\CurrentVersion\Search"#,
            name: r#"SearchboxTaskbarMode"#,
            value: r#"0"#,
            key_type: RegKeyType::DWORD,
        }],
        require_admin: false,
        post_fn: None,tested: JobReadyState::VERIFIED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Windows mouse acceleration is on by default",
        category: JobCategory::Windows,
        list_of_commands: &[StaticPowerShellCommand::new(
            r#"
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
            "#,
        )],
        name: "Mouse Acceleration",tested: JobReadyState::VERIFIED,
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "Disable diagnostic data collection",
        explination: "Disables Windows DataCollection",
        category: JobCategory::Windows,
        reg_keys: &[
            RegKey {
                path: r#"HKLM:\SOFTWARE\Policies\Microsoft\Windows\DataCollection"#,
                name: r#"AllowTelemetry"#,
                value: r#"0"#,
                key_type: RegKeyType::DWORD,
            },
        ],
        require_admin: false,
        post_fn: None,
        tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "Disable Windows Tips & Suggestions",
        explination: "ContentDeliveryManager RegKeys",
        category: JobCategory::Windows,
        reg_keys: &[
            RegKey {
                path: r#"HKCU:\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager"#,
                name: r#"SubscribedContent-338387Enabled"#,
                value: r#"0"#,
                key_type: RegKeyType::DWORD,
            },
            RegKey {
                path: r#"HKCU:\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager"#,
                name: r#"SystemPaneSuggestionsEnabled"#,
                value: r#"0"#,
                key_type: RegKeyType::DWORD,
            },
        ],
        require_admin: false,
        post_fn: None,
        tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "Disable Location Tracking",
        explination: "CapabilityAccessManager RegKeys",
        category: JobCategory::Windows,
        reg_keys: &[
            RegKey {
                path: r#"HKCU:\Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\location"#,
                name: r#"Value"#,
                value: r#"Deny"#,
                key_type: RegKeyType::STRING,
            }
        ],
        require_admin: false,
        post_fn: None,
        tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "Disable Ads in Start Menu and Lock Screen",
        explination: "ContentDeliveryManager RegKeys",
        category: JobCategory::Windows,
        reg_keys: &[
            RegKey {
                path: r#"HKCU:\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager"#,
                name: r#"SilentInstalledAppsEnabled"#,
                value: r#"0"#,
                key_type: RegKeyType::DWORD,
            },
            RegKey {
                path: r#"HKCU:\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager"#,
                name: r#"SystemPaneSuggestionsEnabled"#,
                value: r#"0"#,
                key_type: RegKeyType::DWORD,
            }
        ],
        require_admin: false,
        post_fn: None,
        tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "Disable Microsoft Consumer Experience",
        explination: "suggested apps on new installs",
        category: JobCategory::Windows,
        reg_keys: &[
            RegKey {
                path: r#"HKLM:\SOFTWARE\Policies\Microsoft\Windows\CloudContent"#,
                name: r#"DisableWindowsConsumerFeatures"#,
                value: r#"1"#,
                key_type: RegKeyType::DWORD,
            }
        ],
        require_admin: false,
        post_fn: None,
        tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "Disable OneDrive",
        explination: "",
        category: JobCategory::Windows,
        reg_keys: &[
            RegKey {
                path: r#"HKLM:\SOFTWARE\Policies\Microsoft\Windows\OneDrive"#,
                name: r#"DisableFileSyncNGSC"#,
                value: r#"1"#,
                key_type: RegKeyType::DWORD,
            }
        ],
        require_admin: false,
        post_fn: None,
        tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellRegKey(PowerShellRegKeyCtx {
        name: "Disable Update Auto-Restart",
        explination: "Disable Windows Update Auto-Restart",
        category: JobCategory::Windows,
        reg_keys: &[
            RegKey {
                path: r#"HKLM:\SOFTWARE\Policies\Microsoft\Windows\WindowsUpdate\AU"#,
                name: r#"NoAutoRebootWithLoggedOnUsers"#,
                value: r#"1"#,
                key_type: RegKeyType::DWORD,
            }
        ],
        require_admin: false,
        post_fn: None,
        tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Will disable searching with windows explorer",
        category: JobCategory::Windows,
        list_of_commands: &[StaticPowerShellCommand::new(
            r#"Stop-Service -Name "WSearch""#,
        ),
        StaticPowerShellCommand::new(
            r#"Set-Service -Name "WSearch" -StartupType Disabled"#,
        )],
        name: "Disable Search",tested: JobReadyState::NOTTESTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Microdick Activation Script",
        category: JobCategory::Windows,
        list_of_commands: &[],
        name: "MAS",tested: JobReadyState::NOTIMPLEMENTED,
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "Show File Explorer Extensions",
        category: JobCategory::Windows,
        list_of_commands: &[],
        name: "Explorer Extensions",tested: JobReadyState::NOTIMPLEMENTED,
    }),
];
