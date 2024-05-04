use serde::Serialize;
use sysinfo::Pid;

pub struct Watcher<'a> {
    sys: &'a mut sysinfo::System,
}

impl<'a> Watcher<'a> {
    pub fn new(sys: &'a mut sysinfo::System) -> Self {
        Self { sys }
    }
}

pub type AllProcessOut = Vec<u32>;
pub type ProcessNameOut = Result<String, ProcessError>;
pub type ProcessRuntimeOut = Result<u64, ProcessError>;

// Error related to processes
#[derive(Debug, Serialize)]
pub enum ProcessError {
    /// certain process cannot be found
    NoProcess,
    // Error related to serilization
    Serilizarion(String),
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
            .iter()
            .map(|(id, _)| id.as_u32())
            .collect::<Vec<_>>()
    }

    pub fn name(&mut self, id: Pid) -> ProcessNameOut {
        let refresh_kind = sysinfo::ProcessRefreshKind::everything();
        self.sys.refresh_processes_specifics(refresh_kind);

        let name = self.get_process(id)?.name();
        Ok(name.to_string())
    }

    pub fn runtime(&mut self, id: Pid) -> ProcessRuntimeOut {
        let refresh_kind = sysinfo::ProcessRefreshKind::everything();
        self.sys.refresh_processes_specifics(refresh_kind);

        let runtime = self.get_process(id)?.run_time();
        Ok(runtime)
    }
}
