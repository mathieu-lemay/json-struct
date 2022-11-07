use std::io::Write;
use std::str;

use serde_json::{Number, Value};
use termcolor::ColorChoice;

use crate::error::Result;
use crate::CmdColor;

use console::ConsoleWriter;

mod console;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub(super) trait ValueWriter {
    fn write_string(&mut self, path: &str, value: &str) -> Result<()>;
    fn write_number(&mut self, path: &str, value: &Number) -> Result<()>;
    fn write_bool(&mut self, path: &str, value: bool) -> Result<()>;
    fn write_null(&mut self, path: &str) -> Result<()>;
    fn write_raw(&mut self, path: &str, value: &str) -> Result<()>;
}

pub(super) fn get_writer(writer: &mut dyn Write, color: CmdColor) -> Box<dyn ValueWriter + '_> {
    let color_choice = match color {
        CmdColor::Auto => {
            if atty::is(atty::Stream::Stdout) {
                ColorChoice::Auto
            } else {
                ColorChoice::Never
            }
        }
        CmdColor::Always => ColorChoice::Always,
        CmdColor::Never => ColorChoice::Never,
    };

    Box::new(ConsoleWriter::new(writer, color_choice))
}

pub(super) fn print_value(path: &str, value: Value, writer: &mut dyn ValueWriter) -> Result<()> {
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

fn escape_str_value(value: &str) -> String {
    format!("\"{}\"", value.replace('\n', "\\n"))
}

fn escape_path_element(p: String) -> String {
    let p = p.replace('"', "\\\"");

    if p.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
        format!("\"{}\"", p)
    } else {
        p
    }
}

#[cfg(test)]
mod test_print_value {
    use serde_json::{Map, Number, Value};

    use super::*;

    #[test]
    fn test_print_object() {
        let mut writer = MockValueWriter::new();
        writer
            .expect_write_string()
            .with(eq(".foo"), eq("bar"))
            .times(1)
            .returning(|_, _| Ok(()));
        writer
            .expect_write_null()
            .with(eq(".baz"))
            .times(1)
            .returning(|_| Ok(()));

        let mut map = Map::new();
        map.insert("foo".to_string(), Value::String("bar".to_string()));
        map.insert("baz".to_string(), Value::Null);

        print_value(".", Value::Object(map), &mut writer).unwrap();
    }

    #[test]
    fn test_print_array() {
        let mut writer = MockValueWriter::new();
        writer
            .expect_write_string()
            .with(eq(".[0]"), eq("foo"))
            .times(1)
            .returning(|_, _| Ok(()));
        writer
            .expect_write_number()
            .with(eq(".[1]"), eq(Number::from(0)))
            .times(1)
            .returning(|_, _| Ok(()));
        writer
            .expect_write_bool()
            .with(eq(".[2]"), eq(true))
            .times(1)
            .returning(|_, _| Ok(()));

        let arr = vec![
            Value::String("foo".to_string()),
            Value::Number(Number::from(0)),
            Value::Bool(true),
        ];

        print_value(".", Value::Array(arr), &mut writer).unwrap();
    }

    #[test]
    fn test_print_str() {
        let mut writer = MockValueWriter::new();
        writer
            .expect_write_string()
            .with(eq("foo"), eq("bar"))
            .times(1)
            .returning(|_, _| Ok(()));

        print_value("foo", Value::String("bar".to_string()), &mut writer).unwrap();
    }

    #[test]
    fn test_print_number() {
        let mut writer = MockValueWriter::new();
        writer
            .expect_write_number()
            .with(eq("foo"), eq(Number::from(69)))
            .times(1)
            .returning(|_, _| Ok(()));

        print_value("foo", Value::Number(Number::from(69)), &mut writer).unwrap();
    }

    #[test]
    fn test_print_bool() {
        let mut writer = MockValueWriter::new();
        writer
            .expect_write_bool()
            .with(eq("foo"), eq(true))
            .times(1)
            .returning(|_, _| Ok(()));

        print_value("foo", Value::Bool(true), &mut writer).unwrap();
    }

    #[test]
    fn test_print_null() {
        let mut writer = MockValueWriter::new();
        writer
            .expect_write_null()
            .with(eq("foo"))
            .times(1)
            .returning(|_| Ok(()));

        print_value("foo", Value::Null, &mut writer).unwrap();
    }

    #[test]
    fn test_print_complex() {
        let mut writer = MockValueWriter::new();
        writer
            .expect_write_string()
            .with(eq(r#"."first name""#), eq("John"))
            .times(1)
            .returning(|_, _| Ok(()));
        writer
            .expect_write_string()
            .with(eq(r#"."last name""#), eq("Doe"))
            .times(1)
            .returning(|_, _| Ok(()));
        writer
            .expect_write_number()
            .with(eq(".age"), eq(Number::from(43)))
            .times(1)
            .returning(|_, _| Ok(()));
        writer
            .expect_write_string()
            .with(eq(".address.street"), eq("10 Downing Street"))
            .times(1)
            .returning(|_, _| Ok(()));
        writer
            .expect_write_string()
            .with(eq(".address.city"), eq("London"))
            .times(1)
            .returning(|_, _| Ok(()));
        writer
            .expect_write_string()
            .with(eq(".phones[0]"), eq("+44 1234567"))
            .times(1)
            .returning(|_, _| Ok(()));
        writer
            .expect_write_string()
            .with(eq(".phones[1]"), eq("+44 2345678"))
            .times(1)
            .returning(|_, _| Ok(()));

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

        print_value(".", value, &mut writer).unwrap();
    }

    #[test]
    fn test_empty_dicts_and_arrays_are_printed() {
        let mut writer = MockValueWriter::new();
        writer
            .expect_write_raw()
            .with(eq(".address"), eq("{}"))
            .times(1)
            .returning(|_, _| Ok(()));
        writer
            .expect_write_raw()
            .with(eq(".phones"), eq("[]"))
            .times(1)
            .returning(|_, _| Ok(()));

        let data = r#"
        {
            "address": {},
            "phones": []
        }"#;

        let value: Value = serde_json::from_str(data).unwrap();

        print_value(".", value, &mut writer).unwrap();

        // let mut values = writer.buffer.lock().unwrap();
        // values.sort();

        // assert_eq!(*values, vec![".address => Raw({})", ".phones => Raw([])"]);
    }
}

#[cfg(test)]
mod test_escape_path_element {
    use super::escape_path_element;

    #[test]
    fn test_nothing_to_escape() {
        assert_eq!(escape_path_element("foo".to_string()), "foo");
        assert_eq!(
            escape_path_element("key_with_underscores".to_string()),
            "key_with_underscores"
        );
    }

    #[test]
    fn test_wrap_strings_with_non_alnum_chars_in_double_quotes() {
        assert_eq!(
            escape_path_element("Mathieu Lemay [0]".to_string()),
            "\"Mathieu Lemay [0]\""
        );
        assert_eq!(
            escape_path_element("key-with-dashes".to_string()),
            "\"key-with-dashes\""
        );
    }

    #[test]
    fn test_escape_double_quotes() {
        assert_eq!(
            escape_path_element("Mathieu \"Uncle Matt\" Lemay".to_string()),
            r#""Mathieu \"Uncle Matt\" Lemay""#
        );
    }
}
