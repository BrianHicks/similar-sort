use clap::{crate_authors, crate_version, App, Arg};
use color_eyre::eyre::{ContextCompat, Result, WrapErr};
use rayon::prelude::*;
use std::io::{self, stdin, stdout, BufRead, BufWriter, Write};
use strsim::levenshtein;

#[global_allocator]
static A: bump_alloc::BumpAlloc = bump_alloc::BumpAlloc::new();

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
        .about(
            "works like `sort`, but sorts according to edit distance instead of alphanumerically.",
        )
        .arg(Arg::new("target").about("sort according to distance from this string"))
        .get_matches();

    let target = matches
        .value_of("target")
        .context("could not retrieve target from args. Internal error; please report!")?;

    let lines: Vec<String> = stdin()
        .lock()
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .context("could not read lines from stdin")?;

    let mut distances: Vec<(usize, &String)> = lines
        .iter()
        .map(|candidate| (levenshtein(target, candidate), candidate))
        .collect();

    distances.par_sort_unstable_by_key(|x| x.0);

    let mut out = BufWriter::new(stdout());
    for (_, candidate) in distances {
        writeln!(out, "{}", candidate).context("could not write to stdout")?;
    }
    out.flush().context("could not finish writing to stdout")?;

    Ok(())
}
