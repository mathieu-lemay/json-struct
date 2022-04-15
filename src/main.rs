use std::ffi::OsStr;
use std::fs::File;
use std::io::{stdin, BufReader, Read, Write};
use std::path::Path;

use clap::{ArgEnum, Parser};
use serde_json::{Number, Value};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use crate::error::Result;

mod error;

#[derive(Debug, PartialEq, Eq)]
enum InputDataType {
    Json,
    Yaml,
}

#[derive(Debug, Clone, PartialEq, Eq, ArgEnum)]
enum CmdDataType {
    Auto,
    Json,
    Yaml,
}

#[derive(Debug, Clone, PartialEq, Eq, ArgEnum)]
enum CmdColor {
    Auto,
    Always,
    Never,
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
        _ => match get_extension_from_filename(filename) {
            Some("json") => InputDataType::Json,
            Some("yaml") => InputDataType::Yaml,
            Some("yml") => InputDataType::Yaml,
            _ => InputDataType::Json,
        },
    }
}

fn parse_input_data(filename: &str, data_type: InputDataType) -> Result<Value> {
    let rd = get_reader(filename);

    let val = match data_type {
        InputDataType::Json => serde_json::from_reader(rd)?,
        InputDataType::Yaml => serde_yaml::from_reader(rd)?,
    };

    Ok(val)
}

trait EntryWriter {
    fn write_string(&self, path: &str, value: &str) -> Result<()> {
        println!("{} => {}", path, value);

        Ok(())
    }

    fn write_number(&self, path: &str, value: &Number) -> Result<()> {
        println!("{} => {}", path, value);

        Ok(())
    }

    fn write_bool(&self, path: &str, value: bool) -> Result<()> {
        println!("{} => {}", path, value);

        Ok(())
    }

    fn write_null(&self, path: &str) -> Result<()> {
        println!("{} => null", path);

        Ok(())
    }

    fn write_raw(&self, path: &str, value: &str) -> Result<()> {
        println!("{} => {}", path, value);

        Ok(())
    }
}

struct DefaultWriter {}
impl DefaultWriter {
    fn new() -> Self {
        Self {}
    }
}
impl EntryWriter for DefaultWriter {}

struct ColoredWriter {
    writer: BufferWriter,
}

impl ColoredWriter {
    fn new(color_choice: ColorChoice) -> Self {
        let writer = BufferWriter::stdout(color_choice);
        Self { writer }
    }

    fn _write_value(
        &self,
        path: &str,
        value: &str,
        color: Option<Color>,
        bold: bool,
    ) -> Result<()> {
        let mut buffer = self.writer.buffer();

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
        write!(&mut buffer, "{}", path)?;

        buffer.reset()?;
        write!(&mut buffer, " => ")?;

        buffer.set_color(ColorSpec::new().set_fg(color).set_bold(bold))?;
        writeln!(&mut buffer, "{}", value)?;

        self.writer.print(&buffer)?;

        Ok(())
    }
}

impl EntryWriter for ColoredWriter {
    fn write_string(&self, path: &str, value: &str) -> Result<()> {
        self._write_value(path, &format!("\"{}\"", value), Some(Color::Green), false)
    }

    fn write_number(&self, path: &str, value: &Number) -> Result<()> {
        self._write_value(path, &value.to_string(), None, false)
    }

    fn write_bool(&self, path: &str, value: bool) -> Result<()> {
        self._write_value(path, &value.to_string(), None, false)
    }

    fn write_null(&self, path: &str) -> Result<()> {
        self._write_value(path, "null", Some(Color::Black), true)
    }

    fn write_raw(&self, path: &str, value: &str) -> Result<()> {
        self._write_value(path, value, None, true)
    }
}

fn get_writer(color: CmdColor) -> Box<dyn EntryWriter> {
    match color {
        CmdColor::Auto => {
            if atty::is(atty::Stream::Stdout) {
                Box::new(ColoredWriter::new(ColorChoice::Auto))
            } else {
                Box::new(DefaultWriter::new())
            }
        }
        CmdColor::Always => Box::new(ColoredWriter::new(ColorChoice::Always)),
        CmdColor::Never => Box::new(DefaultWriter::new()),
    }
}

fn escape_path_element(p: String) -> String {
    let p = p.replace('"', "\\\"");

    if p.chars().any(|c| !c.is_digit(36)) {
        format!("\"{}\"", p)
    } else {
        p
    }
}

fn print_value(path: &str, value: Value, writer: &dyn EntryWriter) -> Result<()> {
    match value {
        Value::Object(o) => {
            if !o.is_empty() {
                let prefix = format!("{}{}", path, if path.ends_with('.') { "" } else { "." });

                for (k, v) in o {
                    print_value(&format!("{}{}", prefix, escape_path_element(k)), v, writer)?
                }
            } else {
                writer.write_raw(path, "{}")?
            };

            Ok(())
        }
        Value::Array(a) => {
            if !a.is_empty() {
                for (i, v) in a.into_iter().enumerate() {
                    print_value(&format!("{}[{}]", path, i), v, writer)?
                }
            } else {
                writer.write_raw(path, "[]")?
            };

            Ok(())
        }
        Value::String(s) => writer.write_string(path, &s),
        Value::Number(n) => writer.write_number(path, &n),
        Value::Bool(b) => writer.write_bool(path, b),
        Value::Null => writer.write_null(path),
    }
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
}

