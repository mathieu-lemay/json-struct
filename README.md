# JSON Struct

a simple tool to show all the paths/values in a json document to help with grep.

## Usage

Show struct of a json file:
```bash
$ cat > file.json << EOF
{
  "_id": "61057ba9c7fda48498eb9788",
  "index": 0,
  "guid": "6590e93d-fd73-4995-8175-75910fe12b51",
  "isActive": false,
  "balance": "$3,322.75",
  "picture": "http://placehold.it/32x32",
  "age": 40,
  "eyeColor": "green",
  "name": "Mccray Bates",
  "gender": "male",
  "company": "SURELOGIC",
  "email": "mccraybates@surelogic.com",
  "phone": "+1 (805) 441-2822",
  "address": "430 Malta Street, Neibert, Louisiana, 8652",
  "about": "Ex aute ex minim commodo minim irure veniam reprehenderit esse eiusmod qui irure mollit nisi. Nisi id eiusmod adipisicing in ut dolore laboris laborum ullamco aute irure adipisicing minim esse. Cupidatat ad culpa minim nulla nisi laboris incididunt velit. Voluptate quis voluptate velit pariatur do tempor.\r\n",
  "registered": "2021-03-01T12:34:49 +05:00",
  "latitude": -3.25537,
  "longitude": -123.707933,
  "tags": [
    "eu",
    "ad",
    "occaecat",
    "amet",
    "nisi",
    "in",
    "aliqua"
  ],
  "friends": [
    {
      "id": 0,
      "name": "Stark Cardenas"
    },
    {
      "id": 1,
      "name": "Mcneil Fitzgerald"
    },
    {
      "id": 2,
      "name": "Travis Foley"
    }
  ],
  "greeting": "Hello, Mccray Bates! You have 3 unread messages.",
  "favoriteFruit": "banana"
}
EOF

$ json-struct file.json
."_id" => "61057ba9c7fda48498eb9788"
.about => "Ex aute ex minim commodo minim irure veniam reprehenderit esse eiusmod qui irure mollit nisi. Nisi id eiusmod adipisicing in ut dolore laboris laborum ullamco aute irure adipisicing minim esse. Cupidatat ad culpa minim nulla nisi laboris incididunt velit. Voluptate quis voluptate velit pariatur do tempor.
"
.address => "430 Malta Street, Neibert, Louisiana, 8652"
.age => 40
.balance => ",322.75"
.company => "SURELOGIC"
.email => "mccraybates@surelogic.com"
.eyeColor => "green"
.favoriteFruit => "banana"
.friends[0].id => 0
.friends[0].name => "Stark Cardenas"
.friends[1].id => 1
.friends[1].name => "Mcneil Fitzgerald"
.friends[2].id => 2
.friends[2].name => "Travis Foley"
.gender => "male"
.greeting => "Hello, Mccray Bates! You have 3 unread messages."
.guid => "6590e93d-fd73-4995-8175-75910fe12b51"
.index => 0
.isActive => false
.latitude => -3.25537
.longitude => -123.707933
.name => "Mccray Bates"
.phone => "+1 (805) 441-2822"
.picture => "http://placehold.it/32x32"
.registered => "2021-03-01T12:34:49 +05:00"
.tags[0] => "eu"
.tags[1] => "ad"
.tags[2] => "occaecat"
.tags[3] => "amet"
.tags[4] => "nisi"
.tags[5] => "in"
.tags[6] => "aliqua"
```

Read from stdin:
```bash
$ echo '{"foo": "bar"}' | json-struct
.foo => "bar"
```

## Install

### Install directly from github
```bash
cargo install --git 'https://github.com/mathieu-lemay/json-struct'
```
