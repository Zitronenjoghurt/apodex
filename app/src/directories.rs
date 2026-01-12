use directories::ProjectDirs;
use std::path::PathBuf;

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("io.github", "zitronenjoghurt", "apodex").unwrap()
}

pub fn data_dir_path() -> PathBuf {
    project_dirs().data_dir().to_path_buf()
}

pub fn eframe_save_file_path() -> PathBuf {
    data_dir_path().join("app.ron")
}

pub fn heed_cache_dir() -> PathBuf {
    data_dir_path().join("media")
}
