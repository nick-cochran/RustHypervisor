/// arch_paging.rs
///
/// author: Nick Cochran
/// email: nickcochran02@gmail.com
///
/// This file contains the `ArchPaging` struct which has the information on the paging
/// system for the architecture.

use std::marker::PhantomData;
use crate::exec::EcdBase;
use crate::rust_hypervisor::paging::declarations::PageTable;
use crate::user_space_arch::arch::AcdUserLevel;


/// The `ArchPaging` struct which holds the architecture-specific paging information.
/// and serves as the struct to use with the `ArchPagingAccess` trait.
/// It also uses the `ACD` and `ECD` generic types to allow for combinations of
/// different architecture and execution context descriptors that are set
/// by the architecture the hypervisor is running on top of.
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