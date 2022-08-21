use crate::profile::buffer::{decode_field, Buffer};
use crate::profile::Decoder;
use std::default::Default;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Function {
    // Unique nonzero id for the function.
    pub id: u64,
    // Name of the function, in human-readable form if available.
    pub name: String,
    // Name of the function, as identified by the system.
    // For instance, it can be a C++ mangled name.
    pub system_name: String,
    // Source file containing the function.
    pub filename: String,
    // Line number in source file.
    pub start_line: i64,

    // HELPERS
    // Index into string table
    pub name_index: i64,
    // Index into string table
    pub system_name_index: i64,
    // Index into string table
    pub filename_index: i64,
}

impl Decoder<Function> for Function {
    fn decode(buf: &mut Buffer, data: &mut Vec<u8>) -> Function {
        let mut func = Function::default();
        while !data.is_empty() {
            match decode_field(buf, data) {
                Ok(_) => {
                    match buf.field {
                        // optional uint64 id = 1
                        1 => {
                            func.id = buf.u64;
                        }
                        // optional int64 function_name = 2
                        // index to string table
                        2 => {
                            func.name_index = buf.u64 as i64;
                        }
                        // optional int64 function_system_name = 3
                        // index to string table
                        3 => {
                            func.system_name_index = buf.u64 as i64;
                        }
                        // repeated int64 filename = 4
                        // index to string table
                        4 => {
                            func.filename_index = buf.u64 as i64;
                        }
                        // optional int64 start_line = 5
                        5 => {
                            func.start_line = buf.u64 as i64;
                        }
                        _ => {
                            panic!("Unknown function type");
                        }
                    }
                }
                Err(err) => {
                    panic!("{}", err);
                }
            }
        }
        func
    }
}
