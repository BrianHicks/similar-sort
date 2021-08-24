use clap::{crate_authors, crate_version, App, Arg};
use color_eyre::eyre::{ContextCompat, Result, WrapErr};
use rayon::prelude::*;
use std::io::{self, stdin, BufRead};
use strsim::levenshtein;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{:?}", err);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    color_eyre::install()?;

    let matches = App::new("similar-sort")
        .version(crate_version!())
        .author(crate_authors!())
        .about("works like `sort`, but sorts according to Levenshtein distance instead of alphanumerically")
        .arg(
            Arg::with_name("target")
                .value_name("TARGET")
                .help("sort according to distance from this string")
                .required(true)
        ).get_matches();

    let target = matches.value_of("target").context(
        "could not get the target value. This is an internal error and should be reported.",
    )?;

    let mut lines: Vec<String> = stdin()
        .lock()
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .context("could not read lines from stdin")?;

    lines.par_sort_by_key(|candidate| levenshtein(target, candidate));

    for candidate in lines {
        println!("{}", candidate);
    }

    Ok(())
}
