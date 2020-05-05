use crate::profile::buffer::{Buffer, decode_field};
use crate::profile::{function, Decoder};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Line {
    // Line number in source code.
    pub line: i64,
    // The id of the corresponding profile.Function for this line.
    pub function_index: u64,

    // HELPERS
    pub function: function::Function,
}

impl Decoder<Line> for Line {
    #[inline]
    fn decode(buf: &mut Buffer, data: &mut Vec<u8>) -> Line {
        let mut line = Line::default();
        while !data.is_empty() {
            match decode_field(buf, data) {
                Ok(()) => {
                    match buf.field {
                        // optional uint64 function_id = 1
                        1 => {
                            line.function_index = buf.u64;
                        }
                        // optional int64 line = 2
                        2 => {
                            line.line = buf.u64 as i64;
                        }
                        _ => {
                            panic!("Unknown line type");
                        }
                    }
                }
                Err(err) => {
                    panic!(err);
                }
            }
        }
        line
    }
}
