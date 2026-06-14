use std::path::PathBuf;
#[derive(Default)]
pub struct Track {
    pub path: PathBuf,
    pub filename: String,

    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,

    pub duration: Option<String>,
    pub genre: Option<String>,

    pub year: Option<String>,
    pub track_number: Option<u32>,
    pub disc: Option<u32>,

    pub format: Option<String>,

    pub last_modified: Option<String>,
    pub added: Option<String>,
}
