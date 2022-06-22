use super::components::FileError;

mod file_handler;

fn fetch_file(path: &str) {
    
}

fn file_exists(path: &str) -> bool {
    let file = Path::new(path);
    path.exists() && path.is_file()
}