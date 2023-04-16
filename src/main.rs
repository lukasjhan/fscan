extern crate clap;
use clap::Arg;
use std::str::FromStr;
mod lib;

fn validate_int(s: String) -> Result<(), String> {
    match i64::from_str(&s) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:?} is not a valid integer: {}", s, e)),
    }
}

fn main() {
    let matches = clap::App::new(env!("CARGO_PKG_NAME"))
                            .version(concat!("v", env!("CARGO_PKG_VERSION")))
                            .about(env!("CARGO_PKG_DESCRIPTION"))
                            .arg(Arg::with_name("DIRECTORY")
                                     .help("Directory to list")
                                     .index(1)
                                     .default_value("."))
                            .arg(Arg::with_name("follow-symlinks")
                                     .help("Follow any symbolic links encountered")
                                     .short("s"))
                            .arg(Arg::with_name("max-depth")
                                     .help("Maximal directory depth to recurse, or -1 for infinite")
                                     .short("d")
                                     .default_value("5")
                                     .takes_value(true)
                                     .validator(validate_int))
                            .arg(Arg::with_name("raw-size")
                                     .help("Display raw size in bytes")
                                     .short("r"))
                            .get_matches();

    let follow_symlinks = matches.is_present("follow-symlinks");
    let max_depth = i64::from_str(matches.value_of("max-depth").unwrap()).unwrap();
    let size_format = match matches.is_present("raw-size") {
        true => lib::print::SizeFormat::Raw,
        false => lib::print::SizeFormat::PRETTY,
    };
    let path = matches.value_of("DIRECTORY").unwrap();

    lib::print::print_tree(
        &lib::read_recursive(
            std::path::Path::new(&path),
            follow_symlinks,
        ),
        max_depth,
        size_format,
    );
}
