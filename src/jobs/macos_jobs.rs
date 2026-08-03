use crate::{
    jobs::job::{
        Job, JobCategory, JobReadyState, PowerShellCtx, PowerShellRegKeyCtx, RegKey, RegKeyType,
        StaticPowerShellCommand,
    },
};

// Formatting does not work
#[rustfmt::skip] 
pub static MACOS_JOBS: &[Job] = &[
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
        name: "Test2",
        tested: JobReadyState::NOTTESTED,
    }),
];