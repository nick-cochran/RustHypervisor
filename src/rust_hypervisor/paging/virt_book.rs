/// virt_book.rs
///
/// author: Nick Cochran
/// email: nickcochran02@gmail.com
///
/// This file contains the `VirtBook` struct and a new function for it.


use std::collections::BTreeMap;

use crate::rust_hypervisor::paging::declarations::*;


/// This struct contains BTreeMaps that keep track of the mappings from virtual addresses
/// to physical addresses and vice versa.
pub struct VirtBook {
    /// Maps a virtual address to a chapter to the physical address of the corresponding chapters
    pub virt_page_mapping: Box<BTreeMap<VirtAddress, (NumPages, Vec<PhysAddress>)>>,
    /// Maps a physical address to the virtual address of the chapter it is in
    pub phys_page_mapping: Box<BTreeMap<PhysAddress, (NumPages, VirtAddress)>>,
    /// Variable to hold flags as needed
    pub flags: usize
}

impl VirtBook {
    pub fn new() -> VirtBook {
        VirtBook {
            virt_page_mapping: Box::new(BTreeMap::new()),
            phys_page_mapping: Box::new(BTreeMap::new()),
            flags: 0
        }
    }
}