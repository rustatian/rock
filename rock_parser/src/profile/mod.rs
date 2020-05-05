#![warn(missing_debug_implementations, rust_2018_idioms)]
use crate::profile::buffer::{decode_string, Buffer, WireTypes, decode_varint};
use chrono::NaiveDateTime;
use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use rock_utils::errors::RockError;

pub mod buffer;
mod function;
mod label;
mod line;
mod location;
mod mapping;
mod sample;
mod value_type;

const NSEC_IN_SECOND: i64 = 1_000_000_000;

pub trait Decoder<T> {
    fn decode(buf: &mut Buffer, data: &mut Vec<u8>) -> T;
}

// TODO ADD OPTIONAL TO THE STRUCT FIELDS
// TODO BUG, getString(p.stringTable, &p.dropFramesX, err) p.dropFramesX and similar logic. p.dropFramesX should became 0 !!!!
// Profile is an in-memory representation of profile.proto

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Profile {
    // A description of the samples associated with each Sample.value.
    // For a cpu profile this might be:
    //   [["cpu","nanoseconds"]] or [["wall","seconds"]] or [["syscall","count"]]
    // For a heap profile, this might be:
    //   [["allocations","count"], ["space","bytes"]],
    // If one of the values represents the number of events represented
    // by the sample, by convention it should be at index 0 and use
    // sample_type.unit == "count".
    sample_type: Vec<value_type::ValueType>,
    // The set of samples recorded in this profile.
    sample: Vec<sample::Sample>,
    // Mapping from address ranges to the image/binary/library mapped
    // into that address range.  mapping[0] will be the main binary.
    mapping: Vec<mapping::Mapping>,
    // Useful program location
    pub location: Vec<location::Location>,
    // Functions referenced by locations
    function: Vec<function::Function>,
    // A common table for strings referenced by various messages.
    // string_table[0] must always be "".
    string_table: Vec<String>,
    // frames with Function.function_name fully matching the following
    // regexp will be dropped from the samples, along with their successors.
    drop_frames: String,
    // Index into string table.
    // frames with Function.function_name fully matching the following
    // regexp will be kept, even if it matches drop_functions.
    keep_frames: String, // Index into string table.

    // The following fields are informational, do not affect
    // interpretation of results.
    // Time of collection (UTC) represented as nanoseconds past the epoch.
    time_nanos: i64,
    // Duration of the profile, if a duration makes sense.
    duration_nanos: i64,
    // The kind of events between sampled ocurrences.
    // e.g [ "cpu","cycles" ] or [ "heap","bytes" ]
    period_type: Option<value_type::ValueType>,
    // The number of events between sampled occurrences.
    period: i64,
    // Freeform text associated to the profile.
    comments: Vec<String>,
    // Indices into string table.
    // Index into the string table of the type of the preferred sample
    // value. If unset, clients should default to the last sample value.
    default_sample_type: String,

    // Index into string table.
    comment_index: Vec<i64>,
    // Index into string table.
    drop_frames_index: i64,
    // Index into string table.
    keep_frames_index: i64,

    // Index into string table.
    default_sample_type_index: i64,
}

type NumLabelUnitsWithIgnored = (HashMap<String, String>, HashMap<String, Vec<String>>);

