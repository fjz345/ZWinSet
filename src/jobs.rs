use strum_macros::{EnumCount, EnumIter};

use crate::commands::execute_powershell_command;
use crate::error::Result;

#[derive(Clone, PartialEq)]
pub enum Job {
    PowerShellCommand(PowerShellCtx),
    InstallApplication(InstallApplicationCtx),
}

impl Job {
    pub fn execute(&mut self) {
        match self {
            Job::PowerShellCommand(job) => {
                job.execute().expect("failed to execute powershell command")
            }
            Job::InstallApplication(job) => todo!(),
        }
    }

    pub fn category(&self) -> JobCategory {
        match self {
            Job::PowerShellCommand(job) => job.category,
            Job::InstallApplication(job) => job.category,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Job::PowerShellCommand(job) => job.name,
            Job::InstallApplication(job) => job.name,
        }
    }
}

#[derive(Default, Debug, Copy, Clone, EnumCount, EnumIter, PartialEq)]
pub enum JobCategory {
    #[default]
    Etc,
    Application,
    Windows,
}

trait ExecutableJob {
    fn execute(&mut self) -> Result<()>;
    fn category(&self) -> JobCategory {
        JobCategory::default()
    }
    fn name(&self) -> &'static str;
}

pub type PowerShellCommand = &'static str;

#[derive(Clone, Debug, PartialEq)]
pub struct PowerShellCtx {
    pub(crate) name: &'static str,
    pub(crate) explination: &'static str,
    pub(crate) category: JobCategory,
    pub(crate) list_of_commands: &'static [PowerShellCommand],
}

impl ExecutableJob for PowerShellCtx {
    fn execute(&mut self) -> Result<()> {
        // TEST
        log::info!("Executed job: {:?}", self);
        return Ok(());

        let commands_to_execute = self.list_of_commands.clone();
        for cmd in commands_to_execute {
            match execute_powershell_command(&[cmd]) {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let status = &output.status;

                    if status.success() {
                        // log::info!("{}", stdout);
                        // log::error!("{}", stderr);
                    } else {
                        // log::info!("{}", stdout);
                        // log::error!("{}", stderr);
                        // log::error!("{}", status);
                    }
                }
                Err(e) => log::error!("{e}"),
            }
        }

        Ok(())
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

#[derive(Clone, PartialEq)]
pub struct InstallApplicationCtx {
    pub(crate) name: &'static str,
    pub(crate) explination: &'static str,
    pub(crate) category: JobCategory,
    pub(crate) application_name: &'static str,
}

impl ExecutableJob for InstallApplicationCtx {
    fn execute(&mut self) -> Result<()> {
        todo!()
    }

    fn category(&self) -> JobCategory {
        self.category
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

#[derive(Default)]
pub struct JobHandler {
    jobs_with_progress: Vec<(Job, f32)>,
    
}

impl JobHandler {
    pub fn set_jobs(&mut self, new_jobs: Vec<Job>) {
        self.jobs_with_progress = new_jobs.iter().cloned().map(|job| (job, 0.0_f32)).collect();
    }

    pub fn get_jobs<'a>(&'a self) -> impl Iterator<Item = &'a Job> {
        self.jobs_with_progress.iter().map(|f|&f.0)
    }

    pub fn get_job_progress(&self) -> &Vec<(Job, f32)> {
        &self.jobs_with_progress
    }

    pub fn finished(&self) -> bool {
        self.get_job_progress().iter().map(|f|f.1).all(|f| f >= 1.0)
    }
}
