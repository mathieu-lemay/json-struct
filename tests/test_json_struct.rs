#[cfg(test)]
mod test_parse_stdin {
    use assert_cmd::Command;

    #[test]
    fn test_json() {
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
    fn test_yaml() {
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
    fn test_toml() {
        let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["-t", "toml"])
            .pipe_stdin("./tests/example.toml")
            .unwrap()
            .assert();

        let output = assert.get_output().stdout.clone();
        assert.success();

        let stdout = String::from_utf8(output).unwrap();
        let expected_output = concat!(
            ".clients.data[0][0] => \"gamma\"\n",
            ".clients.data[0][1] => \"delta\"\n",
            ".clients.data[1][0] => 1\n",
            ".clients.data[1][1] => 2\n",
            ".clients.hosts[0] => \"alpha\"\n",
            ".clients.hosts[1] => \"omega\"\n",
            ".database.connection_max => 5000\n",
            ".database.enabled => true\n",
            ".database.ports[0] => 8000\n",
            ".database.ports[1] => 8001\n",
            ".database.ports[2] => 8002\n",
            ".database.server => \"192.168.1.1\"\n",
            ".owner.dob.\"$__toml_private_datetime\" => \"1979-05-27T07:32:00-08:00\"\n",
            ".owner.name => \"Tom Preston-Werner\"\n",
            ".servers.alpha.dc => \"eqdc10\"\n",
            ".servers.alpha.ip => \"10.0.0.1\"\n",
            ".servers.beta.dc => \"eqdc10\"\n",
            ".servers.beta.ip => \"10.0.0.2\"\n",
            ".title => \"TOML Example\"\n",
        );

        assert_eq!(stdout, expected_output);
    }
}

#[cfg(test)]
mod test_parse_file {
    use assert_cmd::Command;

    #[test]
    fn test_json() {
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
    fn test_yaml() {
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
    fn test_toml() {
        let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .arg("./tests/example.toml")
            .assert();

        let output = assert.get_output().stdout.clone();
        assert.success();

        let stdout = String::from_utf8(output).unwrap();
        let expected_output = concat!(
            ".clients.data[0][0] => \"gamma\"\n",
            ".clients.data[0][1] => \"delta\"\n",
            ".clients.data[1][0] => 1\n",
            ".clients.data[1][1] => 2\n",
            ".clients.hosts[0] => \"alpha\"\n",
            ".clients.hosts[1] => \"omega\"\n",
            ".database.connection_max => 5000\n",
            ".database.enabled => true\n",
            ".database.ports[0] => 8000\n",
            ".database.ports[1] => 8001\n",
            ".database.ports[2] => 8002\n",
            ".database.server => \"192.168.1.1\"\n",
            ".owner.dob.\"$__toml_private_datetime\" => \"1979-05-27T07:32:00-08:00\"\n",
            ".owner.name => \"Tom Preston-Werner\"\n",
            ".servers.alpha.dc => \"eqdc10\"\n",
            ".servers.alpha.ip => \"10.0.0.1\"\n",
            ".servers.beta.dc => \"eqdc10\"\n",
            ".servers.beta.ip => \"10.0.0.2\"\n",
            ".title => \"TOML Example\"\n",
        );

        assert_eq!(stdout, expected_output);
    }
}

mod test_color {
    use assert_cmd::Command;

    #[test]
    fn test_with_color() {
        let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["-c", "always"])
            .write_stdin("{\"foo\": \"bar\"}")
            .assert();

        let output = assert.get_output().stdout.clone();
        assert.success();

        let stdout = String::from_utf8(output).unwrap();
        let expected_output = concat!(
            "\u{1b}[0m\u{1b}[34m.foo\u{1b}[0m => \u{1b}[0m\u{1b}[32m\"bar\"\n",
            "\u{1b}[0m",
        );

        assert_eq!(stdout, expected_output);
    }
}

#[cfg(test)]
mod test_regex {
    use assert_cmd::Command;

    #[test]
    fn test_apply_regex_pattern() {
        let assert = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["-p", "alias"])
            .pipe_stdin("./tests/example.json")
            .unwrap()
            .assert();

        let output = assert.get_output().stdout.clone();
        assert.success();

        let stdout = String::from_utf8(output).unwrap();
        let expected_output = concat!(
            ".alias.bar => \"baz\"\n",
            ".alias_reuse.bar => \"baz\"\n",
            ".object.array[3].alias => \"aliases are like variables\"\n",
            ".object.array[4].alias => \"aliases are like variables\"\n",
        );

        assert_eq!(stdout, expected_output);
    }
}
