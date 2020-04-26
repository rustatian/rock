use clap::{App, AppSettings, Arg, SubCommand};
use profile::buffer::ProfileDecoder;
mod binutils;
mod driver;
mod errors;
mod graph;
mod http_server;
mod profile;

fn main() {
    let cli = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("path")
                .about("Path to pprof binary")
                .arg(
                    Arg::with_name("PATH")
                        .help("pprof saved binary")
                        .required(false),
                ),
        )
        .subcommand(SubCommand::with_name("profile").about(
            "backward compatibility \
             with microservices profiles",
        ))
        .get_matches();

    match cli.subcommand() {
        ("path", Some(matches)) => {
            let path = matches.value_of("PATH");
            match path {
                Some(p) => {
                    load_binary(p);
                }

                None => eprintln!("Error: no PATH provided for the pprof binary"),
            }
        }
        ("profile", Some(_matches)) => {
            //profile_microservices();
        }
        _ => unreachable!(),
    }
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
