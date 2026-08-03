use crate::jobs::job::{
    Job, JobCategory, JobReadyState, RegKey, RegKeyType, StaticTerminalCommand, TerminalCtx,
};

// Formatting does not work
#[rustfmt::skip] 
pub static MACOS_JOBS: &[Job] = &[
    Job::TerminalCommand(TerminalCtx {
        explination: "Dev Test, 2 sec duration",
        category: JobCategory::Etc,
        list_of_commands: &[
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
        ],
        name: "Test",
        tested: JobReadyState::NOTTESTED,
    }),
        Job::TerminalCommand(TerminalCtx {
        explination: "Dev Test, 2 sec duration",
        category: JobCategory::Etc,
        list_of_commands: &[
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
            StaticTerminalCommand::new("sleep 0.2"),
        ],
        name: "Test2",
        tested: JobReadyState::NOTTESTED,
    }),
];
