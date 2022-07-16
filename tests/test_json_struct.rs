#[cfg(test)]
mod test_parse_stdin {
    use assert_cmd::Command;

    #[test]
    fn test_json_no_color() {
        let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .pipe_stdin("./tests/example.json")
            .unwrap()
            .assert();

        let output = assert.get_output().stdout.clone();
        assert.success();

        let stdout = String::from_utf8(output).unwrap();
        let expected_output = concat!(
            ".alias.bar => \"baz\"\n",
            ".alias_reuse.bar => \"baz\"\n",
            ".content => \"Or we\\ncan auto\\nconvert line breaks\\nto save space\"\n",
            ".json[0] => \"rigid\"\n",
            ".json[1] => \"better for data interchange\"\n",
            ".object.array[0].null_value => null\n",
            ".object.array[1].boolean => true\n",
            ".object.array[2].integer => 1\n",
            ".object.array[3].alias => \"aliases are like variables\"\n",
            ".object.array[4].alias => \"aliases are like variables\"\n",
            ".object.key => \"value\"\n",
            ".paragraph => \"Blank lines denote\\nparagraph breaks\\n\"\n",
            ".yaml[0] => \"slim and flexible\"\n",
            ".yaml[1] => \"better for configuration\"\n",
        );

        assert_eq!(stdout, expected_output);
    }

    #[test]
    fn test_json_with_color() {
        let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["-c", "always"])
            .pipe_stdin("./tests/example.json")
            .unwrap()
            .assert();

        let output = assert.get_output().stdout.clone();
        assert.success();