fn main() -> Result<()> {
    let args = Args::parse();

    let data_type = detect_data_type(&args.file, args.data_type);
    let data = parse_input_data(&args.file, data_type)?;
    let writer = get_writer(args.color);

    print_value(".", data, &(*writer))
}

#[cfg(test)]
mod test_print_value {
    use std::sync::Mutex;

    use serde_json::{Map, Number, Value};

    use crate::error::Result;

    use super::{print_value, EntryWriter};

    struct TestWriter {
        buffer: Mutex<Vec<String>>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                buffer: Mutex::new(Vec::new()),
            }
        }
    }

    impl EntryWriter for TestWriter {
        fn write_string(&self, path: &str, value: &str) -> Result<()> {
            let value = format!("{} => String({})", path, value);
            self.buffer.lock().unwrap().push(value);

            Ok(())
        }

        fn write_number(&self, path: &str, value: &Number) -> Result<()> {
            let value = format!("{} => Number({})", path, value);
            self.buffer.lock().unwrap().push(value);

            Ok(())
        }

        fn write_bool(&self, path: &str, value: bool) -> Result<()> {
            let value = format!("{} => Bool({})", path, value);
            self.buffer.lock().unwrap().push(value);

            Ok(())
        }

        fn write_null(&self, path: &str) -> Result<()> {
            let value = format!("{} => Null()", path);
            self.buffer.lock().unwrap().push(value);

            Ok(())
        }

        fn write_raw(&self, path: &str, value: &str) -> Result<()> {
            let value = format!("{} => Raw({})", path, value);
            self.buffer.lock().unwrap().push(value);

            Ok(())
        }
    }

    #[test]
    fn test_print_object() {
        let writer = TestWriter::new();

        let mut map = Map::new();
        map.insert("foo".to_string(), Value::String("bar".to_string()));
        map.insert("baz".to_string(), Value::Null);

        print_value(".", Value::Object(map), &writer).unwrap();

        let mut values = writer.buffer.lock().unwrap();
        values.sort();

        assert_eq!(*values, vec![".baz => Null()", ".foo => String(bar)"]);
    }

    #[test]
    fn test_print_array() {
        let writer = TestWriter::new();

        let arr = vec![
            Value::String("foo".to_string()),
            Value::Number(Number::from(0)),
            Value::Bool(true),
        ];

        print_value(".", Value::Array(arr), &writer).unwrap();

        assert_eq!(
            *writer.buffer.lock().unwrap(),
            vec![
                ".[0] => String(foo)",
                ".[1] => Number(0)",
                ".[2] => Bool(true)"
            ]
        );
    }

    #[test]
    fn test_print_str() {
        let writer = TestWriter::new();

        print_value("foo", Value::String("bar".to_string()), &writer).unwrap();

        assert_eq!(*writer.buffer.lock().unwrap(), vec!["foo => String(bar)"]);
    }

    #[test]
    fn test_print_number() {
        let writer = TestWriter::new();

        print_value("foo", Value::Number(Number::from(69)), &writer).unwrap();

        assert_eq!(*writer.buffer.lock().unwrap(), vec!["foo => Number(69)"]);
    }

    #[test]
    fn test_print_bool() {
        let writer = TestWriter::new();

        print_value("foo", Value::Bool(true), &writer).unwrap();

        assert_eq!(*writer.buffer.lock().unwrap(), vec!["foo => Bool(true)"]);
    }

    #[test]
    fn test_print_null() {
        let writer = TestWriter::new();

        print_value("foo", Value::Null, &writer).unwrap();

        assert_eq!(*writer.buffer.lock().unwrap(), vec!["foo => Null()"]);
    }

    #[test]
    fn test_print_complex() {
        let writer = TestWriter::new();

        let data = r#"
        {
            "first name": "John",
            "last name": "Doe",
            "age": 43,
            "address": {
                "street": "10 Downing Street",
                "city": "London"
            },
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let value: Value = serde_json::from_str(data).unwrap();

        print_value(".", value, &writer).unwrap();

        let mut values = writer.buffer.lock().unwrap();
        values.sort();

        assert_eq!(
            *values,
            vec![
                r#"."first name" => String(John)"#,
                r#"."last name" => String(Doe)"#,
                ".address.city => String(London)",
                ".address.street => String(10 Downing Street)",
                ".age => Number(43)",
                ".phones[0] => String(+44 1234567)",
                ".phones[1] => String(+44 2345678)",
            ]
        );
    }

    #[test]
    fn test_empty_dicts_and_arrays_are_output() {
        let writer = TestWriter::new();

        let data = r#"
        {
            "address": {},
            "phones": []
        }"#;

        let value: Value = serde_json::from_str(data).unwrap();

        print_value(".", value, &writer).unwrap();

        let mut values = writer.buffer.lock().unwrap();
        values.sort();

        assert_eq!(*values, vec![".address => Raw({})", ".phones => Raw([])"]);
    }
}

#[cfg(test)]
mod test_escape_path_element {
    use super::escape_path_element;

    #[test]
    fn test_nothing_to_escape() {
        assert_eq!(escape_path_element("foo".to_string()), "foo");
    }

    #[test]
    fn test_wrap_strings_with_non_alnum_chars_in_double_quotes() {
        assert_eq!(
            escape_path_element("Mathieu Lemay [0]".to_string()),
            "\"Mathieu Lemay [0]\""
        );
    }

    #[test]
    fn test_test_escape_double_quotes() {
        assert_eq!(
            escape_path_element("Mathieu \"Uncle Matt\" Lemay".to_string()),
            r#""Mathieu \"Uncle Matt\" Lemay""#
        );
    }
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
