use clap::{App, Arg};
use serde_json::Value;
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
    fn write(&self, path: &str, value: &str) -> Result<()> {
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
}

impl EntryWriter for ColoredWriter {
    fn write(&self, path: &str, value: &str) -> Result<()> {
        let mut buffer = self.writer.buffer();

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
        write!(&mut buffer, "{}", path)?;

        buffer.reset()?;
        write!(&mut buffer, " => ")?;

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        writeln!(&mut buffer, "{}", value)?;

        self.writer.print(&buffer)?;

        Ok(())
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
        Value::String(s) => writer.write(path, &format!("\"{}\"", s)),
        Value::Number(n) => writer.write(path, &n.to_string()),
        Value::Bool(b) => writer.write(path, &b.to_string()),
        Value::Null => writer.write(path, "null"),
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
