#![allow(dead_code)]
use profile::buffer::ProfileDecoder;
use std::path::Path;

mod binutils;
mod driver;
mod errors;
mod http_server;
mod profile;
mod reports;

#[derive(Default)]
pub struct Options {
    binary_path: String,
}

fn pprof(op: &mut Options) {
    if op.binary_path.is_empty() {
        // this is warning, but ok
        eprintln!("binary path is empty")
    }

    // if wrong path provided -> panic
    if !Path::exists(op.binary_path.as_ref()) {
        panic!(format!("provided path {} does not exist", op.binary_path));
    }

    load_binary(&op.binary_path);
}

fn load_binary(path: &str) {
    let binary = std::fs::read(path);
    match binary {
        Ok(data) => {
            let p = profile::buffer::Buffer::unmarshal(data);
            match p {
                Ok(res) => {
                    println!("{}", res.to_string());
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("{}", err.to_string());
        }
    }
}

#[cfg(test)]
mod lib_tests {
    #[test]
    fn main_tests() {
        let mut op = super::Options::default();
        op.binary_path = "tests/CPU.pb.gz".to_string();
        super::pprof(&mut op);
    }
}
