use color_eyre::eyre::{Result, WrapErr};
use rayon::prelude::*;
use std::io::{self, stdin, stdout, BufRead, BufWriter, Write};
use strsim::levenshtein;
use structopt::StructOpt;

// works like `sort`, but sorts according to Levenshtein distance instead of
// alphanumerically.
#[derive(StructOpt)]
#[structopt(name = "similar-sort")]
struct Opts {
    /// sort according to distance from this string
    target: String,
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{:?}", err);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    color_eyre::install()?;

    let opts = Opts::from_args();

    let mut lines: Vec<String> = stdin()
        .lock()
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .context("could not read lines from stdin")?;

    lines.par_sort_unstable_by_key(|candidate| levenshtein(&opts.target, candidate));

    let mut out = BufWriter::new(stdout());
    for candidate in lines {
        writeln!(out, "{}", candidate).context("could not write to stdout")?;
    }

    Ok(())
}
