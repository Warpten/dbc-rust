use std::collections::HashMap;
use std::io::BufRead;
use std::str::from_utf8;

use crate::r#trait::FromBuffer;
use crate::error::Error;

pub type StringTable = HashMap<u32, String>;

impl FromBuffer for StringTable {
    fn read_from(source : &mut dyn BufRead, block_size : u32) -> Result<Self, Error> where Self : Sized {
        let mut string_table = HashMap::<u32, String>::new();
        let mut string_table_index = 0;
        loop {
            let mut buffer : Vec<u8> = vec![];
            match source.read_until(0x00u8, &mut buffer) {
                Ok(read_count) => {
                    if read_count <= 1 || block_size == string_table_index {
                        break;
                    }
                    
                    // This needs to happen because read_until also stops if it attempts to go past the end of the source.
                    if buffer[read_count - 1] != b'\0' {
                        return Err(Error::Malformed);
                    }

                    let slice = &buffer[0..read_count - 1];

                    match from_utf8(&slice) {
                        Ok(value) => string_table.insert(string_table_index, value.to_string()),
                        Err(error) => return Err(Error::from(error))
                    };

                    string_table_index += read_count as u32;
                },
                _ => return Err(Error::Malformed)
            }
        }

        Ok(string_table)
    }
}