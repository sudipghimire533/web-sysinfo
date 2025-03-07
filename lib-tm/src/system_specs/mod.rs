use serde::Serialize;

#[derive(Debug)]
pub struct Watcher<'a> {
    sys: &'a mut sysinfo::System,
}

impl<'a> Watcher<'a> {
    pub fn new(sysinfo: &'a mut sysinfo::System) -> Self {
        Self { sys: sysinfo }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiskSpecs {
    pub name: String,
    pub total_space: u64,
    pub available_space: u64,
    pub file_system: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CpuSepcs {
    pub brand: String,
    pub name: String,
    pub vendor_id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemSpecs {
    pub total_memory: u64,
    pub total_swap: u64,
    pub system_name: Option<String>,
    pub kernel_version: Option<String>,
    pub os_version: Option<String>,
    pub long_os_version: Option<String>,
    pub distribution_id: String,
    pub cpu_arch: Option<String>,
    pub physical_core_count: Option<usize>,
    pub cpus_info: Vec<CpuSepcs>,
    pub disks_info: Vec<DiskSpecs>,
}

#[derive(Debug, Serialize)]
pub struct SystemStats {
    pub boot_time: u64,
    pub free_memory: u64,
    pub available_memory: u64,
    pub cpu_frequency: u64,
    pub cpu_usage: f32,
    pub used_swap: u64,
    pub free_swap: u64,
    pub used_memory: u64,
    pub uptime: u64,
    pub load_average: sysinfo::LoadAvg,
}

impl Watcher<'_> {
    pub fn get_system_specs(&mut self) -> SystemSpecs {
        let refresh_kind = sysinfo::RefreshKind::everything();
        self.sys.refresh_specifics(refresh_kind);

        let sysinfo = &self.sys;
        SystemSpecs {
            total_memory: sysinfo.total_memory(),
            total_swap: sysinfo.total_swap(),
            system_name: sysinfo::System::host_name(),
            kernel_version: sysinfo::System::kernel_version(),
            os_version: sysinfo::System::os_version(),
            long_os_version: sysinfo::System::long_os_version(),
            distribution_id: sysinfo::System::distribution_id(),
            cpu_arch: sysinfo::System::cpu_arch(),
            physical_core_count: sysinfo.physical_core_count(),
            cpus_info: sysinfo
                .cpus()
                .iter()
                .map(|cpu| CpuSepcs {
                    brand: cpu.brand().to_string(),
                    name: cpu.name().to_string(),
                    vendor_id: cpu.vendor_id().to_string(),
                })
                .collect(),
            disks_info: sysinfo::Disks::new_with_refreshed_list()
                .list()
                .iter()
                .filter_map(|disk| match disk.kind() {
                    sysinfo::DiskKind::HDD | sysinfo::DiskKind::SSD => Some(DiskSpecs {
                        name: disk.name().to_string_lossy().to_string(),
                        total_space: disk.total_space(),
                        available_space: disk.available_space(),
                        file_system: disk.file_system().to_string_lossy().to_string(),
                    }),
                    sysinfo::DiskKind::Unknown(_) => None,
                })
                .collect(),
        }
    }

    pub fn get_system_stats(&mut self) -> SystemStats {
        let refresh_kind = sysinfo::ProcessRefreshKind::everything();
        self.sys.refresh_processes_specifics(refresh_kind);

        let sysinfo = &self.sys;
        let cpu = sysinfo.global_cpu_info();
        SystemStats {
            boot_time: sysinfo::System::boot_time(),
            free_memory: sysinfo.free_memory(),
            available_memory: sysinfo.available_memory(),
            cpu_frequency: cpu.frequency(),
            cpu_usage: cpu.cpu_usage(),
            used_swap: sysinfo.used_swap(),
            free_swap: sysinfo.free_swap(),
            used_memory: sysinfo.used_memory(),
            uptime: sysinfo::System::uptime(),
            load_average: sysinfo::System::load_average(),
        }
    }
}
