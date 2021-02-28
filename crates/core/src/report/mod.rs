use crate::profile::Profile;
use std::collections::HashMap;

struct Report<T>
where
    T: Fn(i64) -> String,
{
    prof: Profile,
    total: i64,
    format_value: T,
}

struct Options<T>
where
    T: Fn(&[i64]) -> i64,
{
    output_format: isize,

    cum_sort: bool,
    call_tree: bool,
    drop_negative: bool,
    compact_labels: bool,
    ratio: f64,
    title: String,
    profile_labels: Vec<String>,
    active_filters: Vec<String>,
    num_label_units: HashMap<String, String>,

    node_count: isize,
    node_fraction: f64,
    edge_fraction: f64,

    sample_value: T,
    sample_mean_divisor: T,
    sample_type: String,
    sample_unit: String, // Unit for the sample data from the profile.

    output_unit: String, // Units for data formatting in report.

    // Symbol     *regexp.Regexp // Symbols to include on disassembly report.
    source_path: String, // Search path for source files.
    trim_path: String,   // Paths to trim from source file paths.

    intel_syntax: bool, // Whether or not to print assembly in Intel syntax.
}
