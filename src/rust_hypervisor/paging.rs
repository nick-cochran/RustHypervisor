pub mod declarations;
pub mod phys_book;
pub mod virt_book;
pub mod paging_structure;
pub mod arch_paging;

use std::sync::atomic::Ordering;
// use std::mem::size_of;
use super::super::user_space_arch::arch::AcdUserLevel;
use super::super::exec::EcdBase;

use crate::rust_hypervisor;
// use rust_hypervisor::paging;
use arch_paging::*;
use declarations::*;
use paging_structure::*;
use phys_book::*;
use virt_book::*;
// use rust_hypervisor::hypervisor::{RustHypervisor};
use rust_hypervisor::setup::HYPERVISOR;
use crate::rust_hypervisor::setup::PAGE_SIZE;
// static ARCH_PAGE_SIZE : usize = 1 << 12; // TEMP: this will come from external arch specific code

// static PAGE_SIZE : usize = ARCH_PAGE_SIZE;


/// initialize the hypervisor paging system
/// to enable arch specific paging setup to start
pub fn paging_init() -> Result<(), u8> {

    let mut hypervisor_struct = HYPERVISOR.lock().unwrap();
    let paging_structs = &mut hypervisor_struct.rust_hypervisor_paging;

    paging_structs.paging_structure = PagingStructure::new();
    paging_structs.phys_book = PhysBook::new();

    let mut phys_book = &mut paging_structs.phys_book;
    let mut hv_paging  = &mut paging_structs.paging_structure;



    // let mut hv_paging : PagingStructure<AcdUserLevel, EcdBase> = PagingStructure::new();
    // let mut phys_book: PhysBook = PhysBook::new();

    // TODO do I want to get rid of these in favor of the global variable set by the architecture
    // FIXME YES I NEED TO DO THAT
    let page_size : usize = hv_paging.get_arch_page_size();
    hv_paging.page_size = page_size;

    let mem_size : usize = hv_paging.get_hv_mem_size();
    // this depends on the existence of an allocator because Vec is in the alloc crate
    let num_total_pages : usize = num_pages(mem_size, page_size);
    phys_book.num_total_pages = num_total_pages;
    let start_addr = hv_paging.get_hv_mem_start();
    // TODO move all_pages to arch and only do bookkeeping here
    phys_book.mem_start = start_addr;
    phys_book.free_pages.insert(start_addr, num_total_pages); // insert the whole thing into the free pages map


    /* This whole section came from Jailhouse, but I think it may end up not being what we want

    // not worrying about per_cpu, and not correct at this time
    let num_per_cpu_pages : usize = 0; // temp, currently set to 0 to ignore it
    // let per_cpu_pages: usize = hypervisor_header.max_cpus + size_of(struct per_cpu) / PAGE_SIZE;

    // not sure about this one exactly but I'll probably need something along these lines
    let num_config_pages : usize = num_pages(size_of::<RustHypervisor>(), page_size);

    let total_setup_pages = num_config_pages + num_per_cpu_pages;
    if num_total_pages <= total_setup_pages {
        return Err(1);
    }


    // set num_used_pages and used_pages_bitmap from setting up the hv
    phys_book.num_used_pages += total_setup_pages;
    for i in 0..total_setup_pages {
        used_pages_bitmap[i/8] |= 1 << (i % 8); // I've got no clue if this works, but it's the idea
    }
     */

    // FIXME LATER set any flags for phys_book here


    let virt_book : VirtBook = VirtBook::new();
    // TODO what do I need from VIRTBOOK
    //  -> Is it a remapping structure plus then pass as many of these to the arch as needed?
    //  -> Is it just the remapping struct and one for the arch to hold it's entire virtual setup?
    //  -> Is it just one of those?
    //  Explore how the architecture would use this before deciding that
    //    -> Write some of the alloc and free code first
    //    -> This also influences the global hv struct and how it holds the virtual books
    //        -> Is it just one or is it a collection of them?

    // TODO THIS IS WHAT TO TALK ABOUT ***********************************************************
    //  Additionally, what about, instead of a bitmap, a btreemap which maps a page index to the
    //      number of free pages including and following it
    //  Also, how do I convert an address to a Rust friendly style and when should I do that in the process
    //  -> see below

    hv_paging.arch_paging_init(virt_book)

    // create the pages for the setup
    // TODO create a simple setup for allocating and freeing pages


    // split each of these structs into different modules
    // like phys_book for instance
    // then somewhere else would have global variables for those things
    // top level is a static global hypervisor object that holds all of this -- global atomic reference
    //      -> init function that gives you that global a ref
    //      -> read and write locks on it
    //      -> Arc<Mutex>

}
// FIXME add these TODOs to my notes for record keeping.

// TODO what will the general running of the hv be post-setup?
//      -> this I can worry about much farther down the line
//      -> generally run like a server that gets sent signals to set up pages




// TODO use btreemap with box for both alloc and free pages
//  indices will be nearest to other pages nearest to it in space
//  keyed by address -- usize
//  will make things easier in the long run as well
//  -> random important thought -- make combinations of pages called a chapter


