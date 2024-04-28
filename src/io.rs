use std::io::BufRead;

use crate::error::Error;

/**
 * Trait that describes a type that is readable from a fixed amount of bytes.
 */
pub trait Readable<const N : usize> {
    fn read<Source : BufRead + ?Sized>(source : &mut Source, f : &dyn Fn([u8; N]) -> Self) -> Result<Self, Error> where Self : Sized {
        let mut buffer : [u8; N] = [0u8; N];
        match source.read(&mut buffer) {
            Ok(size) if size == N => Ok(f(buffer)),
            _ => Err(Error::Malformed)
        }
    }
}

impl Readable<8> for f64 { }
impl Readable<4> for f32 { }

impl Readable<8> for i64 { }
impl Readable<8> for u64 { }

impl Readable<4> for u32 { }
impl Readable<4> for i32 { }

impl Readable<2> for u16 { }
impl Readable<2> for i16 { }