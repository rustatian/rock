use crate::profile::buffer::{decode_field, Buffer};
use crate::profile::Decoder;
use std::default::Default;

// TMP
// mapping corresponds to Profile.Mapping
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Mapping {
    // Unique nonzero id for the mapping.
    pub id: u64,
    // Address at which the binary (or DLL) is loaded into memory.
    pub memory_start: u64,
    // The limit of the address range occupied by this mapping.
    pub memory_limit: u64,
    // Offset in the binary that corresponds to the first mapped address.
    pub memory_offset: u64,
    // Index into string table
    // The object this entry is loaded from.  This can be a filename on
    // disk for the main binary and shared libraries, or virtual
    // abstractions like "[vdso]".
    pub filename: String,
    // Index into string table
    // A string that uniquely identifies a particular program version
    // with high probability. E.g., for binaries generated by GNU tools,
    // it could be the contents of the .note.gnu.build-id field.
    pub build_id: String,

    pub has_function: bool,
    pub has_filenames: bool,
    pub has_line_numbers: bool,
    pub has_inline_frames: bool,

    // Index into string table
    pub filename_index: i64,
    // Index into string table
    pub build_id_index: i64,
}

impl Decoder<Mapping> for Mapping {
    fn decode(buf: &mut Buffer, data: &mut Vec<u8>) -> Mapping {
        let mut mapping = Mapping::default();
        while !data.is_empty() {
            match decode_field(buf, data) {
                Ok(_) => {
                    match buf.field {
                        //1
                        1 => {
                            mapping.id = buf.u64;
                        }
                        //2
                        2 => {
                            mapping.memory_start = buf.u64;
                        }
                        //3
                        3 => {
                            mapping.memory_limit = buf.u64;
                        }
                        //4
                        4 => {
                            mapping.memory_offset = buf.u64;
                        }
                        //5
                        5 => {
                            mapping.filename_index = buf.u64 as i64;
                        }
                        //6
                        6 => {
                            mapping.build_id_index = buf.u64 as i64;
                        }
                        //7
                        7 => {
                            if buf.u64 == 0 {
                                mapping.has_function = false;
                            } else {
                                mapping.has_function = true;
                            }
                        }
                        //8
                        8 => match buf.u64 {
                            0 => mapping.has_filenames = false,
                            _ => mapping.has_filenames = true,
                        },
                        //9
                        9 => {
                            if buf.u64 == 0 {
                                mapping.has_line_numbers = false;
                            } else {
                                mapping.has_line_numbers = true;
                            }
                        }
                        //10
                        10 => {
                            if buf.u64 == 0 {
                                mapping.has_inline_frames = false;
                            } else {
                                mapping.has_inline_frames = true;
                            }
                        }
                        _ => {
                            panic!("Unknown mapping type");
                        }
                    }
                }
                Err(err) => {
                    panic!("{}", err);
                }
            }
        }
        mapping
    }
}

impl ToString for Mapping {
    fn to_string(&self) -> String {
        let mut bits = String::new();

        if self.has_function {
            bits.push_str("[FN]");
        }

        if self.has_filenames {
            bits.push_str("[FL]");
        }

        if self.has_line_numbers {
            bits.push_str("[LN]");
        }

        if self.has_inline_frames {
            bits.push_str("[IN]");
        }

        format!(
            "{}: {:#x}/{:#x}/{:#x} {} {} {}",
            self.id,
            self.memory_start,
            self.memory_limit,
            self.memory_offset,
            self.filename,
            self.build_id,
            bits
        )
    }
}
