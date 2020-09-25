use anyhow;
use iced::{Application, Settings};
use log::{debug, trace, LevelFilter};
use log4rs::{
    self,
    append::{
        console::{ConsoleAppender, Target},
        rolling_file::{
            policy::compound::{
                roll::delete::DeleteRoller, trigger::size::SizeTrigger, CompoundPolicy,
            },
            RollingFileAppender,
        },
    },
    config::{Appender, Config, Logger, Root},
    filter::threshold::ThresholdFilter,
    Handle,
};

mod pane;
mod settings;

use pane::RootPane;

fn setup_logger() -> anyhow::Result<Handle> {
    let log_dir = std::env::current_dir()?.join("log");
    if !log_dir.is_dir() {
        std::fs::create_dir(&log_dir)?;
    }

    let stderr = Box::new(ConsoleAppender::builder().target(Target::Stderr).build());

    // Delete a log file when the file's size has passed 1MB.
    let trigger = Box::new(SizeTrigger::new(1 << 20));
    let roll = Box::new(DeleteRoller::new());
    let policy = Box::new(CompoundPolicy::new(trigger, roll));
    let files = Box::new(RollingFileAppender::builder().build(log_dir.join("log.txt"), policy)?);

    let filter = if cfg!(debug_assertions) {
        LevelFilter::Trace
    } else {
        LevelFilter::Warn
    };

    let config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(filter)))
                .build("console", stderr),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Trace)))
                .build("file", files),
        )
        .logger(
            Logger::builder()
                .appender("file")
                .build("gameclockbone", LevelFilter::Trace),
        )
        .build(Root::builder().appender("console").build(filter))?;

    Ok(log4rs::init_config(config)?)
}

fn main() -> anyhow::Result<()> {
    let _logger = setup_logger()?;
    trace!("Loggers have been set.");

    if cfg!(debug_assertions) {
        debug!("Debug mode!");
    }

    RootPane::run(Settings::default());

    Ok(())
}
