use std::collections::HashMap;
use sysinfo::Pid;
use sysinfo::Process;

pub struct Processes<'s> {
    pub sys: &'s mut sysinfo::System,
}

pub struct AllProcessOut<'p> {
    pub processes: &'p HashMap<Pid, Process>,
}

impl Processes<'_> {
    pub fn all_processes(&mut self) -> AllProcessOut {
        // Cleanup: only refresh whats needed
        let refresh_kind = sysinfo::ProcessRefreshKind::everything();
        self.sys.refresh_processes_specifics(refresh_kind);

        let processes = self.sys.processes();

        AllProcessOut { processes }
    }
}
