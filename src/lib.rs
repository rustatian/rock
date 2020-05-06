#![allow(dead_code)]
#![warn(missing_debug_implementations, rust_2018_idioms)]
use crate::types::Options;
use rock_parser::profile;
use rock_parser::profile::buffer::ProfileDecoder;
use rock_parser::profile::Profile;
use rock_reports::reports;
use rock_utils::errors::RockError;
use rock_utils::types;
use std::path::Path;

fn pprof(op: &mut Options) {
    if op.profile_path.is_empty() {
        // this is warning, but ok
        eprintln!("binary path is empty")
    }

    // if wrong path provided -> panic
    if !Path::exists(op.profile_path.as_ref()) {
        panic!(format!("provided path {} does not exist", op.profile_path));
    }

    let profile = load_binary(&op.profile_path).unwrap();
    println!("{}", profile.to_string());

    let mut prooo = reports::Report::new(&profile, op);
}

fn load_binary(path: &str) -> Result<Profile, RockError> {
    match std::fs::read(path) {
        Ok(data) => match profile::buffer::Buffer::decode(data) {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        },
        Err(err) => Err(RockError::Unknown {
            reason: err.to_string(),
        }),
    }
}
