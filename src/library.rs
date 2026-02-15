use crate::tracks::Track;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::read_from_path;
use lofty::tag::ItemKey;
use std::{fs, io, path::Path};

pub fn load_directory(path: &Path) -> io::Result<Vec<Track>> {
    let mut tracks = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "mp3" | "flac" | "ogg" | "wav" | "m4a") {
                    let tagged_file = read_from_path(&path).ok();

                    let (artist, album, duration) = if let Some(tagged) = tagged_file {
                        let tag = tagged.primary_tag();

                        let artist = tag
                            .and_then(|t| t.get_string(ItemKey::TrackArtist))
                            .map(|s| s.to_string());

                        let album = tag
                            .and_then(|t| t.get_string(ItemKey::AlbumTitle))
                            .map(|s| s.to_string());

                        let duration_secs = tagged.properties().duration().as_secs();

                        let duration =
                            Some(format!("{}:{:02}", duration_secs / 60, duration_secs % 60));

                        (artist, album, duration)
                    } else {
                        (None, None, None)
                    };

                    tracks.push(Track {
                        path: path.clone(),
                        filename: path.file_name().unwrap().to_string_lossy().to_string(),
                        artist,
                        album,
                        duration,
                    });
                }
            }
        }
    }

    Ok(tracks)
}
