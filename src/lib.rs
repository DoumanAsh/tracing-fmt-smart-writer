//!Smart writer for
//![FmtSubscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/struct.Subscriber.html) in `tracing-subscriber`
//!
//!## Usage
//!
//!```
//!use tracing_fmt_smart_writer::{WriterBuilder, tracing_subscriber};
//!
//!tracing_subscriber::fmt::Subscriber::builder().with_writer(WriterBuilder::new())
//!                                              .init();
//!```
//!
//!## Platform selection
//!
//!#### Android
//!
//!Writes into logcat
//!
//!##### Level map (tracing to logcat)
//!
//!- `ERROR` -> `ERROR`;
//!- `WARN` -> `WARN`;
//!- `INFO` -> `INFO`;
//!- `DEBUG` -> `DEBUG`;
//!- `TRACE` -> `VERBOSE`;
//!
//!#### Web
//!
//!Writes using `console`
//!
//!##### Level map (tracing to console)
//!
//!- `ERROR` -> `console.error`;
//!- `WARN` -> `console.warn`;
//!- `INFO` -> `console.info`;
//!- `DEBUG` -> `console.debug`;
//!- `TRACE` -> `console.debug`;
//!
//!#### Others
//!
//!Writes into `stdout`

#![no_std]
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

pub use tracing_subscriber;

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use android::*;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))]
mod stdio;
#[cfg(all(not(target_os = "android"), not(target_arch = "wasm32")))]
pub use stdio::*;
