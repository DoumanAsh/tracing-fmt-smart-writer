# tracing-fmt-smart-writer

[![Crates.io](https://img.shields.io/crates/v/tracing-fmt-smart-writer.svg)](https://crates.io/crates/tracing-fmt-smart-writer)
[![Documentation](https://docs.rs/tracing-fmt-smart-writer/badge.svg)](https://docs.rs/crate/tracing-fmt-smart-writer/)
[![Build](https://github.com/DoumanAsh/tracing-fmt-smart-writer/workflows/Rust/badge.svg)](https://github.com/DoumanAsh/tracing-fmt-smart-writer/actions?query=workflow%3ARust)

Smarter writer builder for tracing-subscriber's fmt module

## Usage

```rust
use tracing_fmt_smart_writer::{WriterBuilder, tracing_subscriber};

tracing_subscriber::fmt::Subscriber::builder().with_writer(WriterBuilder::new())
                                              .init();
```

## Platform selection

#### Android

Writes into logcat

##### Level map (tracing to logcat)

- `ERROR` -> `ERROR`;
- `WARN` -> `WARN`;
- `INFO` -> `INFO`;
- `DEBUG` -> `DEBUG`;
- `TRACE` -> `VERBOSE`;

#### Web

Writes using `console`

##### Level map (tracing to console)

- `ERROR` -> `console.error`;
- `WARN` -> `console.warn`;
- `INFO` -> `console.info`;
- `DEBUG` -> `console.debug`;
- `TRACE` -> `console.debug`;

#### Others

Writes into `stdout`
