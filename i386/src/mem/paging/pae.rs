//! The implementation of PAE paging. In this paging mode, page table maps a 
//! linear address to either a 4KiB page or a 2MiB page. We support 32-bit logic address
//! and physical address up to 52-bit. 

use crate::{
    utils::bitwise::mask_assign,
    instrs::{CR0_PG, CR4_PAE},
    mem::{PhysAddr, MemRange, VirtAddr}
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
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Make sure that page_phys is 4KiB aligned (1 << 12)
    pub const fn new(
        writable: bool,
        // if false, user mode cannot access this page
        user_access: bool,
        // memory cache type
        mem_ty: PATMemoryType,
        // if cr4.PGE = 1, determine the translation is global, otherwise must be 0
        global: bool,
        page_phys: PhysAddr,
        xd: bool
    ) -> Self {
        let mut entry = page_phys | ENTRY_PRESENT;
        entry = mask_assign(entry, writable as u64, 1, 0, 1);
        entry = mask_assign(entry, user_access as u64, 2, 0, 1);
        entry = mask_assign(entry, mem_ty.pwt as u64, 3, 0, 1);
        entry = mask_assign(entry, mem_ty.pcd as u64, 4, 0, 1);
        entry = mask_assign(entry, mem_ty.pat as u64, 7, 0, 1);
        entry = mask_assign(entry, global as u64, 8, 0, 1);
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

#[derive(Clone, Copy)]
pub struct PDEntry(u64);

impl PDEntry {
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Create a PDT entry representing a 2MiB page.
    /// Make sure that page_phys is 2MiB aligned (1 << 21)
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
        let mut entry = page_phys | ENTRY_PRESENT;
        entry = mask_assign(entry, writable as u64, 1, 0, 1);
        entry = mask_assign(entry, user_access as u64, 2, 0, 1);
        entry = mask_assign(entry, mem_ty.pwt as u64, 3, 0, 1);
        entry = mask_assign(entry, mem_ty.pcd as u64, 4, 0, 1);
        entry = mask_assign(entry, 1, 7, 0, 1);
        entry = mask_assign(entry, global as u64, 8, 0, 1);
        entry = mask_assign(entry, mem_ty.pat as u64, 12, 0, 1);
        entry = mask_assign(entry, xd as u64, 63, 0, 1);
        Self(entry)
    }

    /// Create a PDT entry representing a Page Table Entry.
    /// Make sure this page_phys is properly aligned (1 << 12 aligned)
    pub const fn new_table(
        writable: bool,
        user_access: bool,
        mem_ty: PATMemoryType,
        page_table_phys: PhysAddr,
        xd: bool
    ) -> Self {
        let mut entry = page_table_phys | ENTRY_PRESENT;
        entry = mask_assign(entry, writable as u64, 1, 0, 1);
        entry = mask_assign(entry, user_access as u64, 2, 0, 1);
        entry = mask_assign(entry, mem_ty.pwt as u64, 3, 0, 1);
        entry = mask_assign(entry, mem_ty.pcd as u64, 4, 0, 1);
        entry = mask_assign(entry, 0, 7, 0, 1);
        entry = mask_assign(entry, xd as u64, 63, 0, 1);
        Self(entry)
    }
}

#[repr(align(4096))]
pub struct PDTable {
    pub entries: [PDEntry; PDE_NUM]
}

/// This data structure encodes a pointer to a Page Directory
#[derive(Clone, Copy)]
pub struct PDPTEntry(pub u64);

impl PDPTEntry {
    /// get an empty entry 
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Make sure page_dir_phys is properly aligned
    pub const fn new(mem_ty: PATMemoryType, page_dir_phys: u64) -> Self {
        // It's caller's responsibility to make sure page_dir_phys is properly aligned
        let mut entry = page_dir_phys | ENTRY_PRESENT;
        entry = mask_assign(entry, mem_ty.pwt as u64, 3, 0, 1);
        entry = mask_assign(entry, mem_ty.pcd as u64, 4, 0, 1);
        Self(entry)
    }
}

/// Page Directory Pointer Table, their are 4 PDPTEs in this table, each of them
/// represents a 1GiB memory range. 
#[repr(align(32))]
pub struct PDPTable {
    pub entries: [PDPTEntry; PDPTE_NUM]
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

macro_rules! impl_page_table {
    ($table:ty, $entry:ty, $default_num:expr) => {
        impl $table {
            pub const fn new() -> Self {
                Self { entries: [<$entry>::empty(); $default_num] }
            }
        
            pub const fn with_entries<const NUM: usize>(entries: [$entry; NUM]) -> Self {
                let mut res = Self::new();
                
                let mut i = 0;
                while i < NUM {
                    res.entries[i] = entries[i];
                    i += 1;
                }
        
                res
            }
        }
    }
}

impl_page_table!(PTable, PTEntry, PTE_NUM);
impl_page_table!(PDTable, PDEntry, PDE_NUM);
impl_page_table!(PDPTable, PDPTEntry, PDPTE_NUM);

pub enum PagingError {
    MapError
}

pub struct PAEPaging<'a> {
    page_table: &'a PDPTable
}

impl<'a> PAEPaging<'a> {
    pub const fn new(table: &'a PDPTable) -> Self {
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
                // Finally, we enable PAE paging mode
                // FIXME: complete these operations with encapsulated control register
                // operations.
                // FIXME: add support for SMAP and SMEP

                // enable PAE, note that we must do this step first, or we will go through 
                // 32-bit paging
                // See *Intel Developer Manual Vol. 3A 4-3*
                "mov eax, cr4",
                "or eax, {PAE}",
                "mov cr4, eax",

                // We need to make sure that the page table is properly set.
                // Then we load the physical address of PDPT into cr3.
                "mov cr3, {page_table}",

                // enable paging
                "mov eax, cr0",
                "or eax, {PG}",
                "mov cr0, eax",
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
