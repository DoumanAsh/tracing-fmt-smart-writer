extern crate std;

use std::io;

use tracing_subscriber::fmt::MakeWriter;
use tracing_core::{Metadata};

#[derive(Copy, Clone)]
///Stdio based writer builder.
pub struct WriterBuilder {
}

impl WriterBuilder {
    #[inline(always)]
    ///Creates new instance.
    pub const fn new() -> Self {
        Self {
        }
    }
}

impl Default for WriterBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl MakeWriter for WriterBuilder {
    type Writer = io::Stdout;

    #[inline(always)]
    fn make_writer(&self) -> Self::Writer {
        io::stdout()
    }

    #[inline(always)]
    fn make_writer_for(&self, _meta: &Metadata<'_>) -> Self::Writer {
        io::stdout()
    }
}
