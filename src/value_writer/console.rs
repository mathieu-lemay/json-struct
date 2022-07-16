use std::io::Write;
use std::str;

use serde_json::Number;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use super::{escape_str_value, ValueWriter};
use crate::error::Result;

lazy_static! {
    static ref KEY_COLOR_SPEC: ColorSpec = ColorSpec::new().set_fg(Some(Color::Blue)).clone();
    static ref STRING_VALUE_COLOR_SPEC: ColorSpec =
        ColorSpec::new().set_fg(Some(Color::Green)).clone();
    static ref NULL_VALUE_COLOR_SPEC: ColorSpec = ColorSpec::new()
        .set_fg(Some(Color::Black))
        .set_bold(true)
        .clone();
    static ref RAW_VALUE_COLOR_SPEC: ColorSpec = ColorSpec::new().set_bold(true).clone();
}

pub(super) struct ConsoleWriter<'a> {
    writer: &'a mut dyn Write,
    buffer: Buffer,
}

impl<'a> ConsoleWriter<'a> {
    pub(super) fn new(writer: &'a mut dyn Write, color_choice: ColorChoice) -> Self {
        let buffer = BufferWriter::stdout(color_choice).buffer();
        Self { writer, buffer }
    }

    fn write_value(
        &mut self,
        path: &str,
        value: &str,
        value_color_spec: Option<&ColorSpec>,
    ) -> Result<()> {
        self.buffer
            .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
        write!(&mut self.buffer, "{}", path)?;
        self.buffer.reset()?;

        write!(&mut self.buffer, " => ")?;

        if let Some(spec) = value_color_spec {
            self.buffer.set_color(spec)?;
            writeln!(&mut self.buffer, "{}", value)?;
            self.buffer.reset()?;
        } else {
            writeln!(&mut self.buffer, "{}", value)?;
        }

        let value = str::from_utf8(self.buffer.as_slice())?;
        write!(self.writer, "{}", value)?;
        self.buffer.clear();

        Ok(())
    }
}

impl<'a> ValueWriter for ConsoleWriter<'a> {
    fn write_string(&mut self, path: &str, value: &str) -> Result<()> {
        self.write_value(
            path,
            &escape_str_value(value),
            Some(&STRING_VALUE_COLOR_SPEC),
        )
    }

    fn write_number(&mut self, path: &str, value: &Number) -> Result<()> {
        self.write_value(path, &value.to_string(), None)
    }

    fn write_bool(&mut self, path: &str, value: bool) -> Result<()> {
        self.write_value(path, &value.to_string(), None)
    }

    fn write_null(&mut self, path: &str) -> Result<()> {
        self.write_value(path, "null", Some(&NULL_VALUE_COLOR_SPEC))
    }

    fn write_raw(&mut self, path: &str, value: &str) -> Result<()> {
        self.write_value(path, value, Some(&RAW_VALUE_COLOR_SPEC))
    }
}

#[cfg(test)]
mod test_console_writer {
    use serde_json::Number;
    use termcolor::ColorChoice;

    use super::{ConsoleWriter, ValueWriter};

    static FORMAT_RESET: &str = "\u{1b}[0m";
    static FORMAT_BOLD: &str = "\u{1b}[1m";
    static COLOR_BLACK: &str = "\u{1b}[30m";
    static COLOR_GREEN: &str = "\u{1b}[32m";
    static COLOR_BLUE: &str = "\u{1b}[34m";

    #[test]
    fn test_write_string_with_color() {
        let mut buffer = Vec::new();
        let mut writer = ConsoleWriter::new(&mut buffer, ColorChoice::Always);

        writer.write_string(".", "Some String").unwrap();

        let actual = std::str::from_utf8(buffer.as_slice()).unwrap();
        let expected = format!(
            "{}{}.{} => {}{}\"Some String\"\n{}",
            FORMAT_RESET, COLOR_BLUE, FORMAT_RESET, FORMAT_RESET, COLOR_GREEN, FORMAT_RESET
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_write_string_no_color() {
        let mut buffer = Vec::new();
        let mut writer = ConsoleWriter::new(&mut buffer, ColorChoice::Never);

        writer.write_string(".", "Some String").unwrap();

        let value = std::str::from_utf8(buffer.as_slice()).unwrap();
        assert_eq!(". => \"Some String\"\n", value);
    }

    #[test]
    fn test_write_number_with_color() {
        let mut buffer = Vec::new();
        let mut writer = ConsoleWriter::new(&mut buffer, ColorChoice::Always);

        writer.write_number(".", &Number::from(420)).unwrap();

        let actual = std::str::from_utf8(buffer.as_slice()).unwrap();
        let expected = format!("{}{}.{} => 420\n", FORMAT_RESET, COLOR_BLUE, FORMAT_RESET);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_write_number_no_color() {
        let mut buffer = Vec::new();
        let mut writer = ConsoleWriter::new(&mut buffer, ColorChoice::Never);

        writer.write_number(".", &Number::from(420)).unwrap();

        let value = std::str::from_utf8(buffer.as_slice()).unwrap();
        assert_eq!(". => 420\n", value);
    }

    #[test]
    fn test_write_bool_with_color() {
        let mut buffer = Vec::new();
        let mut writer = ConsoleWriter::new(&mut buffer, ColorChoice::Always);

        writer.write_bool(".", true).unwrap();

        let actual = std::str::from_utf8(buffer.as_slice()).unwrap();
        let expected = format!("{}{}.{} => true\n", FORMAT_RESET, COLOR_BLUE, FORMAT_RESET);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_write_bool_no_color() {
        let mut buffer = Vec::new();
        let mut writer = ConsoleWriter::new(&mut buffer, ColorChoice::Never);

        writer.write_bool(".", true).unwrap();

        let value = std::str::from_utf8(buffer.as_slice()).unwrap();
        assert_eq!(". => true\n", value);
    }

    #[test]
    fn test_write_null_with_color() {
        let mut buffer = Vec::new();
        let mut writer = ConsoleWriter::new(&mut buffer, ColorChoice::Always);

        writer.write_null(".").unwrap();

        let actual = std::str::from_utf8(buffer.as_slice()).unwrap();
        let expected = format!(
            "{}{}.{} => {}{}{}null\n{}",
            FORMAT_RESET,
            COLOR_BLUE,
            FORMAT_RESET,
            FORMAT_RESET,
            FORMAT_BOLD,
            COLOR_BLACK,
            FORMAT_RESET
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_write_null_no_color() {
        let mut buffer = Vec::new();
        let mut writer = ConsoleWriter::new(&mut buffer, ColorChoice::Never);

        writer.write_null(".").unwrap();

        let value = std::str::from_utf8(buffer.as_slice()).unwrap();
        assert_eq!(". => null\n", value);
    }

    #[test]
    fn test_write_raw_with_color() {
        let mut buffer = Vec::new();
        let mut writer = ConsoleWriter::new(&mut buffer, ColorChoice::Always);

        writer.write_raw(".", "Some String").unwrap();

        let actual = std::str::from_utf8(buffer.as_slice()).unwrap();
        let expected = format!(
            "{}{}.{} => {}{}Some String\n{}",
            FORMAT_RESET, COLOR_BLUE, FORMAT_RESET, FORMAT_RESET, FORMAT_BOLD, FORMAT_RESET
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_write_raw_no_color() {
        let mut buffer = Vec::new();
        let mut writer = ConsoleWriter::new(&mut buffer, ColorChoice::Never);

        writer.write_raw(".", "Some String").unwrap();

        let value = std::str::from_utf8(buffer.as_slice()).unwrap();
        assert_eq!(". => Some String\n", value);
    }
}
