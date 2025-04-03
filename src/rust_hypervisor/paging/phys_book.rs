use std::collections::BTreeMap;
use crate::rust_hypervisor::paging::declarations::Address;

pub type NumPages = usize;


/// Struct to hold physical pages
pub struct PhysBook/*<'phys_mem>*/ {
    pub mem_start: usize,
    pub num_total_pages: usize,
    pub free_pages: Box<BTreeMap<Address, NumPages>>,
    pub used_pages: Box<BTreeMap<Address, NumPages>>,
    pub flags: usize
}



impl PhysBook/*<'static>*/ {
    pub fn new() -> PhysBook/*<'static>*/ {
        PhysBook {
            mem_start: 0,
            num_total_pages: 0,
            free_pages: Box::new(BTreeMap::new()), // FIXME maybe this is correct?
            used_pages: Box::new(BTreeMap::new()),
            flags: 0
        }
    }
}