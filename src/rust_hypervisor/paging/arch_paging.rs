/// TODO file comment

use std::marker::PhantomData;
use crate::exec::EcdBase;
use crate::rust_hypervisor::paging::declarations::PageTable;
use crate::user_space_arch::arch::AcdUserLevel;


pub struct ArchPaging<ACD = AcdUserLevel, ECD = EcdBase> {
    pub page_size: usize, // Size of the page in bytes
    pub root_table: PageTable, // Root page table for the architecture
    _acd_marker: PhantomData<ACD>, // Phantom data marker for the architecture context descriptor
    _ecd_marker: PhantomData<ECD> // Phantom data marker for the execution context descriptor
}


impl<ACD, ECD> ArchPaging<ACD, ECD> {
    pub fn new() -> ArchPaging<ACD, ECD> {
        ArchPaging {
            page_size: 0,
            root_table: (0, 0),
            _acd_marker: PhantomData,
            _ecd_marker: PhantomData,
        }
    }
}