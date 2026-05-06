# fableflake-rs

[![Build](https://github.com/fable/fableflake-rs/workflows/Build/badge.svg)](https://github.com/fable/fableflake-rs/actions?query=workflow%3ABuild)
[![crates.io](https://img.shields.io/crates/v/fableflake.svg)](https://crates.io/crates/fableflake)
[![docs.rs](https://docs.rs/fableflake/badge.svg)](https://docs.rs/fableflake/)
[![License](https://img.shields.io/crates/l/fableflake)](LICENSE-APACHE)

A distributed unique ID generator inspired by [Twitter's Snowflake](https://blog.twitter.com/2010/announcing-snowflake).

This is a Rust implementation of the original [fable/fableflake](https://github.com/sony/sonyflake), which is written in Go.

A Fableflake ID is composed of

- 39 bits for time in units of 10 msec
- 8 bits for a sequence number
- 16 bits for a machine id

## Install

Add the following to your `Cargo.toml`:
```toml
[dependencies]
fableflake = "0.4.1"
```

## Quickstart

```rust
use fableflake::Fableflake;

let sf = Fableflake::new().unwrap();
let next_id = sf.next_id().unwrap();
println!("{}", next_id);
```

## Project Origin

This project is derived from the open-source Rust implementation by Arne Bahlo (https://github.com/bahlo/sonyflake-rs)
under the MIT license. Based on that license, this fork renames the project to `fableflake`,
adds IPv6 compatibility, updates the default epoch to `2023-07-21T00:00:00+08:00`,
and expands tests and benchmarks.

## Benchmarks

Benchmarks were run on a MacBook Pro (15-inch, 2017) with a 2,8GHz i7 and 16 GB memory.
Run them yourself with `cargo bench`.

```benchmark
test bench_decompose ... bench:       1,066 ns/iter (+/- 132)
test bench_new       ... bench:     738,129 ns/iter (+/- 318,192)
test bench_next_id   ... bench:      37,390 ns/iter (+/- 499)
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
