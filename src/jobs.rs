use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;

use eframe::egui::mutex::Mutex;
use strum_macros::{EnumCount, EnumIter};

use crate::commands::{execute_powershell_as_admin, execute_powershell_command};
use crate::error::{Result, ZError};

#[derive(Clone, PartialEq, Debug)]
pub enum Job {
    PowerShellCommand(PowerShellCtx),
    PowerShellRegKey(PowerShellRegKeyCtx),
    RustFunction(RustFunctionCtx),
    InstallApplication(InstallApplicationCtx),
}

#[derive(Debug)]
pub struct JobStep {
    command: PowerShellCommand,
    require_admin: bool,
    post_fn: Option<fn()>,
}

impl JobStep {
    pub fn require_admin(&self) -> bool {
        self.require_admin
    }
    pub fn execute(&mut self) -> Result<()> {
        let powershell_result = if !self.command.is_empty() {
            if self.require_admin() {
                Some(execute_powershell_as_admin(&[self.command.clone()]))
            } else {
                Some(execute_powershell_command(&[self.command.clone()]))
            }
        } else {
            None
        };

        let mut z_error = None;
        if let Some(res) = powershell_result {
            match res {
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
                        if stdout.len() >= 1 {
                            log::info!("{}", stdout);
                            // log::error!("{}", stderr);
                        }
                        if stderr.len() >= 1 {
                            log::error!("{}", stderr);
                        }
                        z_error = Some(ZError::Message(format!("{status}")))
                    }
                }
                Err(e) => z_error = Some(ZError::Message(format!("{e}"))),
            }
        }

        if let Some(post_fn) = self.post_fn {
            post_fn();
        }

        if let Some(err) = z_error {
            Err(err)
        } else {
            Ok(())
        }
    }
}

impl Job {
    pub fn ready_state(&self) -> JobReadyState {
        match self {
            Job::PowerShellCommand(job) => job.tested,
            Job::InstallApplication(_job) => JobReadyState::const_default(),
            Job::PowerShellRegKey(job) => job.tested,
            Job::RustFunction(_job) => JobReadyState::const_default(),
        }
    }
    pub fn require_admin(&self) -> bool {
        self.job_steps().into_iter().any(|f| f.require_admin())
    }
    pub fn job_steps(&self) -> impl Iterator<Item = JobStep> {
        let steps: Vec<JobStep> = match self {
            Job::PowerShellCommand(job) => job.jobs_steps().collect(),
            Job::InstallApplication(job) => job.jobs_steps().collect(),
            Job::PowerShellRegKey(job) => job.jobs_steps().collect(),
            Job::RustFunction(job) => job.jobs_steps().collect(),
        };
        steps.into_iter()
    }

    pub fn job_count(&self) -> usize {
        let steps: Vec<JobStep> = match self {
            Job::PowerShellCommand(job) => job.jobs_steps().collect(),
            Job::InstallApplication(job) => job.jobs_steps().collect(),
            Job::PowerShellRegKey(job) => job.jobs_steps().collect(),
            Job::RustFunction(job) => job.jobs_steps().collect(),
        };
        steps.len()
    }

