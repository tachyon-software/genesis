/// Example stolen from [here](https://github.com/seanmonstar/pretty-env-logger/blob/master/examples/log.rs)
extern crate genesis;
#[macro_use]
extern crate log;

mod nested {
    pub fn deep() {
        trace!("test trace");
    }
}

fn main() {
    genesis::init().expect("Failed to init genesis logger");
    if !log_enabled!(log::Level::Trace) {
        eprintln!("To see the full demo, try setting `RUST_LOG=basic=trace`.");
        return;
    }
    self::nested::deep();
    debug!("test debug");
    info!("test info");
    warn!("test warn");
    error!("test error");
}
