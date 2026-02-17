use std::path::PathBuf;

pub struct Track {
    pub path: PathBuf,
    pub filename: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<String>,
}