/// Text representation of a profile. For debugging and testing purposes.
impl ToString for Profile {
    #[inline]
    fn to_string(&self) -> String {
        // pre-allocate space for vector
        let mut ss: Vec<String> = Vec::with_capacity(
            self.comments.len() + self.sample.len() + self.mapping.len() + self.location.len(),
        );

        // COMMENT SECTION START =================================
        for c in self.comments.iter() {
            ss.push(format!("Comment: {}", c))
        }

        match self.period_type {
            // it is possible, that there is no pt
            None => {}
            Some(ref pt) => ss.push(format!("PeriodType: {} {}", pt.r#type, pt.unit)),
        }
        // PERIOD SECTION START =================================
        ss.push(format!("Period: {}", self.period));
        match self.time_nanos {
            tn if tn > 0 => {
                // 2001-09-09 01:46:40 <-- data format
                ss.push(format!(
                    "Time UTC: {}",
                    NaiveDateTime::from_timestamp((tn / NSEC_IN_SECOND) as i64, 0)
                ));
            }
            _ => {
                // skip
            }
        }

        if self.duration_nanos != 0 {
            ss.push(format!(
                "Duration: {}s",
                std::time::Duration::from_nanos(self.duration_nanos as u64).as_secs_f64()
            ))
        }

        // SAMPLES SECTION START =================================
        ss.push("Samples:".to_string());

        let mut samples = String::new();
        for s in self.sample_type.iter() {
            let dflt = if s.r#type == self.default_sample_type {
                String::from("[dflt]")
            } else {
                String::new()
            };
            if samples.is_empty() {
                samples = format!("{}/{}{} ", s.r#type, s.unit, dflt);
                continue;
            }
            samples = format!("{}{}/{}{} ", samples, s.r#type, s.unit, dflt);
        }

        samples.drain((samples.len() - 1)..);
        ss.push(samples);
        for s in self.sample.iter() {
            ss.push(s.to_string());
        }

        ss.push("Locations".to_string());
        for l in self.location.iter() {
            ss.push(l.to_string());
        }

        ss.push("Mappings".to_string());
        for m in self.mapping.iter() {
            ss.push(m.to_string())
        }

        return format!("{}{}", ss.join("\n"), "\n");
    }
}

impl Profile {
    #[inline]
    pub fn decode_profile_field(&mut self, buf: &mut Buffer, data: &mut Vec<u8>) {
        match buf.field {
            // repeated ValueType sample_type = 1
            1 => {
                self.sample_type
                    .push(value_type::ValueType::decode(buf, data));
            }
            // repeated Sample sample = 2
            2 => {
                self.sample.push(sample::Sample::decode(buf, data));
            }
            // repeated Mapping mapping = 3
            3 => {
                self.mapping.push(mapping::Mapping::decode(buf, data));
            }
            // repeated Location location = 4
            4 => {
                self.location.push(location::Location::decode(buf, data));
            }
            // repeated Function function = 5
            5 => {
                self.function.push(function::Function::decode(buf, data));
            }
            // repeated string string_table = 6
            6 => {
                self.string_table.push(decode_string(data));
                if self.string_table[0] != "" {
                    panic!("String table[0] should be empty");
                }
            }
            // int64 drop_frames = 7
            7 => {
                self.drop_frames_index = buf.u64 as i64;
            }
            // int64 keep_frames = 8
            8 => {
                self.keep_frames_index = buf.u64 as i64;
            }
            // int64 time_nanos = 9
            9 => {
                //https://github.com/google/pprof/issues/273
                if self.time_nanos != 0 {
                    panic!("concatenated profiles detected")
                }
                self.time_nanos = buf.u64 as i64;
            }
            // int64 duration_nanos = 10
            10 => {
                self.duration_nanos = buf.u64 as i64;
            }
            // ValueType period_type = 11
            11 => {
                self.period_type = Option::from(value_type::ValueType::decode(buf, data));
            }
            // int64 period = 12
            12 => {
                self.period = buf.u64 as i64;
            }
            // repeated int64 comment = 13
            13 => match buf.r#type {
                WireTypes::WireBytes => loop {
                    if !data.is_empty() {
                        let res = decode_varint(data);
                        match res {
                            Ok(varint) => self.comment_index.push(varint as i64),
                            Err(err) => {
                                panic!(err);
                            }
                        }
                    } else {
                        break;
                    }
                },
                _ => self.comment_index.push(buf.u64 as i64),
            },
            // int64 defaultSampleType = 14
            14 => {
                self.default_sample_type_index = buf.u64 as i64;
            }
            _ => {}
        }
    }

    #[inline]
    pub fn post_decode(&mut self) {
        // MAPPING DECODE
        let mut mappings: HashMap<u64, mapping::Mapping> = HashMap::new();
        for m in self.mapping.iter_mut() {
            m.filename = self.string_table[m.filename_index as usize].to_string();
            m.build_id = self.string_table[m.build_id_index as usize].to_string();
            mappings.insert(m.id, m.to_owned());
        }

        // FUNCTION DECODE
        let mut functions: HashMap<u64, function::Function> = HashMap::new();
        for f in self.function.iter_mut() {
            f.name = self.string_table[f.name_index as usize].to_string();
            f.system_name = self.string_table[f.system_name_index as usize].to_string();
            f.filename = self.string_table[f.filename_index as usize].to_string();
            functions.insert(f.id, f.to_owned());
        }

        //LOCATION DECODE
        let mut locations: HashMap<u64, location::Location> = HashMap::new();
        for loc in self.location.iter_mut() {
            match mappings.get(loc.mapping_index.borrow()) {
                None => {}
                Some(m) => {
                    loc.mapping = Option::from(m.clone());
                }
            }

            for line in loc.line.iter_mut() {
                if line.function_index != 0 {
                    line.function = functions.get(line.function_index.borrow()).unwrap().clone();
                }
            }

            locations.insert(loc.id, loc.to_owned());
        }

        for st in self.sample_type.iter_mut() {
            st.unit = self.string_table[st.unit_index as usize].to_string();
            st.r#type = self.string_table[st.type_index as usize].to_string();
        }

        for s in self.sample.iter_mut() {
            let mut labels: HashMap<String, Vec<String>> = HashMap::new();
            let mut num_labels: HashMap<String, Vec<i64>> = HashMap::new();
            let mut num_units: HashMap<String, Vec<String>> = HashMap::new();

            for label_index in s.label_index.iter() {
                // key can't be empty
                let key = self.string_table[label_index.key_index as usize].to_string();

                if label_index.str_index != 0 {
                    let key_value = self.string_table[label_index.str_index as usize].to_string();
                    // using or_insert_with because: The function will always be called and potentially allocate an object acting as the default.
                    labels
                        .entry(key)
                        .and_modify(|e| e.push(key_value.clone()))
                        .or_insert_with(|| vec![key_value]);
                } else if label_index.num_index != 0 {
                    //let num_values = num_labels.get(&key); // used only to padStringArray
                    //let units = num_units.get_mut(&key);
                    if label_index.num_unit_index != 0 {
                        let unit =
                            self.string_table[label_index.num_unit_index as usize].to_string();

                        let num_len = num_labels.get(&key).unwrap_or(&Vec::<i64>::new()).len();
                        let units_len = num_units.get(&key).unwrap_or(&Vec::<String>::new()).len();

                        if num_len > units_len {
                            match num_units.entry(key.clone()) {
                                Entry::Occupied(mut e) => {
                                    e.get_mut().resize(num_len, String::new());
                                }
                                Entry::Vacant(e) => {
                                    let mut v: Vec<String> = Vec::new();
                                    for _ in 0..num_len - units_len {
                                        v.push(String::new());
                                    }
                                    e.insert(v);
                                }
                            }
                        }

                        num_units
                            .entry(key.clone())
                            .and_modify(|e| e.push(unit.clone()))
                            .or_insert_with(|| vec![unit]);
                    }

                    num_labels
                        .entry(key)
                        .and_modify(|e| e.push(label_index.num_index))
                        .or_insert_with(|| vec![label_index.num_index]);
                }
            }

            if !labels.is_empty() {
                s.label = labels;
            }

            if !num_labels.is_empty() {
                s.num_label = num_labels.clone();

                for (key, units) in num_units.iter_mut() {
                    if num_labels.get(key).is_some()
                        && !units.is_empty()
                        && units.len() > num_labels.get(key).unwrap().len()
                    {
                        for _ in 0..units.len() - num_labels.get(key).unwrap().len() {
                            units.push(String::new());
                        }
                    }
                }

                s.num_unit_label = num_units;
            }

            for loc_index in &s.location_index.clone() {
                s.location.push(locations.get(loc_index).unwrap().clone())
            }
        }

        match self.period_type.as_mut() {
            None => {
                // just skip if none
            }
            Some(vt) => {
                vt.r#type = self.string_table[vt.type_index as usize].to_string();
                vt.unit = self.string_table[vt.unit_index as usize].to_string()
            }
        }

        self.drop_frames = self.string_table[self.drop_frames_index as usize].to_string();
        self.keep_frames = self.string_table[self.keep_frames_index as usize].to_string();

        for comment_index in self.comment_index.iter() {
            self.comments
                .push(self.string_table[*comment_index as usize].to_string())
        }

        self.default_sample_type =
            self.string_table[self.default_sample_type_index as usize].to_string();
    }

    #[inline]
    pub fn validate(&self) -> Result<(), RockError> {
        if self.sample_type.is_empty() && self.sample.is_empty() {
            panic!("missing sample type information");
        }

        for s in self.sample.iter() {
            if *s == sample::Sample::default() {
                panic!("profile has default (uninitialized) sample")
            }
            if s.value.len() != self.sample_type.len() {
                panic!(
                    "mismatch: sample has {} values vs. {} types",
                    s.value.len(),
                    self.sample_type.len()
                );
            }

            for l in s.location.iter() {
                if *l == location::Location::default() {
                    return Err(RockError::ValidationFailed {
                        reason: String::from("sample has default (uninitialized) location"),
                    });
                }
            }
        }

        // Check that all mappings/locations/functions are in the tables
        // Check that there are no duplicate ids
        let mut mappings = HashMap::<u64, mapping::Mapping>::new();
        for m in self.mapping.iter() {
            if *m == mapping::Mapping::default() {
                return Err(RockError::ValidationFailed {
                    reason: String::from("profile has default (uninitialized) mapping"),
                });
            }
            if m.id == 0 {
                return Err(RockError::ValidationFailed {
                    reason: String::from("found mapping with reserved ID=0"),
                });
            }

            match mappings.entry(m.id) {
                Entry::Occupied(_) => {
                    return Err(RockError::ValidationFailed {
                        reason: format!("multiple mappings with same id: {}", m.id),
                    });
                }
                Entry::Vacant(_) => {
                    //everything OK , continue
                }
            }

            mappings.insert(m.id, m.clone());
        }

        let mut functions = HashMap::<u64, function::Function>::new();
        for f in self.function.iter() {
            if *f == function::Function::default() {
                return Err(RockError::ValidationFailed {
                    reason: String::from("profile has default (uninitialized) function"),
                });
            }
            if f.id == 0 {
                return Err(RockError::ValidationFailed {
                    reason: String::from("found function with reserved ID=0"),
                });
            }

            match functions.entry(f.id) {
                Entry::Occupied(_) => {
                    return Err(RockError::ValidationFailed {
                        reason: format!("multiple functions with same id: {}", f.id),
                    });
                }
                Entry::Vacant(_) => {
                    //everything OK
                }
            }

            functions.insert(f.id, f.clone());
        }

        let mut locations = HashMap::<u64, location::Location>::new();
        for l in self.location.iter() {
            if *l == location::Location::default() {
                return Err(RockError::ValidationFailed {
                    reason: String::from("profile has default (uninitialized) location"),
                });
            }
            if l.id == 0 {
                return Err(RockError::ValidationFailed {
                    reason: String::from("found location with reserved ID=0"),
                });
            }

            match locations.entry(l.id) {
                Entry::Occupied(_) => {
                    return Err(RockError::ValidationFailed {
                        reason: format!("multiple locations with same id: {}", l.id),
                    });
                }
                Entry::Vacant(_) => {
                    //everything OK
                }
            }

            locations.insert(l.id, l.clone());

            match &l.mapping {
                None => {}
                Some(m) => {
                    if m.id == 0 || mappings.get(&m.id) != Some(m) {
                        return Err(RockError::ValidationFailed {
                            reason: format!("inconsistent mapping {:?}: {}", m, m.id),
                        });
                    }
                }
            }

            // according to https://github.com/google/pprof/pull/517
            // check_valid function
            for ln in l.line.iter() {
                if ln.function != function::Function::default()
                    && (ln.function.id == 0
                        || functions.get(&ln.function.id) != Some(ln.function.borrow()))
                {
                    return Err(RockError::ValidationFailed {
                        reason: format!(
                            "inconsistent function {:?}: {}",
                            ln.function, ln.function.id
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    // NumLabelUnits returns a map of numeric label keys to the units
    // associated with those keys and a map of those keys to any units
    // that were encountered but not used.
    // Unit for a given key is the first encountered unit for that key. If multiple
    // units are encountered for values paired with a particular key, then the first
    // unit encountered is used and all other units are returned in sorted order
    // in map of ignored units.
    // If no units are encountered for a particular key, the unit is then inferred
    // based on the key.
    pub fn num_label_units(&self) -> Result<NumLabelUnitsWithIgnored, RockError> {
        let mut num_label_units: HashMap<String, String> = HashMap::new();
        let mut ignored_units: HashMap<String, HashMap<String, bool>> = HashMap::new();
        let mut encountered_keys: HashMap<String, bool> = HashMap::new();

        // Determine units based on numeric tags for each sample.
        for (_, s) in self.sample.iter().enumerate() {
            for (k, _) in s.num_label.iter() {
                encountered_keys.insert(String::from(k), true);

                if let Some(unit_vec) = s.num_unit_label.get(k) {
                    for unit in unit_vec {
                        if unit.is_empty() {
                            continue;
                        }

                        if let Some(want_unit) = num_label_units.get(k) {
                            if want_unit != unit {
                                ignored_units
                                    .entry(String::from(k))
                                    .and_modify(|f| {
                                        f.insert(String::from(unit), true);
                                    })
                                    .or_insert_with(HashMap::new);
                            }
                        } else {
                            num_label_units.insert(String::from(k), String::from(unit));
                        }
                    }
                } else {
                    continue;
                }
            }
        }

        // Infer units for keys without any units associated with
        // numeric tag values.
        for (key, _) in encountered_keys {
            let unit = num_label_units.get(&key);
            match unit {
                None => {
                    match key.as_ref() {
                        "alignment" | "request" => {
                            num_label_units.insert(key, String::from("bytes"));
                        }
                        _ => {
                            num_label_units.insert(key.clone(), key);
                        }
                    };
                }
                Some(_) => {}
            }
        }

        // Copy ignored units into more readable format
        let mut units_ignored: HashMap<String, Vec<String>> = HashMap::new();

        for (key, values) in ignored_units.iter() {
            let mut units: Vec<String> = vec![String::new(); values.len()];

            for (i, value) in values.iter().enumerate() {
                let (unit, _) = value;
                units.insert(i as usize, String::from(unit));
            }

            units.sort();
            units_ignored.insert(String::from(key), units);
        }

        Ok((num_label_units, units_ignored))
    }
}

#[cfg(test)]
mod tests {
    use crate::profile::sample::Sample;
    use crate::profile::Profile;
    use std::collections::HashMap;
    use std::hash::Hash;

    macro_rules! tag_vals_init (
        { $($key:expr => $value:expr),+ } => {
            {
                let mut tag_vals = ::std::vec::Vec::new();
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.entry($key.to_string())
                        .and_modify(|v:&mut Vec<i64>| v.push($value as i64))
                        .or_insert(vec![$value as i64]);
                )+
                tag_vals.push(m);
                tag_vals
            }
         };
    );

    macro_rules! tag_units_init (
        { $($key:expr => $value:expr),+ } => {
            {
                let mut tag_vals = ::std::vec::Vec::new();
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.entry($key.to_string())
                        .and_modify(|v:&mut Vec<String>| v.push($value.to_string()))
                        .or_insert(vec![$value.to_string()]);
                )+
                tag_vals.push(m);
                tag_vals
            }
         };
    );

    macro_rules! want_units_init (
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key.to_string(), $value.to_string());
                )+
                m
            }
         };
    );

    macro_rules! want_ignored_units_init (
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.entry($key.to_string())
                        .and_modify(|v:&mut Vec<String>| v.push($value.to_string()))
                        .or_insert(vec![$value.to_string()]);
                )+
                m
            }
         };
    );

