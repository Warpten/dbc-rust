use std::io::BufRead;
use super::error::Error;

pub trait FromBuffer {
    fn read_from(source : &mut dyn BufRead, block_size : u32) -> Result<Self, Error> where Self : Sized;
}