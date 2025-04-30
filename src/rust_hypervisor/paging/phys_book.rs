/// phys_book.rs
///
/// author: Nick Cochran
/// email: nickcochran02@gmail.com
///
/// This file contains the `PhysBook` struct and a new function for it.

use std::collections::BTreeMap;
use crate::rust_hypervisor::paging::declarations::*;


/// Struct to hold details on physical pages
pub struct PhysBook {
    pub mem_start: PhysAddress, // Address of the start of the available memory
    pub num_total_pages: NumPages, // Total number of pages in that memory
    pub free_pages: Box<BTreeMap<PhysAddress, NumPages>>, // Map of free pages from the start address to the number of pages
    pub used_pages: Box<BTreeMap<PhysAddress, NumPages>>, // Map of used pages from the start address to the number of pages
    pub flags: usize // Variable to hold flags as needed
}



impl PhysBook {
    pub fn new() -> PhysBook {
        PhysBook {
            mem_start: 0,
            num_total_pages: 0,
            free_pages: Box::new(BTreeMap::new()),
            used_pages: Box::new(BTreeMap::new()),
            flags: 0
        }
    }
}