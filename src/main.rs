use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::process;

use clap::{Parser, ValueEnum};

const DEFAULT_SERVER: &str = "https://overpass-api.de";
const DEFAULT_NOMINATIM_SERVER: &str = "https://nominatim.openstreetmap.org";

fn user_agent() -> String {
    format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Output format
    #[arg(short = 'f', long, value_enum)]
    format: Option<Format>,

    /// Output type
    #[arg(short = 'o', long = "out", value_enum)]
    output: Option<Output>,

    /// Area name to geocode with Nominatim (available as 'area' in the query)
    #[arg(long, value_name = "SEARCH_STRING")]
    area: Option<String>,

    /// Global bounding box (implicitly applies to all statements)
    #[arg(long, num_args = 4, value_names = ["MIN_LON", "MIN_LAT", "MAX_LON", "MAX_LAT"], allow_hyphen_values = true)]
    bbox: Option<Vec<f64>>,

    /// Query timeout in seconds
    #[arg(long)]
    timeout: Option<u32>,

    /// Return results for a time in the past (ISO 8601 format)
    #[arg(long, conflicts_with = "diff", conflicts_with = "adiff")]
    date: Option<String>,

    /// Compare results at two different times (ISO 8601 format)
    #[arg(long, num_args = 1..=2, value_names = ["FROM", "TO"], conflicts_with = "date", conflicts_with = "adiff")]
    diff: Option<Vec<String>>,

    /// Like --diff, but returns augmented diff with extra information
    #[arg(long, num_args = 1..=2, value_names = ["FROM", "TO"], conflicts_with = "date", conflicts_with = "diff")]
    adiff: Option<Vec<String>>,

    /// Overpass server
    #[arg(long, value_name = "URL", default_value = DEFAULT_SERVER)]
    server: String,

    /// Nominatim server (queried when --area is used)
    #[arg(long, value_name = "URL", default_value = DEFAULT_NOMINATIM_SERVER)]
    nominatim_server: String,

    /// Construct and print query but do not send to server
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// OverpassQL query string (when omitted, query is read from STDIN)
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

fn quote(s: &str) -> String {
    format!("\"{}\"", s)
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

    if let Some(timeout) = args.timeout {
        settings.insert("timeout", timeout.to_string());
    }

    if let Some(date) = args.date {
        settings.insert("date", quote(&date));
    }

    if let Some(diff) = args.diff {
        let (left, right) = (&diff[0], &diff[1]);
        settings.insert("diff", format!("{},{}", quote(left), quote(right)));
    }

    if let Some(adiff) = args.adiff {
        let (left, right) = (&adiff[0], &adiff[1]);
        settings.insert("adiff", format!("{},{}", quote(left), quote(right)));
    }

    if let Some(area) = args.area {
        let url = format!(
            "{}/search?q={}&format=json",
            args.nominatim_server,
            urlencoding::encode(&area)
        );
        let res = ureq::get(&url).set("User-Agent", &user_agent()).call()?;
        let json: serde_json::Value = serde_json::from_str(&res.into_string()?)?;

        query = format!(
            "{}(id:{});\nmap_to_area;\n{}",
            json[0]["osm_type"].as_str().unwrap(),
            json[0]["osm_id"],
            query
        );
    }

    // add settings to start of query
    if !settings.is_empty() {
        if query.starts_with('[') {
            // TODO: could try to parse the settings from the query and merge them
            // with the ones provided via CLI flags, but for now we'll just error
            eprintln!(
                "Error: query starts with settings, which conflicts wth some of the provided command line flags"
            );
            process::exit(2);
        } else {
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
    let res = ureq::post(&endpoint)
        .set("User-Agent", &user_agent())
        .send_form(&[("data", &query)])?;

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
