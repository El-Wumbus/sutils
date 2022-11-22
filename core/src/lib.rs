use std::{path::PathBuf, io::{SeekFrom, Seek, Read, Write}, fs::File};

use terminal_size::{terminal_size};
pub fn get_col() -> u64
{
    let (width, _height) = terminal_size().unwrap();
    width.0 as u64
}

pub fn read_chunk(file:PathBuf,len:u64, offset: u64) -> (Vec<u8>, usize)
{
    let mut chunk = Vec::new();
    let mut input = File::open(file).unwrap();
    input.seek(SeekFrom::Start(offset))
        .unwrap();

    let read = input.take(len).read_to_end(&mut chunk).unwrap();
    (chunk, read)
}

pub fn write_chunk(file:PathBuf,chunk:Vec<u8>, offset: u64) -> Result<usize, String>
{
    let mut input = File::open(file).unwrap();
    input.seek(SeekFrom::Start(offset)).unwrap();

    match input.write(&chunk)
    {
        Ok(x) => Ok(x),
        Err(x) =>  Err(format!("Error: {}", x)),
    }
}