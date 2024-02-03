mod command_runner;
mod config;
mod listener;

fn main() {
    let config = config::Config::load().unwrap();
    listener::start_listener(config);
}
