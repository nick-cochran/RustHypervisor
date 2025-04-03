

pub fn num_pages(size: usize, page_size: usize) -> usize {
    (size + page_size-1) / page_size
}

pub type Address = usize;

// in theory, PTE should be externally typed by arch
pub type PageTableEntry = (Address, usize); // tuple of usize for addr and size
pub type PageTable = PageTableEntry;

