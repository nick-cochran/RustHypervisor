/// arch_paging_access.rs
///
/// author: Nick Cochran
/// email: nickcochran02@gmail.com
///
/// This file contains the trait that can be used as the access between the architecture specific
/// and architecture agnostic code.


use crate::rust_hypervisor::paging;
use paging::virt_book::VirtBook;
use paging::declarations::{PageTable, PageTableEntry};
use crate::rust_hypervisor::paging::declarations::*;

/// Trait for accessing the architecture's paging system.
/// This trait is how the architecture agnostic code interacts with code specific to an architecture.
///
/// This is currently implemented for my simple page allocation system,
/// but with likely need to be altered and expanded to better fit the needs of all architectures
/// and a more complex and robust paging system.
pub trait ArchPagingAccess<ACD, ECD> {

    /// initialize the architecture specific paging structure that can be used by the hypervisor
    fn arch_paging_init(&self, virt_book : &mut VirtBook) -> Result<(), u8>;

    /// get the page size used for the architecture
    fn get_arch_page_size(&self) -> usize;

    /// get the size of the hypervisor memory
    fn get_hv_mem_size(&self) -> usize;

    /// get the start address of the hypervisor memory
    fn get_hv_mem_start(&self) -> PhysAddress;

    /// get the page table entry for a given virtual address
    fn get_entry(&self, page_table: PageTable, virt_addr: VirtAddress) -> PageTableEntry;

    /// check if the page table entry is valid
    fn is_entry_valid(&self, pte: PageTableEntry, flags: usize) -> bool;

    /// get the physical address for a given page table entry and virtual address
    fn get_phys(&self, pte: PageTableEntry, virt_addr: VirtAddress) -> PhysAddress;

    // create the mapping to a virtual address for a newly allocated chapter (set of pages)
    fn create_mapping(&self, phys_addr: &Vec<PhysAddress>, num_pages_used: NumPages) -> Result<VirtAddress, u8>;

    /// check if the page table is empty
    fn is_page_table_empty(&self, pt: PageTable) -> bool;
}