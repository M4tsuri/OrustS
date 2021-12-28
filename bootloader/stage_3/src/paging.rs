use i386::mem::paging::{
    pae::{PDEntry, PDTable, PDPTable, PDPTEntry, PAEPaging}, 
    PATMemoryType, Paging
};

/// kernel occupies 3 2MiB pages, this value can be adjusted accordingly
#[allow(dead_code)]
const KERNEL_PAGENUM: usize = 2;
const MB: u64 = 1 << 20;

/// 4MB kernel PDT page table entry (directly map virtual address to the same physical address)
static KERNEL_PDT: PDTable = PDTable::with_entries([
    PDEntry::new_page(
        true, 
        false, 
        PATMemoryType::new(false, false, false), 
        false, 
        0 * MB, 
        false
    ),
    PDEntry::new_page(
        true, 
        false, 
        PATMemoryType::new(false, false, false), 
        false, 
        2 * MB, 
        false
    )
]);

/// kernel top level page table
static mut KERNEL_PDPT: PDPTable = PDPTable::new();

pub static KERNEL_PAGING: PAEPaging = PAEPaging::new(unsafe { &KERNEL_PDPT });

pub fn enable_paging() {
    unsafe { KERNEL_PDPT.entries[0] = PDPTEntry::new(
        PATMemoryType::new(false, false, false), 
        &KERNEL_PDT as *const PDTable as u64
    )};

    unsafe { KERNEL_PDPT.entries[1] = PDPTEntry(0xffffffffffffffff); }
    KERNEL_PAGING.enable();
}
