use crate::profile::buffer::{decode_field, Buffer};
use crate::profile::Decoder;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
/// ValueType describes the semantics and measurement units of a value
pub struct ValueType {
    // Type and uint do not present in proto file
    // Used only for parsing
    // cpu, wall, inuse_space, etc
    pub r#type: String,
    // seconds, nanoseconds, bytes, etc
    pub unit: String,

    // index in the string table
    pub type_index: i64,
    // index in the string table
    pub unit_index: i64,
}

impl Decoder<ValueType> for ValueType {
    #[inline]
    fn decode(buf: &mut Buffer, data: &mut Vec<u8>) -> ValueType {
        let mut vt = ValueType::default();
        while !data.is_empty() {
            match decode_field(buf, data) {
                Ok(_) => {
                    match buf.field {
                        //1
                        1 => {
                            vt.type_index = buf.u64 as i64;
                        }
                        //2
                        2 => {
                            vt.unit_index = buf.u64 as i64;
                        }
                        _ => {
                            panic!("Unknown value_type type");
                        }
                    }
                }
                Err(err) => {
                    panic!(err);
                }
            }
        }
        vt
    }
}