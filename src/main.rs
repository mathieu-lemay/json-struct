#[macro_use]
extern crate lazy_static;

use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, stdin, BufReader, Read};
use std::path::Path;
use std::str;

use clap::{ArgEnum, CommandFactory, Parser};
use clap_complete::{generate, Shell};
use serde_json::Value;

use error::Result;
use value_writer::{get_writer, print_value};

mod error;
mod value_writer;

#[derive(Debug, PartialEq, Eq)]
enum InputDataType {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Clone, PartialEq, Eq, ArgEnum)]
enum CmdDataType {
    Auto,
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Clone, PartialEq, Eq, ArgEnum)]
enum CmdColor {
    Auto,
    Always,
    Never,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(help = "File to read. Use '-' for stdin.", default_value = "-")]
    file: String,

    #[clap(
        short = 't',
        long = "type",
        arg_enum,
        help = "Specify input data type. Auto detect if not specified",
        default_value = "auto"
    )]
    data_type: CmdDataType,

    #[clap(
        short,
        long,
        arg_enum,
        help = "Colorize the output",
        default_value = "auto"
    )]
    color: CmdColor,

    #[clap(long, arg_enum, help = "Generate completion for a shell")]
    completion: Option<Shell>,
}

fn get_reader(filename: &str) -> Box<dyn Read> {
    if filename == "-" {
        Box::new(stdin())
    } else {
        let f = File::open(filename).expect("Error reading file");
        Box::new(BufReader::new(f))
    }
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn detect_data_type(filename: &str, data_type: CmdDataType) -> InputDataType {
    match data_type {
        CmdDataType::Json => InputDataType::Json,
        CmdDataType::Yaml => InputDataType::Yaml,
        CmdDataType::Toml => InputDataType::Toml,
        _ => match get_extension_from_filename(filename) {
            Some("json") => InputDataType::Json,
            Some("yaml") => InputDataType::Yaml,
            Some("yml") => InputDataType::Yaml,
            Some("toml") => InputDataType::Toml,
            _ => InputDataType::Json,
        },
    }
}

fn parse_input_data(filename: &str, data_type: InputDataType) -> Result<Value> {
    let mut rd = get_reader(filename);

    let val = match data_type {
        InputDataType::Json => serde_json::from_reader(rd)?,
        InputDataType::Yaml => serde_yaml::from_reader(rd)?,
        InputDataType::Toml => {
            let mut buf: String = Default::default();
            rd.read_to_string(&mut buf)?;
            toml::from_str(&buf)?
        }
    };

    Ok(val)
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(shell) = args.completion {
        generate(
            shell,
            &mut Args::command(),
            "json-struct",
            &mut io::stdout(),
        );
        return Ok(());
    }

    let data_type = detect_data_type(&args.file, args.data_type);
    let data = parse_input_data(&args.file, data_type)?;
    let mut output_writer = io::stdout();
    let mut value_writer = get_writer(&mut output_writer, args.color);

    print_value(".", data, &mut (*value_writer))
}

#[cfg(test)]
mod test_get_extension_from_filename {
    use super::get_extension_from_filename;

    #[test]
    fn test_empty_string_returns_none() {
        assert_eq!(get_extension_from_filename(""), None);
    }

    #[test]
    fn test_file_with_no_extension_returns_none() {
        assert_eq!(get_extension_from_filename("Dockerfile"), None);
    }

    #[test]
    fn test_file_with_single_extension_returns_the_extension() {
        assert_eq!(get_extension_from_filename("file.json"), Some("json"));
    }

    #[test]
    fn test_file_with_multiple_extensions_returns_the_first_extension() {
        assert_eq!(get_extension_from_filename("file.tar.gz"), Some("gz"));
    }

    #[test]
    fn test_file_with_full_path_returns_the_extension() {
        assert_eq!(
            get_extension_from_filename("/some/file/with/full/path.json"),
            Some("json")
        );
    }
}

#[cfg(test)]
mod test_detect_data_type {
    use crate::CmdDataType;

    use super::{detect_data_type, InputDataType};

    #[test]
    fn test_forcing_json_returns_json_type() {
        assert_eq!(
            detect_data_type("file.yaml", CmdDataType::Json),
            InputDataType::Json
        );
    }

    #[test]
    fn test_forcing_yaml_returns_yaml_type() {
        assert_eq!(
            detect_data_type("file.json", CmdDataType::Yaml),
            InputDataType::Yaml
        );
    }

    #[test]
    fn test_forcing_toml_returns_toml_type() {
        assert_eq!(
            detect_data_type("file.json", CmdDataType::Toml),
            InputDataType::Toml
        );
    }

    #[test]
    fn test_file_with_json_extension_returns_json_type() {
        assert_eq!(
            detect_data_type("file.json", CmdDataType::Auto),
            InputDataType::Json
        );
    }

    #[test]
    fn test_file_with_yaml_extension_returns_yaml_type() {
        assert_eq!(
            detect_data_type("file.yaml", CmdDataType::Auto),
            InputDataType::Yaml
        );
    }

    #[test]
    fn test_file_with_yml_extension_returns_yaml_type() {
        assert_eq!(
            detect_data_type("file.yml", CmdDataType::Auto),
            InputDataType::Yaml
        );
    }

    #[test]
    fn test_file_with_toml_extension_returns_toml_type() {
        assert_eq!(
            detect_data_type("file.toml", CmdDataType::Auto),
            InputDataType::Toml
        );
    }

    #[test]
    fn test_file_with_any_unknown_extension_returns_json_type() {
        assert_eq!(
            detect_data_type("file.foo", CmdDataType::Auto),
            InputDataType::Json
        );
    }

    #[test]
    fn test_file_with_no_extension_returns_json_type() {
        assert_eq!(
            detect_data_type("-", CmdDataType::Auto),
            InputDataType::Json
        );
    }
}
