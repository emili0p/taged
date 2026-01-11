use crate::tracks::Track;
use std::{fs, io, path::Path};

pub fn load_directory(path: &Path) -> io::Result<Vec<Track>> {
    let mut tracks = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "mp3" | "flac" | "ogg" | "wav" | "m4a") {
                    tracks.push(Track {
                        filename: path.file_name().unwrap().to_string_lossy().to_string(),
                        path: path.clone(),
                    });
                }
            }
        }
    }
    Ok(tracks)
}

