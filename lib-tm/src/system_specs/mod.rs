use serde::Serialize;

#[derive(Debug)]
pub struct Watcher<'a> {
    sysinfo: &'a mut sysinfo::System,
}

impl<'a> Watcher<'a> {
    pub fn new(sysinfo: &'a mut sysinfo::System) -> Self {
        Self { sysinfo }
    }
}

#[derive(Debug, Serialize)]
pub struct SystemSpecs {
    pub available_memory: u64,
    pub total_swap: u64,
    pub system_name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub long_os_version: Option<String>,
    pub distribution_id: String,
    pub cpu_arch: Option<String>,
}

impl Watcher<'_> {
    pub fn get_system_specs(&mut self) -> SystemSpecs {
        let sysinfo = &self.sysinfo;
        SystemSpecs {
            available_memory: sysinfo.available_memory(),
            total_swap: sysinfo.total_swap(),
            system_name: sysinfo::System::host_name(),
            kernel_version: sysinfo::System::kernel_version(),
            os_version: sysinfo::System::os_version(),
            long_os_version: sysinfo::System::long_os_version(),
            distribution_id: sysinfo::System::distribution_id(),
            cpu_arch: sysinfo::System::cpu_arch(),
        }
    }
}