    #[test]
    fn test_num_label_units() {
        struct TagFilterTests {
            desc: String,
            tag_vals: Vec<HashMap<String, Vec<i64>>>,
            tag_units: Vec<HashMap<String, Vec<String>>>,
            want_units: HashMap<String, String>,
            want_ignored_units: HashMap<String, Vec<String>>,
        }

        let mut tests = vec![];

        tests.push(TagFilterTests {
            desc: String::from("One sample, multiple keys, different specified units"),
            tag_vals: tag_vals_init! {"key1" => 131_072, "key2" => 128},
            tag_units: tag_units_init!("key1" => "bytes", "key2" => "kilobytes"),
            want_units: want_units_init!("key1" => "bytes", "key2" => "kilobytes"),
            want_ignored_units: HashMap::new(),
        });
        tests.push(TagFilterTests {
            desc: String::from("One sample, one key with one value, unit specified"),
            tag_vals: tag_vals_init! {"key1" => 8},
            tag_units: tag_units_init!("key1" => "bytes"),
            want_units: want_units_init!("key1" => "bytes"),
            want_ignored_units: HashMap::new(),
        });
        tests.push(TagFilterTests {
            desc: String::from("One sample, one key with one value, empty unit specified"),
            tag_vals: tag_vals_init! {"key1" => 8},
            tag_units: tag_units_init!("key1" => ""),
            want_units: want_units_init!("key1" => "key1"),
            want_ignored_units: HashMap::new(),
        });
        tests.push(TagFilterTests {
            desc: String::from("Key bytes, unit not specified"),
            tag_vals: tag_vals_init! {"bytes" => 8},
            tag_units: Vec::new(),
            want_units: want_units_init!("bytes" => "bytes"),
            want_ignored_units: HashMap::new(),
        });
        tests.push(TagFilterTests {
            desc: String::from("One sample, one key with one value, unit not specified"),
            tag_vals: tag_vals_init! {"kilobytes" => 8},
            tag_units: Vec::new(),
            want_units: want_units_init!("kilobytes" => "kilobytes"),
            want_ignored_units: HashMap::new(),
        });
        tests.push(TagFilterTests {
            desc: String::from("Key request, unit not specified"),
            tag_vals: tag_vals_init! {"request" => 8},
            tag_units: Vec::new(),
            want_units: want_units_init!("request" => "bytes"),
            want_ignored_units: HashMap::new(),
        });
        tests.push(TagFilterTests {
            desc: String::from("Key alignment, unit not specified"),
            tag_vals: tag_vals_init! {"alignment" => 8},
            tag_units: Vec::new(),
            want_units: want_units_init!("alignment" => "bytes"),
            want_ignored_units: HashMap::new(),
        });
        tests.push(TagFilterTests {
            desc: String::from("One sample, one key with multiple values and two different units"),
            tag_vals: tag_vals_init! {"key1" => 8, "key1" => 8},
            tag_units: tag_units_init!("key1" => "bytes", "key1" => "kilobytes"),
            want_units: want_units_init!("key1" => "bytes"),
            want_ignored_units: want_ignored_units_init!("key1" => "kilobytes"),
        });
        tests.push(TagFilterTests {
            desc: String::from("One sample, one key with multiple values and three different units"),
            tag_vals: tag_vals_init! {"key1" => 8, "key1" => 8},
            tag_units: tag_units_init!("key1" => "bytes", "key1" => "megabytes", "key1" => "kilobytes"),
            want_units: want_units_init!("key1" => "bytes"),
            want_ignored_units: want_ignored_units_init!("key1" => "kilobytes", "key1" => "megabytes"),
        });
        tests.push(TagFilterTests {
            desc: String::from("Two samples, one key, different units specified"),
            tag_vals: tag_vals_init! {"key1" => 8, "key1" => 8},
            tag_units: tag_units_init!("key1" => "bytes", "key1" => "kilobytes"),
            want_units: want_units_init!("key1" => "bytes"),
            want_ignored_units: want_ignored_units_init!("key1" => "kilobytes"),
        });
        tests.push(TagFilterTests {
            desc: String::from("Keys alignment, request, and bytes have units specified"),
            tag_vals: tag_vals_init! {"alignment" => 8, "request" => 8, "bytes" => 8},
            tag_units: tag_units_init!("alignment" => "seconds", "request" => "minutes", "bytes" => "hours"),
            want_units: want_units_init!("alignment" => "seconds", "request" => "minutes", "bytes" => "hours"),
            want_ignored_units: HashMap::new(),
        });

        for test in tests {
            let mut p = Profile::default();
            for (i, num_label) in test.tag_vals.iter().enumerate() {
                let mut s = Sample::default();

                s.num_label = num_label.clone();
                if test.tag_units.is_empty() {
                    s.num_unit_label = HashMap::new();
                } else {
                    s.num_unit_label = test.tag_units[i].clone();
                }

                p.sample.push(s);
            }

            let (units, ignore_units) = p.num_label_units().unwrap();

            assert_eq!(keys_match(&units, &test.want_units), true);
            assert_eq!(keys_match(&ignore_units, &test.want_ignored_units), true);
        }
    }

    fn keys_match<T: Eq + Hash, U>(map1: &HashMap<T, U>, map2: &HashMap<T, U>) -> bool {
        map1.len() == map2.len() && map1.keys().all(|k| map2.contains_key(k))
    }
}
