/// TODO file comment

mod rust_hypervisor;
mod user_space_arch;
mod exec;

use std::io;
use std::io::BufRead;
use std::sync::atomic::Ordering;
use std::time::Instant;
use crate::rust_hypervisor::paging::{alloc, free};
use crate::rust_hypervisor::paging::declarations::{num_pages, FREE_TIME, NOT_ENOUGH_ROOM};
use crate::rust_hypervisor::setup::{hv_init, PAGE_SIZE};
use crate::rust_hypervisor::paging::declarations::*;



fn main() -> Result<(), u8> {

    hv_init(1)?;
    run_server()
}


/// TODO have two servers running so I can see how those interleave
///     I need it to be able to use multiple sections of pages
///     I can then show statistics of how it was faster to find multiple chapters to combine into one
///     I am thinking that I just fill it with 10 pages then 5 pages a bunch of times
///     -> then free the first two 5 page chapters to make a 10 page combined chapter
///
///
///
///
/// TODO WRITE DOWN MY TESTING
///
/// TODO set up benchmarks to check the scalability including the difference b/w hierarchical and not
///  -> use very fragmented requests
///  -> figure out how that affects each of the different operations (alloc, free, coalesce, etc)
///  -> it's not perfect at all, but can be useful in some ways (that is the important part)
///
/// specific focus on ARM archs with cortex-m vs r with linear mapping, etc. TODO figure this out

fn run_server() -> Result<(), u8> {
    let stdin = io::stdin();
    let mut hierarchical = false; // boolean to use hierarchical allocation
    let mut verbose = false; // boolean to print out more information
    let mut benchmark = false; // boolean to print out benchmark timing

    loop {
        let mut two_inputs = false;
        let mut three_inputs = false;
        // read from terminal
        let mut input = String::new();
        stdin.lock().read_line(&mut input).expect("Error: could not read line.");

        let input_words : Vec<&str> = input.split_whitespace().collect();

        if input_words.is_empty() {
            continue;
        }
        if input_words.len() >= 2 {
            two_inputs = true;
            three_inputs = if input_words.len() >= 3 { true } else { false };
        }

        let first_input = input_words[0]; // TODO add index statics

        if first_input == "quit" {
            break;
        }

        else if first_input == "verbose" {
            println!("Verbose mode turned on.");
            verbose = true;
        }

        else if first_input == "hierarchical" {
            println!("Hierarchical mode turned on.");
            hierarchical = true;
        }

        else if first_input == "benchmark" {
            println!("Benchmarking mode turned on.");
            benchmark = true;
        }

        else if first_input == "print" {
            println!("printing something."); // TODO just make a full usage function for this

            if two_inputs {
                let second_input = input_words[1]; // TODO add index statics

                if second_input == "help" {
                    println!("printing helpful information.") // TODO expand all of this
                }
                else if second_input == "pages" {
                    if three_inputs && input_words[2] == "free" {
                        println!("printing free pages.");
                    }
                    else if three_inputs && input_words[2] == "used" {
                        println!("printing used pages.");
                    } else {
                        println!("printing free and used pages.");
                    }
                }
            }

        }
        // If the command line asks for the alloc operation
        else if first_input == "alloc" && two_inputs {
            let start = Instant::now();
            let size = input_words[1].parse::<usize>().unwrap();

            match alloc(size, hierarchical) {
                Ok(addr) => {
                    if verbose {
                        println!("allocated {} pages for a size of {} at address {}.",
                                 num_pages(size, PAGE_SIZE.load(Ordering::SeqCst)), size, addr);
                    }
                },
                Err(e) => {
                    if verbose {
                        println!("failed to allocate pages.");
                    }
                    if e == NOT_ENOUGH_ROOM {
                        println!("not enough room to allocate this amount.");
                        continue;
                    }
                    return Err(e);
                }
            };

            if benchmark {
                let op_time = Instant::now().duration_since(start);
                println!("Alloc ran in {} seconds and {} microseconds",
                         op_time.as_secs(), op_time.subsec_micros());
            }

        }
        // If the command line asks for the free operation
        else if first_input == "free" && two_inputs {
            let start = Instant::now();
            let addr = input_words[1].parse::<usize>().unwrap();

            match free(addr) {
                Ok(()) => {
                    if verbose {
                        println!("freed pages for address {}.", addr);
                    }
                },
                Err(e) => {
                    if verbose {
                        println!("failed to free pages.");
                    }
                    return Err(e);
                }
            };

            if benchmark {
                let op_time = Instant::now().duration_since(start);
                println!("Alloc ran in {} seconds and {} microseconds", op_time.as_secs(), op_time.subsec_micros());
            }

        }
        // Error case for alloc and free
        else if (first_input == "alloc" || first_input == "free") && !two_inputs {
            if verbose {
                println!("Input Error: Must provide another input for alloc or free.");
            }
        }
        // Error default case
        else {
            if verbose {
                println!("Invalid input, try again.");
            }
        }
    }

    Ok(())
}