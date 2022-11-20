use core::*;

use minus::{dynamic_paging, MinusError, Pager};
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
    let (file, progname, page_settings) = get_args();

    // Print instead of using the pager when space is applicable
    pager.set_run_no_overflow(!page_settings)?;

    // Set the text for the bottom bar
    pager.set_prompt(format!(
        "{} - {}",
        progname.display(),
        PathBuf::from(file.file_name().unwrap()).display()
    ))?;
    
    loop {
        let (line, eof) = readline(file.clone(), 48);
        let mut i: u64 = 1;

        for item in line {
            write!(pager, "{:02X} ", item)?;
            if i % 4 == 0 {
                write!(pager, " ")?;
            }
            i += 1;
        }

        writeln!(pager)?;

        if eof == true {
            break;
        }
    }
    pager_thread.join().unwrap()?;
    Ok(())
}