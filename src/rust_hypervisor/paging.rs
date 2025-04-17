/// TODO file comment

pub mod declarations;
pub mod phys_book;
pub mod virt_book;
pub mod arch_paging;
pub mod arch_paging_access;

use std::sync::atomic::Ordering;
use std::time::Instant;
use crate::rust_hypervisor;
use arch_paging_access::*;
use declarations::*;
use phys_book::*;
use virt_book::*;
use rust_hypervisor::setup::HYPERVISOR;
use crate::rust_hypervisor::setup::PAGE_SIZE;


/// initialize the hypervisor paging system
/// to enable arch specific paging setup to start
pub fn paging_init() -> Result<(), u8> {

    let mut hypervisor_struct = HYPERVISOR.lock().unwrap();

    // get references to the relevant structs
    let paging_structs = &mut hypervisor_struct.rust_hypervisor_paging;
    let mut phys_book = &mut paging_structs.phys_book;
    let mut arch_paging  = &mut paging_structs.arch_paging_struct;


    // set up the variables inside of phys_book and hv_paging

    let page_size : usize = PAGE_SIZE.load(Ordering::SeqCst);
    arch_paging.page_size = page_size;

    let mem_size : usize = arch_paging.get_hv_mem_size();
    let num_total_pages : NumPages = mem_size / page_size;
    phys_book.num_total_pages = num_total_pages;

    // This could be changed later to just be an offset starting at 0
    // where the arch code knows the starting address and applies the offset to that
    let start_addr = arch_paging.get_hv_mem_start();
    phys_book.mem_start = start_addr;
    phys_book.free_pages.insert(start_addr, num_total_pages); // insert the whole thing into the free pages map

    // This is where any relevant flags for phys_book can be set
    // –> there are no flags implemented at this time
    phys_book.flags = 0;

    // See virt_book.rs for more on VirtBook
    let virt_book : VirtBook = VirtBook::new();

    // pass virt_book to the architecture in case that becomes useful later
    arch_paging.arch_paging_init(virt_book)
}


/// TODO
pub fn alloc(size: usize, hierarchical: bool) -> Result<Address, u8> { // FIXME to look/work better with hierarchical changes
    let now = Instant::now();

    let mut hypervisor_struct = HYPERVISOR.lock().unwrap();
    let paging_structs = &mut hypervisor_struct.rust_hypervisor_paging;
    let phys_book = &mut paging_structs.phys_book;
    let arch_paging = &mut paging_structs.arch_paging_struct;
    let page_size = PAGE_SIZE.load(Ordering::SeqCst);

    let num_pages = num_pages(size, page_size);

    let phys_addr;
    if hierarchical {
        phys_addr = find_chapters(num_pages, phys_book)?;
    } else {
        phys_addr = find_chapter(num_pages, phys_book)?;
    }
    place_chapter(&phys_addr, num_pages, phys_book);

    let result = arch_paging.create_mapping(phys_addr[0]);

    let mut alloc_time = ALLOC_TIME.lock().unwrap();
    *alloc_time = Instant::now().duration_since(now);

    result
}


/// TODO
pub fn free(phys_addr: usize) -> Result<(), u8> {
    let now = Instant::now();

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

    let mut free_time = FREE_TIME.lock().unwrap();
    *free_time = Instant::now().duration_since(now);
    Ok(())
}


/// TODO
fn coalesce(phys_addr: usize, num_pages: usize, phys_book: &mut PhysBook) {

    let is_prev_free: bool;
    let mut is_next_free: bool = true;
    let free_pages = &mut phys_book.free_pages;
    let page_size = PAGE_SIZE.load(Ordering::SeqCst);

    let mut new_num_pages: usize = num_pages;

    let mut next_page_size = 0;
    let next_page_addr = phys_addr + (num_pages*page_size);
    let next_page_size_opt = &mut free_pages.get(&next_page_addr);
    if next_page_size_opt.is_none() {
        is_next_free = false;
    } else {
        next_page_size = *next_page_size_opt.unwrap();
    }

    let mut curr_addr: usize = phys_addr;
    loop {
        if curr_addr < page_size {
            is_prev_free = false;
            break;
        }

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
        free_pages.remove(&next_page_addr);
    }

    free_pages.insert(curr_addr, new_num_pages);
}


/// TODO
fn place_chapter(chapters: &Vec<Address>, num_pages: NumPages, phys_book: &mut PhysBook) {
    // FIXME currently this only works with 1 page chapters, so when it's not hierarchical
    //  -> should be good now, but requires testing

    let free_pages = &mut phys_book.free_pages;
    let used_pages = &mut phys_book.used_pages;
    let page_size = PAGE_SIZE.load(Ordering::SeqCst);

    let num_chapters = chapters.len();
    for i in 0..num_chapters {
        // guaranteed because of find_chapter
        let num_pages_avail = *free_pages.get(&chapters[i]).unwrap();

        free_pages.remove(&chapters[i]);
        if num_pages_avail > num_pages {
            let new_addr = chapters[i] + num_pages * page_size;
            free_pages.insert(new_addr, num_pages_avail - num_pages);
        }

        used_pages.insert(chapters[i], num_pages);
    }
}

/// simple first_fit search for now
/// TODO
fn find_chapter(num_pages: NumPages, phys_book: &mut PhysBook) -> Result<Vec<Address>, u8> {
    let free_pages = &phys_book.free_pages;

    for (addr, pages) in free_pages.iter() {
        if *pages >= num_pages {
            let mut page = Vec::new();
            page.push(*addr);
            return Ok(page);
        }
    }

    Err(NOT_ENOUGH_ROOM)
}

/// find_chapter but it is hierarchical, meaning it can combine multiple chapters
/// TODO
fn find_chapters(num_pages: NumPages, phys_book: &mut PhysBook) -> Result<Vec<Address>, u8> {
    // TODO idk why this has a todo

    let free_pages = &phys_book.free_pages;
    let mut chapter = Vec::new();
    let mut chapter_size: usize = 0;

    for (addr, pages) in free_pages.iter() {

            chapter.push(*addr);
            chapter_size += pages;

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







