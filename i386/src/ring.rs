/// Module for privilege management

/// Privilege level on Intel platform.
///
/// - Ring0 is for OS Kernel.
/// - Ring1 and Ring2 are for OS Services
/// - Ring3 is for user mode applications
///
/// See *Intel Developer Manual Vol. 3A 5-6 5.5 PRIVILEGE LEVELS*
#[repr(u8)]
pub enum Privilege {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3
}
