use clap::{crate_authors, crate_version, App, Arg, ArgGroup};
use color_eyre::eyre::{ContextCompat, Result, WrapErr};
use rayon::prelude::*;
use std::io::{self, stdin, stdout, BufRead, BufWriter, Write};
use strsim::{jaro_winkler, levenshtein};

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
        .long_about(
            "works like `sort`, but sorts according to edit distance instead of alphanumerically.\n\nYou can choose the edit distance algorithm we use for this! If you don't know which one you need, Levenshtein is a good default. Try Jaro-Winkler if you care about your strings having similar prefixes (for example files in a project.)"
        )
        .arg(Arg::new("target").about("sort according to distance from this string"))
        .arg(
            Arg::new("levenshtein")
                .long("levenshtein")
                .about("sort according to Levenshtein distance (the default)"),
        )
        .arg(
            Arg::new("jaro-winkler")
                .long("jaro-winkler")
                .about("sort according to Jaro-Winkler edit distance"),
        )
        .group(
            ArgGroup::new("edit-method")
                .arg("levenshtein")
                .arg("jaro-winkler")
        )
        .get_matches();

    let target = matches
        .value_of("target")
        .context("could not retrieve target from args. Internal error; please report!")?;

    let lines: Vec<String> = stdin()
        .lock()
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .context("could not read lines from stdin")?;

    let mut distances: Vec<(usize, &String)> = if matches.is_present("jaro-winkler") {
        lines
            .iter()
            .map(|candidate| {
                (
                    -(jaro_winkler(target, candidate) * 1000.0) as usize,
                    candidate,
                )
            })
            .collect()
    } else {
        // levenshtein, the default
        lines
            .iter()
            .map(|candidate| (levenshtein(target, candidate), candidate))
            .collect()
    };

    distances.par_sort_unstable_by_key(|x| x.0);

    let mut out = BufWriter::new(stdout());
    for (_, candidate) in distances {
        writeln!(out, "{}", candidate).context("could not write to stdout")?;
    }
    out.flush().context("could not finish writing to stdout")?;

    Ok(())
}
