use clap::{App, Arg};
use serde_json::{Number, Value};
use std::fs::File;
use std::io::{stdin, BufReader, Read, Result, Write};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

fn get_reader(filename: &str) -> Box<dyn Read> {
    if filename == "-" {
        Box::new(stdin())
    } else {
        let f = File::open(filename).expect("Error reading file");
        Box::new(BufReader::new(f))
    }
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
}

fn get_writer(color: &str) -> Box<dyn EntryWriter> {
    match color {
        "always" => Box::new(ColoredWriter::new(ColorChoice::Always)),
        "auto" => {
            if atty::is(atty::Stream::Stdout) {
                Box::new(ColoredWriter::new(ColorChoice::Auto))
            } else {
                Box::new(DefaultWriter::new())
            }
        }
        "never" => Box::new(DefaultWriter::new()),
        _ => panic!("Unexpected value for color: {}", color),
    }
}

fn escape_path_element(p: String) -> String {
    let p = p.replace("\"", "\\\"");

    if p.chars().any(|c| !c.is_digit(36)) {
        format!("\"{}\"", p)
    } else {
        p
    }
}

fn print_value(path: &str, value: Value, writer: &dyn EntryWriter) -> Result<()> {
    match value {
        Value::Object(o) => {
            for (k, v) in o {
                print_value(
                    &format!(
                        "{}{}{}",
                        path,
                        if path.ends_with('.') { "" } else { "." },
                        escape_path_element(k)
                    ),
                    v,
                    writer,
                )?
            }

            Ok(())
        }
        Value::Array(a) => {
            for (i, v) in a.into_iter().enumerate() {
                print_value(&format!("{}[{}]", path, i), v, writer)?
            }

            Ok(())
        }
        Value::String(s) => writer.write_string(path, &s),
        Value::Number(n) => writer.write_number(path, &n),
        Value::Bool(b) => writer.write_bool(path, b),
        Value::Null => writer.write_null(path),
    }
}

fn main() -> Result<()> {
    let matches = App::new("JSON Struct")
        .version("0.1.0")
        .author("Mathieu Lemay <acidrain1@gmail.com>")
        .about("Shows the full struct of a json file")
        .arg(
            Arg::with_name("file")
                .help("File to read. Use '-' for stdin.")
                .required(true)
                .default_value("-")
                .index(1),
        )
        .arg(
            Arg::with_name("color")
                .short("-c")
                .long("--color")
                .help("colorize the output; <color> can be 'always' (default if omitted), 'auto', or 'never'")
                .takes_value(true)
                .default_value("auto")
                .possible_values(&["always", "auto", "never"])
        )
        .get_matches();

    let filename = matches.value_of("file").unwrap();
    let color = matches.value_of("color").unwrap();

    let writer = get_writer(color);

    let rd = get_reader(filename);
    let v: Value = serde_json::from_reader(rd)?;

    print_value(".", v, &(*writer))
}

#[cfg(test)]
mod test_print_value {
    use super::{print_value, EntryWriter};

    use serde_json::{Map, Number, Value};
    use std::io::Result;
    use std::sync::Mutex;

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