pub fn alloc(size: usize, hierarchical: bool) -> Result<Address, u8> { // FIXME to look/work better with hierarchical changes
    let mut hypervisor_struct = HYPERVISOR.lock().unwrap();
    let paging_structs = &mut hypervisor_struct.rust_hypervisor_paging;
    let phys_book = &mut paging_structs.phys_book;
    let paging = &mut paging_structs.paging_structure;
    let page_size = PAGE_SIZE.load(Ordering::SeqCst);

    let num_pages = num_pages(size, page_size);

    let phys_addr;
    if hierarchical {
        phys_addr = find_chapters(num_pages, phys_book)?;
    } else {
        phys_addr = find_chapter(num_pages, phys_book)?;
    }
    place_chapter(&phys_addr, num_pages, phys_book);

    paging.create_mapping(phys_addr[0])
}


pub fn free(phys_addr: usize) -> Result<(), u8> {
    let mut hypervisor_struct = HYPERVISOR.lock().unwrap();
    let paging_structs = &mut hypervisor_struct.rust_hypervisor_paging;
    let mut phys_book = &mut paging_structs.phys_book;

    let num_pages_opt = phys_book.used_pages.get(&phys_addr);
    if num_pages_opt.is_none() {
        return Err(1);
    }

    let num_pages: usize = *num_pages_opt.unwrap();
    phys_book.used_pages.remove(&phys_addr);

    coalesce(phys_addr, num_pages, &mut phys_book);


    Ok(())

    // TODO use unsafe to change the reference and I'll know that's safe (note from meeting)
    //  -> reference to start and number of pages with a global constant page size
}


fn coalesce(phys_addr: usize, num_pages: usize, phys_book: &mut PhysBook) {

    let is_prev_free: bool;
    let mut is_next_free: bool = true;
    let free_pages = &mut phys_book.free_pages;
    let page_size = PAGE_SIZE.load(Ordering::SeqCst);

    let mut new_num_pages: usize = num_pages;

    let next_page = &(phys_addr + (num_pages*page_size));
    let next_page_size_opt = &mut free_pages.get(next_page);
    if next_page_size_opt.is_none() {
        is_next_free = false;
    }
    let next_page_size = *next_page_size_opt.unwrap();

    let mut curr_addr: usize = phys_addr;
    loop {
        curr_addr = curr_addr - page_size;
        if free_pages.get(&curr_addr).is_some() {
            is_prev_free = true;
            break;
        }
        if phys_book.used_pages.get(&curr_addr).is_some() {
            is_prev_free = false;
            break;
        }
    }

    if is_prev_free {
        new_num_pages += free_pages.get(&curr_addr).unwrap();
        free_pages.remove(&curr_addr);
    } else {
        curr_addr = phys_addr;
    }
    if is_next_free {
        new_num_pages += next_page_size;
        free_pages.remove(next_page);
    }

    free_pages.insert(curr_addr, new_num_pages);
}


fn place_chapter(phys_addr: &Vec<Address>, num_pages: NumPages, phys_book: &mut PhysBook) {
    // FIXME currently this only works with 1 page chapters, so when it's not hierarchical

    // guaranteed because of find_chapter
    let num_pages_avail= *phys_book.free_pages.get(&phys_addr[0]).unwrap();
    let free_pages = &mut phys_book.free_pages;
    let used_pages = &mut phys_book.used_pages;
    let page_size = PAGE_SIZE.load(Ordering::SeqCst);

    free_pages.remove(&phys_addr[0]);
    if num_pages_avail > num_pages {
        let new_addr = phys_addr[0] + num_pages*page_size;
        free_pages.insert(new_addr, num_pages_avail - num_pages);
    }

    used_pages.insert(phys_addr[0], num_pages);
}

/// simple first_fit search for now
fn find_chapter(num_pages: NumPages, phys_book: &mut PhysBook) -> Result<Vec<Address>, u8> {
    let free_pages = &phys_book.free_pages;

    for (addr, size) in free_pages.iter() {
        if *size >= num_pages {
            let mut page = Vec::new();
            page.push(*addr);
            return Ok(page);
        }
    }

    Err(1)
}

/// find_chapter but it is hierarchical, meaning it can combine multiple chapters
fn find_chapters(num_pages: NumPages, phys_book: &mut PhysBook) -> Result<Vec<Address>, u8> {
    // TODO

    let free_pages = &phys_book.free_pages;
    let mut chapter = Vec::new();
    let mut chapter_size: usize = 0;

    for (addr, size) in free_pages.iter() {

            chapter.push(*addr);
            chapter_size += size;

            if chapter_size >= num_pages {
                break;
            }
    }

    if chapter_size >= num_pages {
        Ok(chapter)
    } else {
        Err(1)
    }
}



// TODO todo list
//  -> finish out the simple alloc/free implementation (rough done)
//      -> figure out paging_create from jailhouse and if I need a version of that or if that can be in arch specific
//  -> make a working user-space arch implementation (rough done)
//  -> create a simple server setup that just works out of the command line
//  -> clean up code to make it easy to read for others

// ARM Cortex-R/M depending on what Lydia is doing (choose that one)


// SLIDES
// What questions are you asking?
// -> what did you have to learn to make that happen
// What I did--> development and contributions







