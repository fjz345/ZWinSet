use crate::jobs::{InstallApplicationCtx, Job, JobCategory, PowerShellCtx};

pub static ALL_JOBS: &[Job] = &[
    Job::PowerShellCommand(PowerShellCtx {
        explination: "InstallDiscord",
        category: JobCategory::Application,
        list_of_commands: &["asd", "ASd"],
        name: "Discord",
    }),
    Job::PowerShellCommand(PowerShellCtx {
        explination: "InstallSteam",
        category: JobCategory::Application,
        list_of_commands: &["asd", "ASd"],
        name: "Steam",
    }),
    Job::InstallApplication(InstallApplicationCtx {
        explination: "Install Steam",
        category: JobCategory::Application,
        application_name: "Steam",
        name: "Steam",
    }),
];
