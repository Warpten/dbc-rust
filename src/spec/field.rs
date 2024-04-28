/**
 * A [`ByteSized`] field has a size in bytes.
 */
pub trait ByteSized {
    fn byte_size(&self) -> u32;
}

/**
 * A [`BitSized`] field has a size in bits.
 */
pub trait BitSized {
    fn bit_size(&self) -> u32;
}

/**
 * A [`ByteOffset`] field has a byte offset.
 */
pub trait ByteOffset {
    fn byte_offset(&self) -> u32;
}

/**
 * A [`BitOffset`] field has a bit offset.
 */
pub trait BitOffset {
    fn bit_offset(&self) -> u32;
}

/**
 * An [`Indexed`] field 
 */
pub trait Indexed {

}

/**
 * An [`HasCommonValue`] is a field specification that has a default value. It can (and usually is) bitpacked.
 */
pub trait HasDefaultValue {
    fn default_value(&self) -> u32;
}