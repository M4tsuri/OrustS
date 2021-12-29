//! The module holds the data structure for passing nessessary information to kernel.

use i386::{
    driver::mem::e820::E820MemInfo, 
    driver::disk::ata::pio::ATADiskInfo, mem::paging::Paging
};

use crate::mem::MEMINFO_MAX;


pub struct KernelContext {
    pub disk_info: ATADiskInfo,
    pub mem_info: E820MemInfo<MEMINFO_MAX>,
    pub kernel_paging: &'static dyn Paging
}
