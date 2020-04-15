use crate::profile::buffer::Buffer;
use crate::profile::buffer::WireTypes::WireVarint;
use crate::profile::Decoder;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Label {
    pub key_index: i64, // Index into string table

    // one of the two following values must be set
    pub str_index: i64,
    // Index into string table
    pub num_index: i64,

    // Should only be present when num is present.
    // Specifies the units of num.
    // Use arbitrary string (for example, "requests") as a custom count unit.
    // If no unit is specified, consumer may apply heuristic to deduce the unit.
    // Consumers may also  interpret units like "bytes" and "kilobytes" as memory
    // units and units like "seconds" and "nanoseconds" as time units,
    // and apply appropriate unit conversions to these.
    pub num_unit_index: i64,
}

impl Decoder<Label> for Label {
    #[inline]
    fn decode(buf: &mut Buffer, data: &mut Vec<u8>) -> Label {
        let mut lb = Label::default();
        while !data.is_empty() {
            match Buffer::decode_field(buf, data) {
                Ok(()) => {
                    match buf.field {
                        //1
                        1 => {
                            lb.key_index = buf.u64 as i64;
                        }
                        //2
                        2 => {
                            lb.str_index = buf.u64 as i64;
                        }
                        //3
                        3 => {
                            lb.num_index = buf.u64 as i64;
                        }
                        //4
                        4 => {
                            lb.num_unit_index = buf.u64 as i64;
                        }
                        _ => {
                            panic!("Unknown label type");
                        }
                    }
                }
                Err(err) => {
                    panic!(err);
                }
            }
        }

        lb
    }
}
