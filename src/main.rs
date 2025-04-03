mod rust_hypervisor;
mod user_space_arch;
mod exec;

use std::io;
use std::io::BufRead;
use crate::rust_hypervisor::paging::{alloc, free};
use crate::rust_hypervisor::setup::{hv_init};



fn main() -> Result<(), u8> {

    hv_init(1)?;
    run_server()
}


/// TODO have two servers running so I can see how those interleave
///     I need it to be able to use multiple sections of pages
///     I can then show statistics of how it was faster to find multiple chapters to combine into one
///     I am thinking that I just fill it with 10 pages then 5 pages a bunch of times
///     -> then free the first two 5 page chapters to make a 10 page combined chapter

fn run_server() -> Result<(), u8> {
    let stdin = io::stdin();
    let mut hierarchical = false;
    let mut verbose = false;

    loop {
        let mut enough_inputs = false;
        // read from terminal
        let mut input = String::new();
        stdin.lock().read_line(&mut input).expect("Error: could not read line.");

        let input_words : Vec<&str> = input.split_whitespace().collect();

        if input_words.is_empty() {
            continue;
        }
        if input_words.len() >= 2 {
            enough_inputs = true;
        }

        let first_input = input_words[0];

        if first_input == "quit" {
            break;
        }
        else if first_input == "verbose" {
            println!("Verbose mode turned on.");
            verbose = true;
        }
        else if first_input == "combine_chapters" {
            println!("Hierarchical mode turned on.");
            hierarchical = true;
        }
        else if first_input == "print" {
            println!("printing something.");
            // this should be expanded to print out various info
        }
        else if first_input == "alloc" && enough_inputs {
            let size = input_words[1].parse::<usize>().unwrap();
            match alloc(size, hierarchical) {
                Ok(addr) => {
                    if verbose {
                        println!("allocated pages for a size of {} at address {}.", size, addr);
                    }
                },
                Err(e) => {
                    if verbose {
                        println!("failed to allocate pages.");
                    }
                    return Err(e);
                }
            };
        }
        else if first_input == "free" && enough_inputs {
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
        } else if (first_input == "alloc" || first_input == "free") && !enough_inputs {
            if verbose {
                println!("Input Error: Must provide another input for alloc or free.");
            }
        } else {
            if verbose {
                println!("Invalid input, try again.");
            }
        }
    }

    Ok(())
}