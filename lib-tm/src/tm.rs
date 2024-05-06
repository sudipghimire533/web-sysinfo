pub use sysinfo;
use sysinfo::RefreshKind;

/// Type alias to use outside this library
pub type Sysinfo = sysinfo::System;
pub type ProcessId = usize;

pub fn new_sysinfo() -> Sysinfo {
    Sysinfo::new_with_specifics(RefreshKind::new())
}
