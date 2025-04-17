/// TODO file comment

use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::exec::EcdBase;
use crate::user_space_arch::arch::AcdUserLevel;

use crate::rust_hypervisor;
use rust_hypervisor::paging;
use paging::arch_paging::ArchPaging;
use paging::arch_paging_access::ArchPagingAccess;
use paging::virt_book::VirtBook;
use paging::declarations::{Address, PageTable, PageTableEntry};

lazy_static! {
    /// Global Arc Mutex to hold all available memory.
    ///  –> this depends on the existence of an allocator because Vec is in the alloc crate
    pub static ref HV_MEM : Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
}


impl ArchPagingAccess<AcdUserLevel, EcdBase> for ArchPaging<AcdUserLevel, EcdBase> {
    fn arch_paging_init(&self, virt_book : VirtBook) -> Result<(), u8> {
        let mut phys_mem = HV_MEM.lock().unwrap();
        phys_mem.reserve(self.get_hv_mem_size());
        println!("{}", phys_mem.capacity());

        Ok(())
    }

    fn get_arch_page_size(&self) -> usize {
        1
    }

    fn get_hv_mem_size(&self) -> usize {
        1 << 9
    }

    fn get_hv_mem_start(&self) -> Address {
        // TODO check this
        HV_MEM.lock().unwrap().as_ptr() as Address
    }

    fn get_entry(&self, page_table: PageTable, virt_addr: usize) -> PageTableEntry { // not used yet.
        (2, 1000)
    }

    fn is_entry_valid(&self, pte: PageTableEntry, flags: usize) -> bool { // not used yet.
        true
    }

    fn get_phys(&self, pte: PageTableEntry, virt_addr: usize) -> usize { // not used yet.
        virt_addr
    }

    /// return the virtual address for a given physical address
    fn create_mapping(&self, phys_addr: Address) -> Result<Address, u8> {
        Ok(phys_addr)
    }

    fn is_page_table_empty(&self, pt: PageTable) -> bool { // not used yet.
        false
    }
}