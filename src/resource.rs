use std::{
    fs,
    path::{Path, PathBuf},
};

#[cfg(debug_assertions)]
pub fn get_app_dir() -> PathBuf {
    std::env::current_dir().expect("Cannot get current dir")
}

#[cfg(not(debug_assertions))]
pub fn get_app_dir() -> PathBuf {
    std::env::current_dir().expect("Cannot get current dir")
}

#[cfg(debug_assertions)]
pub fn get_resource(res_path: &str) -> Result<std::vec::Vec<u8>, std::io::Error> {
    fs::read(Path::new(&get_app_dir().join("static").join(res_path)))
}

#[cfg(not(debug_assertions))]
pub fn get_resource(res_path: &str) -> Result<std::vec::Vec<u8>, std::io::Error> {
    todo!("Get resource from zip")
}
