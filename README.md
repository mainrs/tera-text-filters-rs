# tera-text-filters

[![crates.io](https://img.shields.io/crates/v/tera-text-filters.svg)](https://crates.io/crates/tera-text-filters)
[![crates.io](https://img.shields.io/crates/d/tera-text-filters)](https://crates.io/crates/tera-text-filters)
[![Documentation](https://docs.rs/tera-text-filters/badge.svg)](https://docs.rs/tera-text-filters)

> Text transformation filters for the Tera template engine.

## Usage

```rust
use tera::Tera;
use tera_text_filters::camel_case;

let mut tera = Tera::default();
tera.register_filter("camel_case", camel_case);
```

Alternatively, you can register all filters at once:

```rust
use tera::Tera;
use tera_text_filters::register_all;

let mut tera = Tera::default();
register_all(&mut tera);
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
