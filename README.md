# base16384

[![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/Wybxc/base16384-rs/check.yml)](https://github.com/Wybxc/base16384-rs/actions/workflows/check.yml)
[![Crates.io](https://img.shields.io/crates/v/base16384)](https://crates.io/crates/base16384)
[![docs.rs](https://img.shields.io/docsrs/base16384)](https://docs.rs/base16384)
[![License](https://img.shields.io/github/license/Wybxc/base16384-rs)](https://github.com/Wybxc/base16384-rs/blob/main/LICENSE)


Encode binary file to printable utf16be, and vice versa.

It is a Rust reimplementation of [base16384](https://github.com/fumiama/base16384).

## Examples

```rust
use base16384::Base16384;

let data = b"12345678";
let encoded = Base16384::encode(data);
let text = String::from_utf16(&encoded).unwrap();
assert_eq!(text, "婌焳廔萷尀㴁");
```

```rust
use base16384::Base16384;

let data = "婌焳廔萷尀㴁".encode_utf16().collect::<Vec<_>>();
let decoded = Base16384::decode(&data).unwrap();
assert_eq!(decoded, b"12345678");
```
