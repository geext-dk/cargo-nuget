#[macro_use]
extern crate quick_error;
extern crate clap;
extern crate term_painter;
extern crate xml;
extern crate zip;
extern crate toml;
extern crate semver;
extern crate chrono;

#[cfg(test)]
#[macro_use]
mod test_utils;

#[macro_use]
mod macros;

pub mod cargo;
pub mod nuget;
mod args;

use std::error::Error;

use term_painter::ToStyle;
use term_painter::Color::*;

use clap::ArgMatches;

fn main() {
    let args = args::app().get_matches();

    if let Some(args) = args.subcommand_matches(args::PACK_CMD) {
        match build(args) {
            Ok(_) => {
                println!("{}", Green.paint("The build finished successfully"));
            }
            Err(e) => {
                println!("{}", Red.paint(e));
                println!("\n{}",
                         Red.bold().paint("The build did not finish successfully"));
            }
        }
    }
    else {
        args::app().print_help().unwrap();
        println!("");
    }
}

fn build(args: &ArgMatches) -> Result<(), Box<Error>> {
    let cargo_toml = pass!("reading cargo manifest" => args => cargo::parse_toml);

    let cargo_lib = pass!("building Rust lib" => (args, &cargo_toml) => |args| {
        let result = cargo::build_lib(args);
        println!("");

        result
    });

    let nuspec = pass!("building nuspec" => &cargo_toml => nuget::spec);

    let nupkg = pass!("building nupkg" => (&nuspec, &cargo_lib) => nuget::pack);

    pass!("saving nupkg" => (args, &nupkg) => nuget::save_nupkg);

    Ok(())
}
