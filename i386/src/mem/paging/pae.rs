//! The implementation of PAE paging. In this paging mode, page table maps a 
//! linear address to either a 4KiB page or a 2MiB page. We support 32-bit logic address
//! and physical address up to 52-bit. 

use crate::{
    utils::bitwise::mask_assign,
    instrs::{CR0_PG, CR4_PAE},
    mem::{PhysAddr, info::MemRange, VirtAddr}
};
use core::arch::asm;
use super::{Paging, PATMemoryType};

/// The number of PDPTEs in Page Directory Pointer Table, according to 
/// *Intel Developer Manual Vol. 3A 4-13*, this should be 4.
/// First Level
const PDPTE_NUM: usize = 1 << 2;
/// Second Level
const PDE_NUM: usize = 1 << 9;
/// Third Level
const PTE_NUM: usize = 1 << 9;

/// page table entry present mask
const ENTRY_PRESENT: u64 = 1;

#[derive(Clone, Copy)]
pub struct PTEntry(u64);

impl PTEntry {
    pub const fn new(
        writable: bool,
        // if false, user mode cannot access this pag
        user_access: bool,
        // memory cache type
        mem_ty: PATMemoryType,
        // if cr4.PGE = 1, determine the translation is global, otherwise must be 0
        global: bool,
        page_phys: PhysAddr,
        xd: bool
    ) -> Self {
        let mut entry = ENTRY_PRESENT;
        entry = mask_assign(entry, writable as u64, 1, 0, 1);
        entry = mask_assign(entry, user_access as u64, 2, 0, 1);
        entry = mask_assign(entry, mem_ty.pwt as u64, 3, 0, 1);
        entry = mask_assign(entry, mem_ty.pcd as u64, 4, 0, 1);
        entry = mask_assign(entry, mem_ty.pat as u64, 7, 0, 1);
        entry = mask_assign(entry, global as u64, 8, 0, 1);
        entry = mask_assign(entry, page_phys as u64, 12, 0, 51);
        entry = mask_assign(entry, xd as u64, 63, 0, 1);
        Self(entry)
    }
}

/// The third level page table
#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct PTable {
    pub entries: [PTEntry; PTE_NUM]
}

impl Default for PTable {
    fn default() -> Self {
        Self { entries: [PTEntry(0); PTE_NUM] }
    }
}

#[derive(Clone, Copy)]
pub struct PDTEntry(u64);

impl PDTEntry {
    /// create a PDT entry representing a 2MiB page
    pub const fn new_page(
        writable: bool,
        // if false, user mode cannot access this pag
        user_access: bool,
        // memory cache type
        mem_ty: PATMemoryType,
        // if cr4.PGE = 1, determine the translation is global, otherwise must be 0
        global: bool,
        page_phys: PhysAddr,
        xd: bool
    ) -> Self {
        let mut entry = ENTRY_PRESENT;
        entry = mask_assign(entry, writable as u64, 1, 0, 1);
        entry = mask_assign(entry, user_access as u64, 2, 0, 1);
        entry = mask_assign(entry, mem_ty.pwt as u64, 3, 0, 1);
        entry = mask_assign(entry, mem_ty.pcd as u64, 4, 0, 1);
        entry = mask_assign(entry, 1, 7, 0, 1);
        entry = mask_assign(entry, global as u64, 8, 0, 1);
        entry = mask_assign(entry, mem_ty.pat as u64, 12, 0, 1);
        entry = mask_assign(entry, page_phys as u64, 21, 0, 42);
        entry = mask_assign(entry, xd as u64, 63, 0, 1);
        Self(entry)
    }

