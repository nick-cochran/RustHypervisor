/// declarations.rs
///
/// author: Nick Cochran
/// email: nickcochran02@gmail.com
///
/// This file contains many types, variables, and small helper functions
/// that are used throughout the hypervisor code.

pub type PhysAddress = usize;
pub type VirtAddress = usize;

pub type NumPages = usize;

// in theory, PTE should be externally typed by arch
pub type PageTableEntry = (PhysAddress, usize); // tuple of usize for address and size
pub type PageTable = PageTableEntry;

pub static FIRST_INPUT_INDEX : usize = 0;
pub static SECOND_INPUT_INDEX : usize = 1;
pub static THIRD_INPUT_INDEX : usize = 2;

pub static SINGLE_CPU_ID : usize = 0;

pub static NOT_ENOUGH_MEMORY: u8 = 1;
pub static CANNOT_FIND_ADDRESS : u8 = 2;
pub static FAILED_TO_READ_LINE : u8 = 3;

pub static CMD_LINE_ARGS_NO_FILE : usize = 1;
pub static _PROGRAM_NAME_IDX : usize = 0;
pub static FILE_NAME_IDX : usize = 1;

pub fn num_pages(size: usize, page_size: usize) -> usize {
    (size + page_size-1) / page_size
}