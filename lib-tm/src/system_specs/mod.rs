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
    pub cpu_vendor_id: String,
    pub cpu_name: String,
    pub cpu_brand: String,
    pub physical_core_count: Option<usize>,
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
        let sysinfo = &self.sysinfo;
        let cpu = sysinfo.global_cpu_info();
        SystemSpecs {
            available_memory: sysinfo.available_memory(),
            total_swap: sysinfo.total_swap(),
            system_name: sysinfo::System::host_name(),
            kernel_version: sysinfo::System::kernel_version(),
            os_version: sysinfo::System::os_version(),
            long_os_version: sysinfo::System::long_os_version(),
            distribution_id: sysinfo::System::distribution_id(),
            cpu_arch: sysinfo::System::cpu_arch(),
            cpu_brand: cpu.brand().to_string(),
            cpu_vendor_id: cpu.vendor_id().to_string(),
            cpu_name: cpu.name().to_string(),
            physical_core_count: sysinfo.physical_core_count(),
        }
    }

    pub fn get_system_stats(&mut self) -> SystemStats {
        let sysinfo = &self.sysinfo;
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
