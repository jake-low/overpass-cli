use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::process;

use clap::{Parser, ValueEnum};

const DEFAULT_SERVER: &str = "https://overpass-api.de";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Output format
    #[arg(short = 'f', long, value_enum)]
    format: Option<Format>,

    /// Output type
    #[arg(short = 'o', long = "out", value_enum)]
    output: Option<Output>,

    /// Global bounding box (implicitly applies to all statements)
    #[arg(long, num_args = 4, value_names = ["MIN_LON", "MIN_LAT", "MAX_LON", "MAX_LAT"], allow_hyphen_values = true)]
    bbox: Option<Vec<f64>>,

    /// Return results for a time in the past (ISO 8601 format)
    #[arg(long, conflicts_with = "diff", conflicts_with = "adiff")]
    date: Option<String>,

    /// Compare results at two different times (ISO 8601 format)
    #[arg(long, num_args = 1..=2, value_names = ["FROM", "TO"], conflicts_with = "date", conflicts_with = "adiff")]
    diff: Option<Vec<String>>,

    /// Like --diff, but returns augmented diff with extra information
    #[arg(long, num_args = 1..=2, value_names = ["FROM", "TO"], conflicts_with = "date", conflicts_with = "diff")]
    adiff: Option<Vec<String>>,

    /// Server URL
    #[arg(long, value_name = "URL", default_value = DEFAULT_SERVER)]
    server: String,

    /// Construct and print query but do not send to server
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// OverpassQL query string
    query: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Xml,
    Json,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Format::Xml => "xml",
            Format::Json => "json",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Output {
    Ids,
    Skel,
    Body,
    Tags,
    Meta,
    Center,
    Geom,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Output::Ids => "ids",
            Output::Skel => "skel",
            Output::Body => "body",
            Output::Tags => "tags",
            Output::Meta => "meta",
            Output::Center => "center",
            Output::Geom => "geom",
        };
        write!(f, "{}", s)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    let mut query = if let Some(query) = args.query {
        query.trim().to_string()
    } else {
        io::read_to_string(io::stdin())?
    };

    // prepare query settings
    let mut settings: HashMap<&str, String> = HashMap::new();

    if let Some(bbox) = args.bbox {
        settings.insert(
            "bbox",
            format!("{},{},{},{}", bbox[1], bbox[0], bbox[3], bbox[2]),
        );
    }

    if let Some(format) = args.format {
        settings.insert("out", format.to_string());
    }

    if let Some(date) = args.date {
        settings.insert("date", date);
    }

    if let Some(diff) = args.diff {
        settings.insert("diff", diff.join(","));
    }

    if let Some(adiff) = args.adiff {
        settings.insert("adiff", adiff.join(","));
    }

    // add settings to start of query
    if !query.starts_with('[') && !settings.is_empty() {
        query = format!(
            "{};\n{}",
            settings
                .iter()
                .map(|(k, v)| format!("[{}:{}]", k, v))
                .collect::<Vec<String>>()
                .join(""),
            query
        );
    }

    // add semicolon to end of query if missing
    if !query.ends_with(';') {
        query = format!("{};", query);
    }

    // add output format if missing
    if !(query
        .split(';')
        .rev()
        .fuse()
        .nth(1)
        .unwrap()
        .starts_with("out "))
    {
        let out = args.output.unwrap_or(Output::Body);
        query = format!("{}\nout {};", query, out);
    }

    if args.dry_run {
        println!("{}", query);
        process::exit(0);
    }

    let endpoint = format!("{}/api/interpreter", args.server);
    let res = ureq::post(&endpoint).send_form(&[("data", &query)])?;

    match res.content_type() {
        "application/json" => {
            jsonxf::pretty_print_stream(&mut res.into_reader(), &mut io::stdout())?;
            writeln!(&mut io::stdout())?; // add trailing newline
        }
        _ => {
            io::copy(&mut res.into_reader(), &mut io::stdout())?;
        }
    }

    Ok(())
}