    /// create a PDT entry representing a Page Table Entry
    pub const fn new_entry(
        writable: bool,
        user_access: bool,
        mem_ty: PATMemoryType,
        page_phys: PhysAddr,
        xd: bool
    ) -> Self {
        let mut entry = ENTRY_PRESENT;
        entry = mask_assign(entry, writable as u64, 1, 0, 1);
        entry = mask_assign(entry, user_access as u64, 2, 0, 1);
        entry = mask_assign(entry, mem_ty.pwt as u64, 3, 0, 1);
        entry = mask_assign(entry, mem_ty.pcd as u64, 4, 0, 1);
        entry = mask_assign(entry, 0, 7, 0, 1);
        entry = mask_assign(entry, page_phys as u64, 12, 0, 51);
        entry = mask_assign(entry, xd as u64, 63, 0, 1);
        Self(entry)
    }
}

#[repr(align(4096))]
pub struct PDTable {
    pub entries: [PDTEntry; PDE_NUM]
}

impl Default for PDTable {
    fn default() -> Self {
        Self { entries: [PDTEntry(0); PDE_NUM] }
    }
}

/// This data structure encodes a pointer to a Page Directory
#[derive(Clone, Copy)]
pub struct PDPTEntry(u64);

impl PDPTEntry {
    pub const fn new(mem_ty: PATMemoryType, page_dir_phys: u64) -> Self {
        let mut entry = ENTRY_PRESENT;
        entry = mask_assign(entry, mem_ty.pwt as u64, 3, 0, 1);
        entry = mask_assign(entry, mem_ty.pcd as u64, 4, 0, 1);
        entry = mask_assign(entry, page_dir_phys, 12, 0, 52);
        Self(entry)
    }
}

/// Page Directory Pointer Table, their are 4 PDPTEs in this table, each of them
/// represents a 1GiB memory range. 
#[repr(align(32))]
pub struct PDPTable {
    pub entries: [PDPTEntry; PDPTE_NUM]
}

impl Default for PDPTable {
    fn default() -> Self {
        Self { entries: [PDPTEntry(0); PDPTE_NUM] }
    }
}

pub enum PagingError {
    MapError
}

impl PDPTable {
    pub fn add_map(virt: MemRange<VirtAddr>, phys: MemRange<PhysAddr>) -> Result<(), PagingError> {
        if virt.len != phys.len as VirtAddr {
            return Err(PagingError::MapError)
        }
        // TODO
        Ok(())
    }
}

pub struct PAEPaging<'a> {
    page_table: &'a PDPTable
}

impl<'a> PAEPaging<'a> {
    pub fn new(table: &'a PDPTable) -> Self {
        Self { page_table: table }
    }
}

impl<'a> Paging for PAEPaging<'a> {
    /// According to *Intel Developer Manual 4-1 Vol. 3A: 
    /// To enable PAE paging mode, we need to set 
    /// CR0.PG = 1, CR4.PAE = 1 and IA32_EFER.LME = 0.
    /// So we set PG and PAE in this function.
    /// This process can be done in protect mode.
    /// FIXME: In this function, we do not check LME bit, which can lead to 
    /// potential error.
    fn enable(&self) {
        unsafe {
            asm!(
                // Firstly, we need to make sure that the page table is properly set.
                // Then we load the physical address of PDPT into cr3.
                "mov cr3, {page_table}",
                // Finally, we enable PAE paging mode
                // FIXME: complete these operations with encapsulated control register
                // operations.
                // FIXME: add support for SMAP and SMEP
                "mov eax, cr0",
                "or eax, {PG}",
                "mov cr0, eax",
                "mov eax, cr4",
                "or eax, {PAE}",
                "mov cr4, eax",
                page_table = in(reg) self.page_table,
                PG = const CR0_PG,
                PAE = const CR4_PAE,
                out("eax") _
            )
        }
    }

    /// in PAE paging mode, the PDPTE registers are reloaded on mov to cr3.
    /// See *Intel Developer Manual Vol. 3A 4-13*
    fn update(&self) {
        unsafe {
            asm!(
                "mov cr3, {page_table}",
                page_table = in(reg) self.page_table
            )
        }
    }
}
