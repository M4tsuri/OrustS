/// The structure describing the state of a currently executing task.
/// During a task switching, the state of current task is saved into a TSS, 
/// then the TSS of dispatched task is loaded, CPU will execute the dispatched task
/// from EIP specified in the TSS. The TSS of callee will also save 
/// the TSS descriptor of its caller.
#[repr(packed)]
#[allow(unused)]
pub struct TSS {
    /// shadow stack pointer
    ssp: u32,
    io_base: u16,
    _pad1: u16,
    ldt_selector: u16,
    _pad2: u16,
    gs: u16,
    _pad3: u16,
    fs: u16,
    _pad4: u16,
    ds: u16,
    _pad5: u16,
    ss: u16,
    _pad6: u16,
    cs: u16,
    _pad7: u16,
    es: u16,
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    eflags: u32,
    eip: u32,
    cr3: u32,
    _pad8: u16,
    ss2: u16,
    esp2: u32,
    _pad9: u16,
    ss1: u16,
    esp1: u16,
    _pad10: u16,
    ss0: u16,
    esp0: u32,
    _pad11: u16,
    /// the tss segment selector of the caller task
    prev_task: u16
}


