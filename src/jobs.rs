use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;

use eframe::egui::mutex::Mutex;
use strum_macros::{EnumCount, EnumIter};

use crate::commands::{execute_powershell_as_admin, execute_powershell_command};
use crate::error::Result;

#[derive(Clone, PartialEq, Debug)]
pub enum Job {
    PowerShellCommand(PowerShellCtx),
    InstallApplication(InstallApplicationCtx),
}

#[derive(Debug)]
struct JobStep {
    command: PowerShellCommand,
    require_admin: bool,
}

impl JobStep {
    pub fn require_admin(&self) -> bool {
        self.require_admin
    }
    pub fn execute(&mut self) -> Result<()> {
        log::info!("{}", self.command);
        let command_result = if self.require_admin() {
            execute_powershell_as_admin(&[self.command])
        } else {
            execute_powershell_command(&[self.command])
        };
        match command_result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let status = &output.status;

                if status.success() {
                    if stdout.len() >= 1 {
                        log::info!("{}", stdout);
                        // log::error!("{}", stderr);
                    }
                } else {
                    // log::info!("{}", stdout);
                    if stdout.len() >= 1 {
                        log::error!("{}", stderr);
                    }
                    log::error!("{}", status);
                }
            }
            Err(e) => log::error!("{e}"),
        }

        Ok(())
    }
}

impl Job {
    pub fn job_steps(&self) -> impl Iterator<Item = JobStep> {
        let steps: Vec<JobStep> = match self {
            Job::PowerShellCommand(job) => job.jobs_steps().collect(),
            Job::InstallApplication(job) => job.jobs_steps().collect(),
        };
        steps.into_iter()
    }

    pub fn job_count(&self) -> usize {
        let steps: Vec<JobStep> = match self {
            Job::PowerShellCommand(job) => job.jobs_steps().collect(),
            Job::InstallApplication(job) => job.jobs_steps().collect(),
        };
        steps.len()
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
    fn jobs_steps(&self) -> impl Iterator<Item = JobStep>;
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
    pub(crate) require_admin: bool,
}

impl ExecutableJob for PowerShellCtx {
    fn name(&self) -> &'static str {
        self.name
    }

    fn jobs_steps(&self) -> impl Iterator<Item = JobStep> {
        self.list_of_commands.iter().map(|f| JobStep {
            command: f,
            require_admin: self.require_admin,
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct InstallApplicationCtx {
    pub(crate) name: &'static str,
    pub(crate) explination: &'static str,
    pub(crate) category: JobCategory,
    pub(crate) application_name: &'static str,
    pub(crate) require_admin: bool,
}

impl ExecutableJob for InstallApplicationCtx {
    fn category(&self) -> JobCategory {
        self.category
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn jobs_steps(&self) -> impl Iterator<Item = JobStep> {
        [JobStep {
            command: "TODO",
            require_admin: self.require_admin,
        }]
        .into_iter()
    }
}

#[derive(Default)]
pub struct JobHandler {
    jobs_with_progress: Arc<Mutex<Vec<(Job, f32)>>>,
    jobs_not_queued: Vec<Job>,
    jobs_queued: VecDeque<Job>,
}

impl JobHandler {
    pub fn set_jobs(&mut self, new_jobs: Vec<Job>) {
        let jobs_with_0_progress: Vec<(Job, f32)> =
            new_jobs.iter().cloned().map(|job| (job, 0.0_f32)).collect();
        self.jobs_with_progress = Arc::new(Mutex::new(jobs_with_0_progress));
        self.jobs_not_queued = new_jobs;
    }

    pub fn get_jobs(&self) -> impl Iterator<Item = Job> {
        let mutex = self.jobs_with_progress.lock();
        let jobs: Vec<Job> = mutex.iter().map(|(a, _b)| a.clone()).collect();
        jobs.into_iter()
    }

    pub fn get_job_progress(&self) -> Vec<(Job, f32)> {
        let mutex = self.jobs_with_progress.lock();
        mutex.iter().map(|(a, b)| (a.clone(), *b)).collect()
    }

    pub fn finished(&self) -> bool {
        self.get_job_progress()
            .iter()
            .map(|f| f.1)
            .all(|f| f >= 1.0)
    }

    pub fn update(&mut self) {
        const MAX_QUEUED_JOBS: usize = 100;
        while self.jobs_not_queued.len() > 0 && self.jobs_queued.len() <= MAX_QUEUED_JOBS {
            let popped = self.jobs_not_queued.pop().unwrap();
            self.jobs_queued.push_back(popped);
        }

        let jobs_to_queue = self.jobs_queued.clone();
        self.jobs_queued.clear();

        let update_job_progress =
            |complete_job_mutex: Arc<Mutex<Vec<(Job, f32)>>>, job: &Job, new_progress: f32| {
                let mut mutex = complete_job_mutex.lock();
                let job_in_progress = mutex
                    .iter_mut()
                    .find(|(jjob, _progress)| *jjob == *job)
                    .unwrap();
                job_in_progress.1 = new_progress;
            };
        for job in jobs_to_queue {
            let complete_job_mutex = self.jobs_with_progress.clone();
            let _handle = thread::spawn(move || {
                log::trace!("Spawned thread {:?}", &job);

                let job_steps = job.job_steps();
                let job_count = job.job_count();
                for (i, mut step) in job_steps.enumerate() {
                    let res = step.execute();
                    match res {
                        Ok(_) => {}
                        Err(e) => log::error!("{e}"),
                    }
                    let new_progress = i as f32 / (job_count as f32);
                    update_job_progress(complete_job_mutex.clone(), &job, new_progress);
                }

                update_job_progress(complete_job_mutex, &job, 1.0);
                log::trace!("thread finished {:?}", &job);
            });
        }
    }
}
