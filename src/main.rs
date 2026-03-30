mod db;
mod file_handler;

fn main() {
    file_handler::init_folders().unwrap();

    file_handler::init_files().unwrap();

    file_handler::init_db();
}
