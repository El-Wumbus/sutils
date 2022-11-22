use core::read_chunk;
use std::{path::PathBuf, env::args};

use structopt::StructOpt;
static mut LAST_BYTE_LOCATION: u64 = 0;

pub struct Configuration
{
    pub file: PathBuf,
    pub program_name: String,
    pub force_pager: bool,

}

impl Configuration
{
    pub fn get() -> Configuration
    {
        let option = Opt::from_args();

    let args: Vec<String> = args().collect();
    let progname = format!("{}",PathBuf::from(PathBuf::from(args[0].clone()).file_name().unwrap()).display());
    
        Configuration {
            file: option.file,
            program_name: progname,
            force_pager: option.pager,
        }
    }
}

pub fn readline(file: PathBuf, bytes: u64) -> (Vec<u8>, bool) {
    let (chunk, read) = read_chunk(file, bytes, unsafe {LAST_BYTE_LOCATION});
    unsafe { LAST_BYTE_LOCATION += bytes };
    let mut eof = false;
    if read < bytes as usize {
        eof = true
    }
    (chunk, eof)
}

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt
{
    /// The file to read.
    #[structopt(parse(from_str))]
    file: PathBuf,

    // Force the use of a pager.
    #[structopt(long, short = "P")]
    pager: bool,
}

