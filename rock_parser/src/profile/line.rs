use crate::profile::buffer::{decode_field, Buffer};
use crate::profile::{function, Decoder};
use std::cell::RefCell;
use std::rc::Rc;

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
    fn decode(buf: &mut Buffer, data: Rc<RefCell<Vec<u8>>>) -> Line {
        let mut line = Line::default();
        while !data.borrow().is_empty() {
            match decode_field(buf, data.clone()) {
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
