use clap::{App, Arg};
use serde_json::{Result, Value};
use std::fs::File;
use std::io::{stdin, BufReader, Read};

fn get_reader(filename: &str) -> Box<dyn Read> {
    if filename == "-" {
        Box::new(stdin())
    } else {
        let f = File::open(filename).expect("Error reading file");
        Box::new(BufReader::new(f))
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

fn print_value(path: &str, value: Value) {
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
                )
            }
        }
        Value::Array(a) => {
            for (i, v) in a.into_iter().enumerate() {
                print_value(&format!("{}[{}]", path, i), v)
            }
        }
        Value::String(s) => {
            println!("{} => \"{}\"", path, s);
        }
        Value::Number(n) => {
            println!("{} => {}", path, n);
        }
        Value::Bool(b) => {
            println!("{} => {}", path, b);
        }
        Value::Null => {
            println!("{} => null", path);
        }
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
        .get_matches();

    let filename = matches.value_of("file").unwrap();

    let rd = get_reader(filename);
    let v: Value = serde_json::from_reader(rd)?;

    print_value(".", v);

    Ok(())
}