    pub fn category(&self) -> JobCategory {
        match self {
            Job::PowerShellCommand(job) => job.category,
            Job::InstallApplication(job) => job.category,
            Job::PowerShellRegKey(job) => job.category,
            Job::RustFunction(job) => job.category,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Job::PowerShellCommand(job) => job.name,
            Job::InstallApplication(job) => job.name,
            Job::PowerShellRegKey(job) => job.name,
            Job::RustFunction(job) => job.name,
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

    fn tested(&self) -> JobReadyState {
        JobReadyState::const_default()
    }
}

pub type PowerShellCommand = String;
#[derive(Debug, PartialEq)]
pub struct StaticPowerShellCommand {
    pub cmd: &'static str,
    pub requires_admin: RequireAdmin,
}

impl From<&'static str> for StaticPowerShellCommand {
    fn from(value: &'static str) -> Self {
        Self {
            cmd: value,
            requires_admin: RequireAdmin::default(),
        }
    }
}

impl StaticPowerShellCommand {
    pub const fn new(cmd: &'static str) -> Self {
        Self {
            cmd: cmd,
            requires_admin: RequireAdmin::const_default(),
        }
    }
    pub const fn req_admin(mut self) -> Self {
        self.requires_admin = RequireAdmin::YES;
        self
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum RequireAdmin {
    #[default]
    NO,
    YES,
}

impl Into<bool> for RequireAdmin {
    fn into(self) -> bool {
        match self {
            RequireAdmin::NO => false,
            RequireAdmin::YES => true,
        }
    }
}

impl RequireAdmin {
    pub const fn const_default() -> Self {
        RequireAdmin::NO
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum JobReadyState {
    #[default]
    NOTTESTED,
    VERIFIED,
}

impl Into<bool> for JobReadyState {
    fn into(self) -> bool {
        match self {
            JobReadyState::NOTTESTED => false,
            JobReadyState::VERIFIED => true,
        }
    }
}

impl JobReadyState {
    pub const fn const_default() -> Self {
        JobReadyState::NOTTESTED
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PowerShellCtx {
    pub(crate) name: &'static str,
    pub(crate) explination: &'static str,
    pub(crate) category: JobCategory,
    pub(crate) list_of_commands: &'static [StaticPowerShellCommand],
    pub(crate) tested: JobReadyState,
}

impl ExecutableJob for PowerShellCtx {
    fn tested(&self) -> JobReadyState {
        self.tested
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn jobs_steps(&self) -> impl Iterator<Item = JobStep> {
        self.list_of_commands.iter().map(|f| JobStep {
            command: f.cmd.to_string(),
            require_admin: f.requires_admin.into(),
            post_fn: None,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RegKeyType {
    DWORD,
    QWORD,
    STRING,
}

impl RegKeyType {
    pub fn to_string(&self) -> &'static str {
        match self {
            RegKeyType::DWORD => "DWORD",
            RegKeyType::QWORD => "QWORD",
            RegKeyType::STRING => "STRING",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegKey {
    pub(crate) path: &'static str,
    pub(crate) name: &'static str,
    pub(crate) value: &'static str,
    pub(crate) key_type: RegKeyType,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PowerShellRegKeyCtx {
    pub(crate) name: &'static str,
    pub(crate) explination: &'static str,
    pub(crate) category: JobCategory,
    pub(crate) reg_keys: &'static [RegKey],
    pub(crate) require_admin: bool,
    pub(crate) post_fn: Option<fn()>,
    pub(crate) tested: JobReadyState,
}

impl ExecutableJob for PowerShellRegKeyCtx {
    fn tested(&self) -> JobReadyState {
        self.tested
    }
    fn name(&self) -> &'static str {
        self.name
    }

    fn jobs_steps(&self) -> impl Iterator<Item = JobStep> {
        let mut iters: Vec<_> = self
            .reg_keys
            .iter()
            .map(|f| {
                let variables = format!(
                    "$regPath = '{}';$regName = '{}';$regValue = '{}';$regType = '{}';",
                    f.path, f.name, f.value, f.key_type.to_string()
                );

    //             let iter: Vec<_> = [
    //                 r#"if (-not (Test-Path $regPath)) {
    //             New-Item -Path $regPath -Force | Out-Null
    //         }"#,
    //                 r#"if (-not (Get-ItemProperty -Path $regPath -Name $regName -ErrorAction SilentlyContinue)) {
    //     New-ItemProperty -Path $regPath -Name $regName -Value $regValue -PropertyType $regType -Force
    // } else {
    //     Set-ItemProperty -Path $regPath -Name $regName -Value $regValue
    // }"#,
    // r#"Write-Host "✅ $regName set to $regValue in $regPath""#,
    //             ]

    let iter: Vec<_> = [
                    r#"if (-not (Test-Path $regPath)) {
                New-Item -Path $regPath -Force | Out-Null
            }
                    if (-not (Get-ItemProperty -Path $regPath -Name $regName -ErrorAction SilentlyContinue)) {
        New-ItemProperty -Path $regPath -Name $regName -Value $regValue -PropertyType $regType -Force
    } else {
        Set-ItemProperty -Path $regPath -Name $regName -Value $regValue
    }
   Write-Host "✅ $regName set to $regValue in $regPath""#,
                ]
                .iter()
                .map(move |f| JobStep {
                    command: format!("{}{}", variables, f),
                    require_admin: self.require_admin,
                    post_fn: None,
                })
                .collect();

                iter
            })
            .flatten()
            .collect();

        if let Some(post_job) = self.post_fn {
            let job_steps = vec![JobStep {
                command: "".to_string(),
                require_admin: false,
                post_fn: Some(post_job),
            }];
            iters.extend(job_steps);
        }

        iters.into_iter()
    }
}

#[derive(Clone)]
pub struct RustFunctionCtx {
    pub(crate) name: &'static str,
    pub(crate) explination: &'static str,
    pub(crate) category: JobCategory,
    pub(crate) func: fn(),
}

impl std::fmt::Debug for RustFunctionCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RustFunctionCtx")
            .field("name", &self.name)
            .field("explination", &self.explination)
            .field("category", &self.category)
            .field("func", &"<function>") // can't print closure itself
            .finish()
    }
}

impl PartialEq for RustFunctionCtx {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.explination == other.explination
            && self.category == other.category
        // && self.func == other.func
    }
}

impl ExecutableJob for RustFunctionCtx {
    fn name(&self) -> &'static str {
        self.name
    }

    fn jobs_steps(&self) -> impl Iterator<Item = JobStep> {
        // fake job_step
        (self.func)();
        let job_step = JobStep {
            command: "".to_string(),
            require_admin: false,
            post_fn: None,
        };
        vec![job_step].into_iter()
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
            command: "TODO".to_string(),
            require_admin: self.require_admin,
            post_fn: None,
        }]
        .into_iter()
    }
}

#[derive(Default, Clone, Debug)]
pub enum JobStatus {
    #[default]
    NotStarted,
    InProgress(f32),
    Failed(f32),
    Finished,
}

#[derive(Default)]
pub struct JobHandler {
    jobs_with_progress: Arc<Mutex<Vec<(Job, JobStatus)>>>,
    jobs_not_queued: Vec<Job>,
    jobs_queued: VecDeque<Job>,
}

impl JobHandler {
    pub fn set_jobs(&mut self, new_jobs: Vec<Job>) {
        let jobs_with_0_progress: Vec<(Job, JobStatus)> = new_jobs
            .iter()
            .cloned()
            .map(|job| (job, JobStatus::default()))
            .collect();
        self.jobs_with_progress = Arc::new(Mutex::new(jobs_with_0_progress));
        self.jobs_not_queued = new_jobs;
    }

    pub fn get_jobs(&self) -> impl Iterator<Item = Job> {
        let mutex = self.jobs_with_progress.lock();
        let jobs: Vec<Job> = mutex.iter().map(|(a, _b)| a.clone()).collect();
        jobs.into_iter()
    }

    pub fn get_job_progress(&self) -> Vec<(Job, JobStatus)> {
        let mutex = self.jobs_with_progress.lock();
        mutex.iter().map(|(a, b)| (a.clone(), b.clone())).collect()
    }

    pub fn finished(&self) -> bool {
        self.get_job_progress()
            .iter()
            .map(|f| &f.1)
            .all(|f| match f {
                JobStatus::InProgress(_) => false,
                JobStatus::Failed(_) => true,
                JobStatus::Finished => true,
                JobStatus::NotStarted => false,
            })
    }

    pub fn update(&mut self) {
        const MAX_QUEUED_JOBS: usize = 100;
        while self.jobs_not_queued.len() > 0 && self.jobs_queued.len() <= MAX_QUEUED_JOBS {
            let popped = self.jobs_not_queued.pop().unwrap();
            self.jobs_queued.push_back(popped);
        }

        let jobs_to_queue = self.jobs_queued.clone();
        self.jobs_queued.clear();

        let update_job_progress = |complete_job_mutex: Arc<Mutex<Vec<(Job, JobStatus)>>>,
                                   job: &Job,
                                   new_progress: JobStatus| {
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
                        Err(e) => {
                            log::error!("{e}");
                            break;
                        }
                    }
                    let new_progress = i as f32 / (job_count as f32);
                    update_job_progress(
                        complete_job_mutex.clone(),
                        &job,
                        JobStatus::InProgress(new_progress),
                    );
                }

                update_job_progress(complete_job_mutex, &job, JobStatus::Finished);
                log::trace!("thread finished {:?}", &job);
            });
        }
    }
}
