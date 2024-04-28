use std::{collections::HashMap, slice::Chunks};

use crate::{error::Error, Deserializable, Identifiable};


pub mod wdbc;
pub mod field;

trait FieldSpec { }

pub trait Spec {
    /**
     * Returns chunks corresponding to all records stored in the file described by this specification.
     */
    fn records(&self) -> Chunks<'_, u8>;

    fn record(&self, index : u32) -> &[u8];

    /**
     * Returns the amount of fields in a record.
     */
    fn field_count(&self) -> u32;

    /**
     * Returns a chunk of bytes associated with the field of given index in the provided buffer associated with a record.
     */
    fn field_data(&self, record : &[u8], index : u32) -> &[u8];

    fn exec<T : Deserializable + Identifiable>(&self) -> Result<HashMap<T::KeyType, T>, Error>;
}