        let stdout = String::from_utf8(output).unwrap();
        let expected_output = concat!(
            "\u{1b}[0m\u{1b}[34m.alias.bar\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"baz\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.alias_reuse.bar\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"baz\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.content\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"Or we\\ncan auto\\nconvert line breaks\\nto save space\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.json[0]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"rigid\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.json[1]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"better for data interchange\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.array[0].null_value\u{1b}[0m => \u{1b}[0m\u{1b}[1m\u{1b}[30mnull\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.array[1].boolean\u{1b}[0m => true\n",
            "\u{1b}[0m\u{1b}[34m.object.array[2].integer\u{1b}[0m => 1\n",
            "\u{1b}[0m\u{1b}[34m.object.array[3].alias\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"aliases are like variables\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.array[4].alias\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"aliases are like variables\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.key\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"value\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.paragraph\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"Blank lines denote\\nparagraph breaks\\n\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.yaml[0]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"slim and flexible\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.yaml[1]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"better for configuration\"\n",
            "\u{1b}[0m",
        );

        assert_eq!(stdout, expected_output);
    }

    #[test]
    fn test_yaml_no_color() {
        let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["-t", "yaml"])
            .pipe_stdin("./tests/example.yaml")
            .unwrap()
            .assert();

        let output = assert.get_output().stdout.clone();
        assert.success();

        let stdout = String::from_utf8(output).unwrap();
        let expected_output = concat!(
            ".alias.bar => \"baz\"\n",
            ".alias_reuse.bar => \"baz\"\n",
            ".content => \"Or we\\ncan auto\\nconvert line breaks\\nto save space\"\n",
            ".json[0] => \"rigid\"\n",
            ".json[1] => \"better for data interchange\"\n",
            ".object.array[0].null_value => null\n",
            ".object.array[1].boolean => true\n",
            ".object.array[2].integer => 1\n",
            ".object.array[3].alias => \"aliases are like variables\"\n",
            ".object.array[4].alias => \"aliases are like variables\"\n",
            ".object.key => \"value\"\n",
            ".paragraph => \"Blank lines denote\\nparagraph breaks\\n\"\n",
            ".yaml[0] => \"slim and flexible\"\n",
            ".yaml[1] => \"better for configuration\"\n",
        );

        assert_eq!(stdout, expected_output);
    }

    #[test]
    fn test_yaml_with_color() {
        let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["-t", "yaml", "-c", "always"])
            .pipe_stdin("./tests/example.yaml")
            .unwrap()
            .assert();

        let output = assert.get_output().stdout.clone();
        assert.success();

        let stdout = String::from_utf8(output).unwrap();
        let expected_output = concat!(
            "\u{1b}[0m\u{1b}[34m.alias.bar\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"baz\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.alias_reuse.bar\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"baz\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.content\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"Or we\\ncan auto\\nconvert line breaks\\nto save space\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.json[0]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"rigid\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.json[1]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"better for data interchange\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.array[0].null_value\u{1b}[0m => \u{1b}[0m\u{1b}[1m\u{1b}[30mnull\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.array[1].boolean\u{1b}[0m => true\n",
            "\u{1b}[0m\u{1b}[34m.object.array[2].integer\u{1b}[0m => 1\n",
            "\u{1b}[0m\u{1b}[34m.object.array[3].alias\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"aliases are like variables\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.array[4].alias\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"aliases are like variables\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.key\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"value\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.paragraph\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"Blank lines denote\\nparagraph breaks\\n\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.yaml[0]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"slim and flexible\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.yaml[1]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"better for configuration\"\n",
            "\u{1b}[0m",
        );

        assert_eq!(stdout, expected_output);
    }
}

#[cfg(test)]
mod test_parse_file {
    use assert_cmd::Command;

    #[test]
    fn test_json_no_color() {
        let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg("./tests/example.json")
            .assert();

        let output = assert.get_output().stdout.clone();
        assert.success();

        let stdout = String::from_utf8(output).unwrap();
        let expected_output = concat!(
            ".alias.bar => \"baz\"\n",
            ".alias_reuse.bar => \"baz\"\n",
            ".content => \"Or we\\ncan auto\\nconvert line breaks\\nto save space\"\n",
            ".json[0] => \"rigid\"\n",
            ".json[1] => \"better for data interchange\"\n",
            ".object.array[0].null_value => null\n",
            ".object.array[1].boolean => true\n",
            ".object.array[2].integer => 1\n",
            ".object.array[3].alias => \"aliases are like variables\"\n",
            ".object.array[4].alias => \"aliases are like variables\"\n",
            ".object.key => \"value\"\n",
            ".paragraph => \"Blank lines denote\\nparagraph breaks\\n\"\n",
            ".yaml[0] => \"slim and flexible\"\n",
            ".yaml[1] => \"better for configuration\"\n",
        );

        assert_eq!(stdout, expected_output);
    }

    #[test]
    fn test_json_with_color() {
        let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["./tests/example.json", "-c", "always"])
            .assert();

        let output = assert.get_output().stdout.clone();
        assert.success();

        let stdout = String::from_utf8(output).unwrap();
        let expected_output = concat!(
            "\u{1b}[0m\u{1b}[34m.alias.bar\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"baz\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.alias_reuse.bar\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"baz\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.content\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"Or we\\ncan auto\\nconvert line breaks\\nto save space\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.json[0]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"rigid\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.json[1]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"better for data interchange\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.array[0].null_value\u{1b}[0m => \u{1b}[0m\u{1b}[1m\u{1b}[30mnull\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.array[1].boolean\u{1b}[0m => true\n",
            "\u{1b}[0m\u{1b}[34m.object.array[2].integer\u{1b}[0m => 1\n",
            "\u{1b}[0m\u{1b}[34m.object.array[3].alias\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"aliases are like variables\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.array[4].alias\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"aliases are like variables\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.key\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"value\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.paragraph\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"Blank lines denote\\nparagraph breaks\\n\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.yaml[0]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"slim and flexible\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.yaml[1]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"better for configuration\"\n",
            "\u{1b}[0m",
        );

        assert_eq!(stdout, expected_output);
    }

    #[test]
    fn test_yaml_no_color() {
        let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg("./tests/example.yaml")
            .assert();

        let output = assert.get_output().stdout.clone();
        assert.success();

        let stdout = String::from_utf8(output).unwrap();
        let expected_output = concat!(
            ".alias.bar => \"baz\"\n",
            ".alias_reuse.bar => \"baz\"\n",
            ".content => \"Or we\\ncan auto\\nconvert line breaks\\nto save space\"\n",
            ".json[0] => \"rigid\"\n",
            ".json[1] => \"better for data interchange\"\n",
            ".object.array[0].null_value => null\n",
            ".object.array[1].boolean => true\n",
            ".object.array[2].integer => 1\n",
            ".object.array[3].alias => \"aliases are like variables\"\n",
            ".object.array[4].alias => \"aliases are like variables\"\n",
            ".object.key => \"value\"\n",
            ".paragraph => \"Blank lines denote\\nparagraph breaks\\n\"\n",
            ".yaml[0] => \"slim and flexible\"\n",
            ".yaml[1] => \"better for configuration\"\n",
        );

        assert_eq!(stdout, expected_output);
    }

    #[test]
    fn test_yaml_with_color() {
        let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["./tests/example.yaml", "-c", "always"])
            .assert();

        let output = assert.get_output().stdout.clone();
        assert.success();

        let stdout = String::from_utf8(output).unwrap();
        let expected_output = concat!(
            "\u{1b}[0m\u{1b}[34m.alias.bar\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"baz\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.alias_reuse.bar\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"baz\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.content\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"Or we\\ncan auto\\nconvert line breaks\\nto save space\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.json[0]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"rigid\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.json[1]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"better for data interchange\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.array[0].null_value\u{1b}[0m => \u{1b}[0m\u{1b}[1m\u{1b}[30mnull\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.array[1].boolean\u{1b}[0m => true\n",
            "\u{1b}[0m\u{1b}[34m.object.array[2].integer\u{1b}[0m => 1\n",
            "\u{1b}[0m\u{1b}[34m.object.array[3].alias\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"aliases are like variables\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.array[4].alias\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"aliases are like variables\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.object.key\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"value\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.paragraph\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"Blank lines denote\\nparagraph breaks\\n\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.yaml[0]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"slim and flexible\"\n",
            "\u{1b}[0m\u{1b}[0m\u{1b}[34m.yaml[1]\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"better for configuration\"\n",
            "\u{1b}[0m",
        );

        assert_eq!(stdout, expected_output);
    }
}
