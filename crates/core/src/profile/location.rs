use crate::profile::buffer::{decode_field, Buffer};
use crate::profile::mapping::Mapping;
use crate::profile::{function, line, Decoder};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
// Describes function and line table debug information.
pub struct Location {
    // Unique nonzero id for the location.  A profile could use
    // instruction addresses or any integer sequence as ids.
    pub id: u64,
    // The id of the corresponding profile.Mapping for this location.
    // It can be unset if the mapping is unknown or not applicable for
    // this profile type.
    pub mapping_index: u64,
    // The instruction address for this location, if available.  It
    // should be within [Mapping.memory_start...Mapping.memory_limit]
    // for the corresponding mapping. A non-leaf address may be in the
    // middle of a call instruction. It is up to display tools to find
    // the beginning of the instruction if necessary.
    pub address: u64,
    // Multiple line indicates this location has inlined functions,
    // where the last entry represents the caller into which the
    // preceding entries were inlined.
    //
    // E.g., if memcpy() is inlined into printf:
    //    line[0].function_name == "memcpy"
    //    line[1].function_name == "printf"
    pub line: Vec<line::Line>,
    // Provides an indication that multiple symbols map to this location's
    // address, for example due to identical code folding by the linker. In that
    // case the line information above represents one of the multiple
    // symbols. This field must be recomputed when the symbolization state of the
    // profile changes.
    pub is_folder: bool,

    //HELPER
    pub mapping: Option<Mapping>,
}

impl Decoder<Location> for Location {
    fn decode(buf: &mut Buffer, data: &mut Vec<u8>) -> Location {
        let mut loc = Location::default();
        while !data.is_empty() {
            match decode_field(buf, data) {
                Ok(ref mut buf_data) => {
                    match buf.field {
                        // optional uint64 function_id = 1
                        1 => {
                            loc.id = buf.u64;
                        }
                        // optional int64 line = 2
                        2 => {
                            loc.mapping_index = buf.u64;
                        }
                        // optional uint64 address = 3;
                        3 => {
                            loc.address = buf.u64;
                        }
                        // repeated Line line = 4
                        4 => {
                            // todo!(why buf copied twice) ?????
                            loc.line.push(line::Line::decode(buf, buf_data));
                        }
                        5 => {
                            if buf.u64 == 0 {
                                loc.is_folder = false
                            } else {
                                loc.is_folder = true
                            }
                        }
                        _ => {
                            panic!("Unknown location type");
                        }
                    }
                }
                Err(err) => {
                    panic!(err);
                }
            }
        }
        loc
    }
}

impl ToString for Location {
    #[inline]
    fn to_string(&self) -> String {
        let mut ss: Vec<String> = vec![];
        let mut loc_str = format!("{:6}: {:#x} ", self.id, self.address);

        match self.mapping {
            None => {}
            Some(ref mapping) => {
                loc_str.push_str(format!("M={} ", mapping.id).as_ref());
            }
        }

        if self.is_folder {
            loc_str.push_str("[F] ");
        }
        if self.line.is_empty() {
            // clone string w/o getting ownership, because we also use push in the for cycle below
            ss.push(loc_str.clone());
        }

        for (li, _) in self.line.iter().enumerate() {
            let mut ln_str = String::from("??");

            let func = self.line[li].function.clone();
            // TODO better to use option
            if func != function::Function::default() {
                ln_str.clear();
                ln_str.push_str(&format!(
                    "{} {}:{} s={}",
                    func.name, func.filename, self.line[li].line, func.start_line
                ));

                if func.name != func.system_name {
                    ln_str.push_str(&format!("({})", func.system_name));
                }
            }
            // HERE ^^
            ss.push(format!("{}{}", loc_str, ln_str));
            loc_str.clear();
            loc_str.push_str("             ");
        }

        ss.join("\n")
    }
}
