use serde::Serialize;
use sysinfo::{Pid, Process};

pub struct Watcher<'a> {
    sys: &'a mut sysinfo::System,
}

impl<'a> Watcher<'a> {
    pub fn new(sys: &'a mut sysinfo::System) -> Self {
        Self { sys }
    }
}

pub type AllProcessOut = Vec<Pid>;
pub type ProcessInfoOut<'a> = anyhow::Result<&'a Process>; // ProcessError>;
pub type ProcessRuntimeOut = Result<u64, ProcessError>;

// Error related to processes
#[derive(Debug, Serialize, thiserror::Error)]
pub enum ProcessError {
    #[error("No process found")]
    NoProcess,
}

impl Watcher<'_> {
    fn get_process(&self, id: Pid) -> Result<&sysinfo::Process, ProcessError> {
        self.sys.process(id).ok_or(ProcessError::NoProcess)
    }

    pub fn all_processes(&mut self) -> AllProcessOut {
        // Cleanup: only refresh whats needed
        let refresh_kind = sysinfo::ProcessRefreshKind::everything();
        self.sys.refresh_processes_specifics(refresh_kind);

        self.sys
            .processes()
            .into_iter()
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>()
    }

    pub fn process_info(&mut self, id: Pid) -> ProcessInfoOut {
        let refresh_kind = sysinfo::ProcessRefreshKind::everything();
        self.sys.refresh_processes_specifics(refresh_kind);

        let info = self.get_process(id)?;
        Ok(info)
    }

    pub fn runtime(&mut self, id: Pid) -> ProcessRuntimeOut {
        let refresh_kind = sysinfo::ProcessRefreshKind::everything();
        self.sys.refresh_processes_specifics(refresh_kind);

        let runtime = self.get_process(id)?.run_time();
        Ok(runtime)
    }
}
