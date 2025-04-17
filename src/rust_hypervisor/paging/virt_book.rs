/// TODO file comment

use crate::rust_hypervisor::paging::declarations::PageTableEntry;


/// VirtBook currently does not do anything.
/// The idea came from remapping structure in Jailhouse which used virtual addresses
/// from the same struct that also was used for what is now PhysBook.
///
/// I do not know if this will be used, but I have opted to leave it in the codebase
/// in case a use for it shows up later.  I see this as a real possibility, but it depends
/// on what happens when the connection with architectures is fully fleshed out.

pub struct VirtBook<'virt_mem> {
    pub pages: &'virt_mem [PageTableEntry],
    pub num_pages: usize,
    pub num_used_pages: usize,
    pub used_pages_bitmap: &'virt_mem [u8],
    pub flags: usize
}

impl<'virt_mem> VirtBook<'virt_mem> {
    pub fn new() -> VirtBook<'virt_mem> {
        VirtBook {
            pages: &[],
            num_pages: 0,
            num_used_pages: 0,
            used_pages_bitmap: &[],
            flags: 0,
        }
    }
}