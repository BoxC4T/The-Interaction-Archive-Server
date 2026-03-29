use crate::file_handler::init_folders;

mod file_handler;

fn main() {
    println!("Hello, world!");
    init_folders().unwrap_err()
}
