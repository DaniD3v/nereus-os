pub mod index;
pub mod ptm;

pub use x86_64::structures::paging::PageTableFlags as PageEntryFlags;
pub mod flag {
    use super::PageEntryFlags;

    pub const DEFAULT_EXEC: PageEntryFlags =
        PageEntryFlags::PRESENT.union(PageEntryFlags::WRITABLE);
    pub const DEFAULT_DATA: PageEntryFlags = DEFAULT_EXEC.union(PageEntryFlags::NO_EXECUTE);
}

/// Page Directory or Page Table
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct PageEntry(u64);

impl PageEntry {
    /// Create new page entry based on address and flags
    pub fn new(address: u64, flags: PageEntryFlags) -> Self {
        let address_shifted = address & 0x000f_ffff_ffff_f000;
        let flags_bits = flags.bits();
        PageEntry(address_shifted | flags_bits)
    }

    /// Set address of page entry
    pub fn set_address(&mut self, address: u64) {
        let address = address & 0x000f_ffff_ffff_f000;
        self.0 = (self.0 & 0xfff) | address;
    }

    /// Set flags of page entry
    pub fn set_flags(&mut self, flags: PageEntryFlags) {
        let flags_bits = flags.bits() & 0xfff; // only use lower 12 bits
        self.0 = (self.0 & !0xfff) | flags_bits;
    }

    /// Get address of page entry
    pub fn address(&self) -> u64 {
        self.0 & 0x000f_ffff_ffff_f000
    }

    /// Get address of page entry
    pub fn flags(&self) -> PageEntryFlags {
        PageEntryFlags::from_bits_truncate(self.0 & 0xfff) // Mask to get only the lower 12 bits for flags
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(align(4096))]
pub struct PageTable {
    pub entries: [PageEntry; 512],
}
