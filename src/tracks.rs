use std::path::PathBuf;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Track {
    pub path: PathBuf,
    pub filename: String,
}
