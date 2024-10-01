# simple_rust_pgp_encryption

## usage
```
♪ cargo run
   Compiling simple_rust_pgp_encryption v0.1.0 (../simple_rust_pgp_encryption)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/simple_rust_pgp_encryption`
Public Key:
-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAr7qaPPVzeP6BpRm6w8Gn
cUliIN603uCGomwE+vHXvKc6e6epHyHpW22WWUqXqGEdlHG+MCik4qH3iV8KgxrA
...
[snip]
...
JRlWPr/HrI+DiPNxhEE0TtKseDqwigXkSgc08h4+f/GTuTOVBbDls7uZvhJ/0z7W
JqI/yU5SH8krPfARmF/c2mU+ltFfr6sRBHdrxWBo2o7HSugviAGILHL+V+Kv7I0L
RQIDAQAB
-----END PUBLIC KEY-----

Encrypted: "a0b175399422cdc80eb92d63dc852b8e6f4749a30a1bf06fe87f90fb2675bbf5b335dc0b773771992cd17a5454e1164b63804bd93bde05e732e2a331a2a41f384181988120c6ce86409439f69d32bd981e38915b62b8eb35876c3a8a9c4df8f7634bf2c34f7fd04afbe96aafcaf35a5141f66eb5ba3aae22df29e6e7ced56f0fb86a77aa046815cee9eaacebbd48e4d87a98d3f0ba66bbf6a40fdc475f662b5a34b85dd41e4e654ea1f15257dc313b1aa7dcf5a01bf9f662b056f03f79b3a0274516f68fe3f036b5720d1b3c153b3906c23f3b2fe552814c0a45ed3c42f41c638b971210091ba596a0a0e4f7a4d547ba18862a22adabffd19bda62e59ce98418"
Decrypted: Hello, world!
♪
```
