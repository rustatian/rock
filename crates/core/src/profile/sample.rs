use crate::profile::buffer::{decode_field, decode_varint, Buffer, WireTypes};
use crate::profile::{label, location, Decoder};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
// Each Sample records values encountered in some program
// context. The program context is typically a stack trace, perhaps
// augmented with auxiliary information like the thread-id, some
// indicator of a higher level request being handled etc.
pub struct Sample {
    // The ids recorded here correspond to a Profile.location.id.
    // The leaf is at location_id[0].
    pub location: Vec<location::Location>,
    // The type and unit of each value is defined by the corresponding
    // entry in Profile.sample_type. All samples must have the same
    // number of values, the same as the length of Profile.sample_type.
    // When aggregating multiple samples into a single sample, the
    // result has a list of values that is the elemntwise sum of the
    // lists of the originals.
    pub value: Vec<i64>,
    // label includes additional context for this sample. It can include
    // things like a thread id, allocation size, etc
    pub label: HashMap<String, Vec<String>>,
    // key is label.key_index(in string table), value is associated str_index
    // entry in Profile.sample_type
    pub num_label: HashMap<String, Vec<i64>>,
    // label and numbers in string table, key_index is a key
    pub num_unit_label: HashMap<String, Vec<String>>, // label and unit measurement, key_index also is a key

    // These types are not present in the proto file
    pub location_index: Vec<u64>,
    pub label_index: Vec<label::Label>,
}

impl Decoder<Sample> for Sample {
    #[inline]
    fn decode(buf: &mut Buffer, data: &mut Vec<u8>) -> Sample {
        let mut s = Sample::default();
        while !data.is_empty() {
            match decode_field(buf, data) {
                Ok(ref mut buf_data) => {
                    match buf.field {
                        //1
                        1 => match buf.r#type {
                            WireTypes::WireBytes => {
                                while !buf_data.is_empty() {
                                    match decode_varint(buf_data) {
                                        Ok(varint) => {
                                            s.location_index.push(varint as u64)
                                        }
                                        Err(err) => {
                                            panic!(err);
                                        }
                                    }
                                }
                            }

                            _ => {
                                if buf.r#type != WireTypes::WireVarint {
                                    panic!("value is not varint type");
                                }

                                s.location_index.push(buf.u64);
                            }
                        },
                        //2
                        2 => match buf.r#type {
                            WireTypes::WireBytes => {
                                while !buf_data.is_empty() {
                                    match decode_varint(buf_data) {
                                        Ok(varint) => s.value.push(varint as i64),
                                        Err(err) => {
                                            panic!(err);
                                        }
                                    }
                                }
                            }
                            _ => {
                                if buf.r#type != WireTypes::WireVarint {
                                    panic!("value is not varint type");
                                }

                                s.value.push(buf.u64 as i64);
                            }
                        },
                        //3
                        3 => {
                            s.label_index.push(label::Label::decode(buf, buf_data));
                        }
                        _ => {
                            panic!("Unknown sample type");
                        }
                    }
                }
                Err(err) => {
                    panic!(err);
                }
            }
        }
        s
    }
}

impl ToString for Sample {
    #[inline]
    fn to_string(&self) -> String {
        let mut ss: Vec<String> = vec![];
        let mut sv = String::new();

        for val in self.value.iter() {
            sv.push_str(format!(" {:10}", val).as_ref());
        }

        sv.push_str(": ");

        for loc in self.location.iter() {
            sv.push_str(format!("{} ", loc.id).as_ref());
        }
        sv.drain((sv.len() - 1)..);

        ss.push(sv);
        let label_header = String::from("                ");

        if !self.label.is_empty() {
            // todo test labels parsing. Add profile with labels
            let mut ls = vec![];
            for (k, v) in self.label.iter() {
                let tmp: String = v.iter().map(ToString::to_string).collect();
                ls.push(format!("{}:[{}]", k, tmp));
            }

            ls.sort();
            ss.push(format!("{}{}", label_header, ls.join(" ")))
        }

        if !self.num_label.is_empty() {
            let mut ls: Vec<String> = vec![];

            for (k, v) in self.num_label.iter() {
                let units = self.num_unit_label.get(k);
                let mut label_string = String::new();

                match units {
                    None => {
                        label_string.push_str(&format!(
                            "{}:[{}]",
                            k,
                            v.iter().map(|v| { v.to_string() }).collect::<String>()
                        ));
                    }
                    Some(units) => {
                        if units.len() == v.len() {
                            let mut values = vec![];
                            for _ in 0..v.len() {
                                values.push(String::new());
                            }
                            //alignment:[3 kilobytes 4 kilobytes] bytes:[3 4] key1:[1 2] key2:[3 4] requests:[1  1  3 seconds 4  5 s]
                            for (i, vv) in v.iter().enumerate() {
                                values[i] = format!("{} {}", vv, units[i]);
                            }

                            label_string.push_str(&format!(
                                "{}:[{}]",
                                k,
                                values
                                    .iter()
                                    .map(|v| { format!("{} ", v) })
                                    .collect::<String>()
                            ));
                        } else {
                            label_string.push_str(&format!(
                                "{}:[{}]",
                                k,
                                v.iter().map(ToString::to_string).collect::<String>()
                            ));
                        }
                    }
                }
                ls.push(label_string);
            }
            ls.sort();
            ss.push(format!("{}{}", label_header, ls.join(" ")));
        }

        ss.join("\n")
    }
}
