use clap::{crate_authors, crate_version, App, Arg};
use color_eyre::eyre::Result;

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

    println!("{:?}", matches);

    Ok(())
}
