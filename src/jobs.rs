use strum_macros::{EnumCount, EnumIter};

use crate::commands::execute_powershell_command;
use crate::error::Result;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct PowerShellCtx {
    pub(crate) name: &'static str,
    pub(crate) explination: &'static str,
    pub(crate) category: JobCategory,
    pub(crate) list_of_commands: &'static [PowerShellCommand],
}

impl ExecutableJob for PowerShellCtx {
    fn execute(&mut self) -> Result<()> {
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

#[derive(Clone)]
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
    jobs: Vec<Job>,
}

impl JobHandler {
    pub fn set_jobs(&mut self, new_jobs: Vec<Job>) {
        self.jobs = new_jobs;
    }

    pub fn get_jobs(&self) -> &[Job] {
        &self.jobs
    }

    pub fn get_job_progress<'a>(&'a self) -> impl Iterator<Item = (&'a Job, f32)> {
        let progress_iter = [1.0_f32; 3].iter();
        assert_eq!(
            progress_iter.clone().count(),
            self.jobs.len(),
            "not the same length"
        );

        self.jobs.iter().zip(progress_iter).map(|(j, p)| (j, *p))
    }

    pub fn finished(&self) -> bool {
        self.get_job_progress().all(|f| f.1 >= 1.0)
    }
}
