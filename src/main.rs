mod file_handler;

fn main() {
    file_handler::init_folders().unwrap_err();
}
