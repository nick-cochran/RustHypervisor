/// TODO file comment

use std::sync::{Arc, Mutex};
use std::time::Duration;
use lazy_static::lazy_static;

pub fn num_pages(size: usize, page_size: usize) -> usize {
    (size + page_size-1) / page_size
}

pub type Address = usize;

// in theory, PTE should be externally typed by arch
pub type PageTableEntry = (Address, usize); // tuple of usize for address and size
pub type PageTable = PageTableEntry;

lazy_static! {
    pub static ref OP_TIME : Arc<Mutex<Duration>> = Arc::new(Mutex::new(Duration::new(0, 0)));
    pub static ref ALLOC_TIME : Arc<Mutex<Duration>> = Arc::new(Mutex::new(Duration::new(0, 0)));
    pub static ref FREE_TIME : Arc<Mutex<Duration>> = Arc::new(Mutex::new(Duration::new(0, 0)));
}

pub static NOT_ENOUGH_ROOM : u8 = 1;

