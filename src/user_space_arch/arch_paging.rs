/// arch_paging.rs
///
/// author: Nick Cochran
/// email: nickcochran02@gmail.com
///
/// This file contains my simple implementation of the paging system access trait
/// that serves as the simple user space "architecture" for testing the hypervisor.

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use lazy_static::lazy_static;
use crate::exec::EcdBase;
use crate::user_space_arch::arch::AcdUserLevel;

use crate::rust_hypervisor;
use rust_hypervisor::paging;
use paging::arch_paging::ArchPaging;
use paging::arch_paging_access::ArchPagingAccess;
use paging::virt_book::VirtBook;
use paging::declarations::{PhysAddress, PageTable, PageTableEntry};
use crate::rust_hypervisor::paging::declarations::*;

lazy_static! {
    /// Global Arc Mutex to hold all available memory.
    ///  –> this depends on the existence of an allocator because Vec is in the alloc crate
    pub static ref HV_MEM : Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
}

// global static variable to hold the current virtual address
// as the current implementation simply just keeps counting up under the assumption that
// there is endless virtual space.
pub static CURR_VIRT_ADDR: AtomicUsize = AtomicUsize::new(1);


impl ArchPagingAccess<AcdUserLevel, EcdBase> for ArchPaging<AcdUserLevel, EcdBase> {
    fn arch_paging_init(&self, _virt_book : &mut VirtBook) -> Result<(), u8> {
        let mut phys_mem = HV_MEM.lock().unwrap();
        phys_mem.reserve(self.get_hv_mem_size());

        Ok(())
    }

    fn get_arch_page_size(&self) -> usize {
        1
    }

    fn get_hv_mem_size(&self) -> usize {
        1 << 9
    }

    fn get_hv_mem_start(&self) -> PhysAddress {
        // this doesn't necessarily return the correct address, but it works for my purposes
        HV_MEM.lock().unwrap().as_ptr() as PhysAddress
    }

    fn get_entry(&self, _page_table: PageTable, _virt_addr: VirtAddress) -> PageTableEntry {
        // not used at this time.
        (2, 1000)
    }

    fn is_entry_valid(&self, _pte: PageTableEntry, _flags: usize) -> bool {
        // not used at this time.
        true
    }

    fn get_phys(&self, _pte: PageTableEntry, virt_addr: VirtAddress) -> PhysAddress {
        // not used at this time.
        virt_addr
    }

    /// return the virtual address for a given physical address
    fn create_mapping(&self, _phys_addr: &Vec<PhysAddress>, num_pages_used: NumPages) -> Result<VirtAddress, u8> {
        // currently: add the number of pages used so far to the address, assuming endless virtual space
        let curr_virt_addr = CURR_VIRT_ADDR.load(Ordering::SeqCst);
        CURR_VIRT_ADDR.store(curr_virt_addr + (num_pages_used * self.page_size), Ordering::SeqCst);

        Ok(curr_virt_addr)
    }

    fn is_page_table_empty(&self, _pt: PageTable) -> bool {
        // not used at this time.
        false
    }
}