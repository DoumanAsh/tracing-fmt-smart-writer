pub use androidy_log::Writer;
use androidy_log::LogPriority;
use tracing_subscriber::fmt::MakeWriter;

use tracing_core::{Level, Metadata};

#[derive(Copy, Clone)]
///Writer using `android` logcat
///
///Whenever possible maps log priority to logcat priority.
///Otherwise uses `INFO` as default
///
///Logcat `tag` is generated from `Metadata::module_path` if present.
///Otherwise default `tag` is `Rust`
///
///## Level map (tracing to logcat)
///
///- `ERROR` -> `ERROR`;
///- `WARN` -> `WARN`;
///- `INFO` -> `INFO`;
///- `DEBUG` -> `DEBUG`;
///- `TRACE` -> `VERBOSE`;
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

impl MakeWriter for WriterBuilder {
    type Writer = Writer;

    #[inline(always)]
    fn make_writer(&self) -> Self::Writer {
        Writer::new("rust", LogPriority::INFO)
    }

    #[inline(always)]
    fn make_writer_for(&self, meta: &Metadata<'_>) -> Self::Writer {
        let level = meta.level();
        let prio = if level == &Level::ERROR {
            LogPriority::ERROR
        } else if level == &Level::WARN {
            LogPriority::WARN
        } else if level == &Level::INFO {
            LogPriority::INFO
        } else if level == &Level::DEBUG {
            LogPriority::DEBUG
        } else if level == &Level::TRACE {
            LogPriority::VERBOSE
        } else {
            //Generally should not happen
            //but if it happens, blame `tracing`
            LogPriority::INFO
        };

        match meta.module_path() {
            //Limit to root component, otherwise overflow is very much likely.
            Some(module) => match module.find(':') {
                Some(len) => Writer::new(&module[..len], prio),
                None => Writer::new(module, prio),
            },
            None => Writer::new_default(prio),
        }
    }
}
