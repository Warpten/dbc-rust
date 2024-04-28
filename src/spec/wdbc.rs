use std::collections::HashMap;
use std::io::BufRead;

use crate::block::string_table::StringTable;
use crate::error::Error;
use crate::io::Readable;
use crate::r#trait::FromBuffer;
use crate::{Deserializable, Identifiable};

/* WDBC specification */
pub(crate) struct Spec {
    field_count : u32,
    record_count : u32,
    record_size : usize,
    data : Vec<u8>,
    string_table : StringTable
}

impl TryFrom<&mut dyn BufRead> for Spec {
    fn try_from(source: &mut dyn BufRead) -> Result<Self, Error> {
        let record_count = match u32::read(source, &u32::from_le_bytes) {
            Ok(value) => value,
            _ => return Err(Error::Malformed)
        };
        let field_count = match u32::read(source, &u32::from_le_bytes) {
            Ok(value) => value,
            _ => return Err(Error::Malformed)
        };
        let record_size = match u32::read(source, &u32::from_le_bytes) {
            Ok(value) => value,
            _ => return Err(Error::Malformed)
        };
        
        let string_block_size = match u32::read(source, &u32::from_le_bytes) {
            Ok(value) => value,
            _ => return Err(Error::Malformed)
        };

        let mut record_block = match usize::try_from(record_size * record_count).map(&Vec::<u8>::with_capacity) {
            Ok(container) => container,
            _ => return Err(Error::Memory)
        };
        if source.read(&mut record_block[..]).is_err() {
            return Err(Error::Malformed);
        }

        Ok(Spec {
            field_count,
            record_count,
            record_size  : record_size as usize,
            data         : record_block,
            string_table : StringTable::read_from(source, string_block_size)?
        })
    }
    
    type Error = Error;
}

impl super::Spec for Spec {
    fn records(&self) -> std::slice::Chunks<'_, u8> {
        self.data.chunks(self.record_size)
    }

    fn field_count(&self) -> u32 { self.field_count }

    fn field_data(&self, _record : &[u8], _index : u32) -> &[u8] {
        unimplemented!()
    }
    
    fn exec<T : Deserializable + Identifiable>(&self) -> Result<HashMap<T::KeyType, T>, Error> {
        todo!()
    }
    
    fn record(&self, index : u32) -> &[u8] {
        let record_offset = self.record_size * (index as usize);
        let record_end = record_offset + self.record_size;

        &self.data[record_offset..record_end]
    }
}
