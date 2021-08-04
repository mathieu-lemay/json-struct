# JSON Struct

a simple tool to show all the paths/values in a json/yaml document to help with grep.

## Usage

Show struct of a json file:
```bash
$ cat > file.json << EOF
{
  "name": "John Doe",
  "age": 43,
  "address": {
    "street": "10 Downing Street",
    "city": "London"
  },
  "phones": [
    "+44 1234567",
    "+44 2345678"
  ]
}
EOF

$ json-struct file.json
.address.city => "London"
.address.street => "10 Downing Street"
.age => 43
.name => "John Doe"
.phones[0] => "+44 1234567"
.phones[1] => "+44 2345678"
```

Read json from stdin:
```bash
$ echo '{"foo": "bar"}' | json-struct
.foo => "bar"
```

Show struct of a yaml file:
```bash
$ cat > file.yaml << EOF
name: John Doe
age: 43
address:
  street: 10 Downing Street
  city: London
phones:
  - +44 1234567
  - +44 2345678
EOF

$ json-struct file.yaml
.address.city => "London"
.address.street => "10 Downing Street"
.age => 43
.name => "John Doe"
.phones[0] => "+44 1234567"
.phones[1] => "+44 2345678"
```

Read yaml from stdin:
```bash
$ echo 'foo: bar' | json-struct --type yaml
.foo => "bar"
```

## Install

### Install directly from github
```bash
cargo install --git 'https://github.com/mathieu-lemay/json-struct'
```
