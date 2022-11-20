use std::{path::PathBuf, env::args};

use structopt::StructOpt;
static mut LAST_BYTE_LOCATION: u64 = 0;

pub fn readline(file: PathBuf, bytes: u64) -> (Vec<u8>, bool) {
    use std::{
        fs::File,
        io::{Read, Seek, SeekFrom},
    };
    let mut byte_array = Vec::new();
    let mut input = File::open(file).unwrap();

    // Seek to the start position
    input
        .seek(SeekFrom::Start(unsafe { LAST_BYTE_LOCATION } + 1))
        .unwrap();

    unsafe { LAST_BYTE_LOCATION += bytes }
    // Create a reader with a fixed length
    let mut chunk = input.take(bytes);

    let read = chunk.read_to_end(&mut byte_array).unwrap();
    let mut eof = false;
    if read < bytes as usize {
        eof = true
    }
    (byte_array, eof)
}

#[derive(StructOpt, Debug)]
#[structopt(name = "plsplay")]
struct Opt
{
    /// The audio file to play.
    #[structopt(parse(from_str))]
    file: PathBuf,

    // Force the use of a pager.
    #[structopt(long, short = "P")]
    pager: bool,
}

pub fn get_args() -> (PathBuf, PathBuf, bool)
{
    let option = Opt::from_args();

    let args: Vec<String> = args().collect();
    let progname = PathBuf::from(PathBuf::from(args[0].clone()).file_name().unwrap());
    (option.file, progname , option.pager)
}