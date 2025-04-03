use crate::rust_hypervisor::paging::declarations::PageTableEntry;




pub struct VirtBook<'virt_mem> { // --> this will be passed off to arch to build their paging system
    pub pages: &'virt_mem [PageTableEntry], // this will be statically sized to something smaller like 8 bits so it
                                            // doesn't need to be dynamically size changed to hold more PTEs
                                            // TODO add this idea to notes doc
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