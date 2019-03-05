extern crate chrono;
extern crate colored;
extern crate log;
#[macro_use]
extern crate derive_builder;
use colored::{Color, Colorize};
use log::{Level, Metadata, Record};

#[derive(Debug, Clone)]
pub enum DisplayMode {
    Bars,
    Text,
}

impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode::Bars
    }
}

#[derive(Default, Builder)]
#[builder(setter(into))]
#[builder(default)]
pub struct Configuration {
    display_mode: DisplayMode,
    wrap: bool,
}

struct DefaultingLevel(Level);
impl Default for DefaultingLevel {
    fn default() -> DefaultingLevel {
        DefaultingLevel::from(Level::Trace)
    }
}

impl From<Level> for DefaultingLevel {
    fn from(lev: Level) -> DefaultingLevel {
        DefaultingLevel { 0: lev }
    }
}

fn to_color(level: Level) -> Color {
    match level {
        Level::Error => Color::Red,
        Level::Warn => Color::Yellow,
        Level::Info => Color::Green,
        Level::Debug => Color::Cyan,
        Level::Trace => Color::Magenta,
    }
}

fn to_display(level: Level) -> String {
    match level {
        Level::Error => "ERROR",
        Level::Warn => " WARN",
        Level::Info => " INFO",
        Level::Debug => "DEBUG",
        Level::Trace => "TRACE",
    }
    .color(to_color(level))
    .to_string()
}

impl DefaultingLevel {
    fn colorize(&self, what: char) -> String {
        let block = repeat_char(what, 5);
        block.color(to_color(self.0)).to_string()
    }
}

#[derive(Default)]
struct LogMechanism(DefaultingLevel, Configuration);
impl LogMechanism {
    pub fn new<T>(level: T, config: Configuration) -> LogMechanism
    where
        T: Into<DefaultingLevel>,
    {
        LogMechanism {
            0: level.into(),
            1: config,
        }
    }

    pub fn configured(conf: Configuration) -> LogMechanism {
        LogMechanism::new(DefaultingLevel::default(), conf)
    }

    pub fn level<T>(level: T) -> LogMechanism
    where
        T: Into<DefaultingLevel>,
    {
        Self::new(level.into(), Default::default())
    }
}

fn should_display_file_info(level: log::Level) -> bool {
    match level {
        Level::Trace | Level::Debug | Level::Error => true,
        _ => false,
    }
}

impl LogMechanism {
    fn display_stage(&self, erec: &Record) {
        let lev = erec.metadata().level();
        match self.1.display_mode {
            DisplayMode::Bars => print!(
                "{} ",
                DefaultingLevel::from(erec.metadata().level()).colorize('█')
            ),
            DisplayMode::Text => print!("{} ", to_display(lev)),
        }
    }

    fn time_stage(&self, _rec: &Record) {
        print!("{} ", chrono::Local::now().format("%k:%M"));
    }

    fn message_stage(&self, rec: &Record) {
        print!("{} ", rec.args())
    }

    fn file_info_stage(&self, erec: &Record) {
        if let Some(file) = erec.file() {
            if let Some(line) = erec.line() {
                let data = file.split('/').map(String::from).collect::<Vec<String>>();
                print!("({}:{}) ", data[data.len() - 1], line)
            }
        }
    }
}

fn repeat_char(ch: char, n: usize) -> String {
    std::iter::repeat_with(|| ch).take(n).collect()
}

impl log::Log for LogMechanism {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= (self.0).0
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.display_stage(record);
            self.time_stage(record);
            if should_display_file_info(record.metadata().level()) {
                self.file_info_stage(record);
            }
            self.message_stage(record);
            println!();
        }
    }

    fn flush(&self) {}
}

#[inline]
pub fn init_config(config: Configuration) -> Result<(), log::SetLoggerError> {
    log::set_boxed_logger(Box::new(LogMechanism::configured(config)))?;
    log::set_max_level(log::Level::Trace.to_level_filter());
    Ok(())
}

#[inline]
pub fn init_level(level: log::Level) -> Result<(), log::SetLoggerError> {
    log::set_boxed_logger(Box::new(LogMechanism::level(level)))?;
    log::set_max_level(log::Level::Trace.to_level_filter());
    Ok(())
}

#[inline]
pub fn init() -> Result<(), log::SetLoggerError> {
    log::set_boxed_logger(Box::new(LogMechanism::default()))?;
    log::set_max_level(log::Level::Trace.to_level_filter());
    Ok(())
}
