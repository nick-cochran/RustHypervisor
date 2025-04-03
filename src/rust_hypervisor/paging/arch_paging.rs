use crate::rust_hypervisor::paging;
use paging::virt_book::VirtBook;
use paging::declarations::{PageTable, PageTableEntry};
use crate::rust_hypervisor::paging::declarations::Address;

pub trait ArchPaging<ACD, ECD> {
    fn arch_paging_init(&self, virt_book : VirtBook) -> Result<(), u8>;
    fn get_arch_page_size(&self) -> usize;
    fn get_hv_mem_size(&self) -> usize;
    fn get_hv_mem_start(&self) -> Address;
    fn get_entry(&self, page_table: PageTable, virt_addr: Address) -> PageTableEntry;
    fn is_entry_valid(&self, pte: PageTableEntry, flags: usize) -> bool;
    fn get_phys(&self, pte: PageTableEntry, virt_addr: Address) -> Address;
    // create the mapping to a virtual address for a newly allocated chapter (set of pages)
    fn create_mapping(&self, phys_addr: Address) -> Result<Address, u8>;

    fn is_page_table_empty(&self, pt: PageTable) -> bool;
}

// 3, 5, 7, 17