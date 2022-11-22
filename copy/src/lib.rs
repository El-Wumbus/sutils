use core::read_chunk;
use std::path::PathBuf;
use progress_bar::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt
{
    /// The file to read.
    #[structopt(parse(from_str))]
    file: PathBuf,

    /// Overwrite any existing file
    #[structopt(long, short)]
    force: bool,

    /// Show what's happening
    #[structopt(long, short)]
    verbose: bool,
}

fn copy_file(source: PathBuf, dest: PathBuf, verbose:bool)
{
    let mut byte_offset: u64 = 0;
    let file_data = source.metadata().unwrap();
    let size = file_data.len();
    let chunk_size = {
        if size < 10_485_760 // 10MB
        {
            size
        }
        else {
            size / 50
        }
    };
    
        set_progress_bar_action("Reading", Color::Blue, Style::Bold);

        while {let (chunk, read) = read_chunk(source.clone(), chunk_size, byte_offset);  read == 0} {
            inc_progress_bar();
            
        }
}