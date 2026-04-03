use crate::file_handler::Config;

mod file_handler;
mod schema;

fn main() {
    let config: Config = file_handler::load_config().unwrap();
}
