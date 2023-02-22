pub use web_log::Console;
use web_log::ConsoleType;
use tracing_subscriber::fmt::MakeWriter;

use tracing_core::{Level, Metadata};

#[derive(Copy, Clone)]
///Writer using `web` console
///
///Whenever possible maps log priority to console type.
///Otherwise uses `console.info` as default
///
///###### Level map (tracing to console)
///
///- `ERROR` -> `console.error`;
///- `WARN` -> `console.warn`;
///- `INFO` -> `console.info`;
///- `DEBUG` -> `console.debug`;
///- `TRACE` -> `console.debug`;
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

impl MakeWriter<'_> for WriterBuilder {
    type Writer = Console;

    #[inline(always)]
    fn make_writer(&'_ self) -> Self::Writer {
        Console::new(ConsoleType::Info)
    }

    #[inline(always)]
    fn make_writer_for(&'_ self, meta: &Metadata<'_>) -> Self::Writer {
        let level = meta.level();
        let prio = if level == &Level::ERROR {
            ConsoleType::Error
        } else if level == &Level::WARN {
            ConsoleType::Warn
        } else if level == &Level::INFO {
            ConsoleType::Info
        } else if level == &Level::DEBUG {
            ConsoleType::Debug
        } else if level == &Level::TRACE {
            ConsoleType::Debug
        } else {
            //Generally should not happen
            //but if it happens, blame `tracing`
            ConsoleType::Info
        };

        Console::new(prio)
    }
}
