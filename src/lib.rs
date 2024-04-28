use std::{collections::HashMap, io::BufRead, marker::PhantomData};

use error::Error;
use io::Readable;
use spec::{wdbc, Spec};

mod spec;
mod block;
mod io;
pub mod error;
pub mod r#trait;

/**
 * A [`Serializable`] is a DBC record type that allows serializing to a sequence of bytes.
 * This is usually implemented to allow write support for DBC files.
 */
pub trait Serializable {

}

/**
 * A [`Deserializable`] is a DBC record type that allows deserialization from a sequence of bytes.
 * This is usually implemented to allow read support for DBC files.
 */
pub trait Deserializable where Self : Sized {
    fn from<Source : Spec>(index : u32, spec : &Source) -> Result<Self, Error>;
}

/**
 * An [`Identifiable`] is a type that has an unique associated identifier.
 */
pub trait Identifiable {
    type KeyType;

    /**
     * Returns this object's associated unique identifier.
     */
    fn id(&self) -> Self::KeyType;
}

pub struct Deserializer<T : Deserializable>(PhantomData<T>);

impl<T : Deserializable + Identifiable> Deserializer<T> {
    pub fn new(source : &mut dyn BufRead) -> Result<HashMap<T::KeyType, T>, Error> {
        match u32::read(source, &u32::from_le_bytes) {
            Ok(1464091203) => wdbc::Spec::try_from(source)?.exec(),
            _ => Err(Error::Magic),
        }
    }
}

pub struct Serializer<T : Serializable>(PhantomData<T>);

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use crate::io::Readable;

    struct AchievementEntry {
        id : u32,
        arr: [u32; 4],
    }

    // Scaffolding for codegen
    impl super::Deserializable for AchievementEntry {
        fn from<Source : super::Spec>(index : u32, spec : &Source) -> Result<Self, crate::error::Error> {
            let data = spec.record(index);
            assert_eq!(spec.field_count(), 1); // NOTE: 1 is generated here
            // This assert becomes hell with arrays; need to retrieve array size

            // This whole block would also be generated with 1 changing and the read calls as well.
            let id = {
                let mut readable = BufReader::new(spec.field_data(data, 1));
                u32::read(&mut readable, &u32::from_le_bytes)?
            };

            let arr = {
                let mut readable = BufReader::new(spec.field_data(data, 2));
                let mut data : [u32; 4] = [0; 4];
                for i in 0..4 {
                    data[i] = u32::read(&mut readable, &u32::from_le_bytes)?
                }
                data
            };

            // And then generate this
            Ok(AchievementEntry { id : id, arr : arr })
        }
    }

    #[test]
    fn test() {

    }
}