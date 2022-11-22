use core::*;
use minus::{dynamic_paging, MinusError, Pager};
use xdump::{Configuration, readline};
use std::{
    fmt::Write, 
    thread::{spawn}, 
    path::PathBuf
};

fn main() -> Result<(), MinusError> {
    // Initialize the pager
    let mut pager = Pager::new();
    // Run the pager in a separate thread
    let pager2 = pager.clone();
    let pager_thread = spawn(move || dynamic_paging(pager2));
    let conf = Configuration::get();
    let file = conf.file;
    // Print instead of using the pager when space is applicable
    pager.set_run_no_overflow(!conf.force_pager)?;

    // Set the text for the bottom bar
    pager.set_prompt(format!(
        "{} - {}",
        conf.program_name,
        PathBuf::from(file.file_name().unwrap()).display()
    ))?;
    
    // Read from the file in chunks and print as chunks are read.
    loop {
        let (line, eof) = readline(file.clone(), 4 * 1_024 );
        // For grouping bytes in groups of four
        let mut i: u64 = 1;

        for item in line {
            write!(pager, "{:02X} ", item)?;

            // 
            if i % 4 == 0 {
                write!(pager, " ")?;
            }
            i += 1;

            if i >= get_col() / 2
            {
                writeln!(pager)?;
            }
        }

        writeln!(pager)?;

        if eof == true {
            break;
        }
    }
    pager_thread.join().unwrap()?;
    Ok(())
}