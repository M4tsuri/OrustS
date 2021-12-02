use i386::{bios::mem::E820MemInfo, hardware::ata::pio::ATADiskInfo};
use crate::mem::MEMINFO_MAX;

/// The module holds the data structure for passing nessessary information to kernel.


pub struct KernelContext {
    pub disk_info: ATADiskInfo,
    pub mem_info: E820MemInfo<MEMINFO_MAX>
}
