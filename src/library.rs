use crate::tracks::Track;
use chrono::{DateTime, Local};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::read_from_path;
use lofty::tag::{Accessor, ItemKey};
use std::{fs, io, path::Path, time::SystemTime};

pub fn load_directory(path: &Path) -> io::Result<Vec<Track>> {
    let mut tracks = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext, "mp3" | "flac" | "ogg" | "wav" | "m4a") {
                    let metadata = fs::metadata(&path).ok();

                    let last_modified = metadata
                        .as_ref()
                        .and_then(|m| m.modified().ok())
                        .and_then(|t| format_time(t));

                    let added = metadata
                        .as_ref()
                        .and_then(|m| m.created().ok())
                        .and_then(|t| format_time(t));

                    let tagged_file = read_from_path(&path).ok();

                    let (title, artist, album, genre, year, track_number, disc, duration, format) =
                        if let Some(tagged) = tagged_file {
                            let tag = tagged.primary_tag();

                            let title = tag
                                .and_then(|t| t.get_string(ItemKey::TrackTitle))
                                .map(|s| s.to_string());

                            let artist = tag
                                .and_then(|t| t.get_string(ItemKey::TrackArtist))
                                .map(|s| s.to_string());

                            let album = tag
                                .and_then(|t| t.get_string(ItemKey::AlbumTitle))
                                .map(|s| s.to_string());

                            let genre = tag
                                .and_then(|t| t.get_string(ItemKey::Genre))
                                .map(|s| s.to_string());

                            let year = tag
                                .and_then(|t| t.get_string(ItemKey::Year))
                                .map(|s| s.to_string());

                            let track_number = tag.and_then(|t| t.track());
                            let disc = tag.and_then(|t| t.disk());

                            let duration_secs = tagged.properties().duration().as_secs();
                            let duration =
                                Some(format!("{}:{:02}", duration_secs / 60, duration_secs % 60));

                            let props = tagged.properties();

                            let format = Some(format!(
                                "{}:{}:{}",
                                props.sample_rate().unwrap_or(0),
                                props.bit_depth().unwrap_or(0),
                                props.channels().unwrap_or(0)
                            ));
                            (
                                title,
                                artist,
                                album,
                                genre,
                                year,
                                track_number,
                                disc,
                                duration,
                                format,
                            )
                        } else {
                            (None, None, None, None, None, None, None, None, None)
                        };

                    tracks.push(Track {
                        path: path.clone(),
                        filename: path.file_name().unwrap().to_string_lossy().to_string(),

                        title,
                        artist,
                        album,

                        duration,
                        last_modified,
                        added,

                        genre,
                        year,

                        track_number,
                        disc,

                        format,
                    });
                }
            }
        }
    }

    Ok(tracks)
}

fn format_time(time: SystemTime) -> Option<String> {
    let datetime: DateTime<Local> = time.into();
    Some(datetime.format("%Y-%m-%d").to_string())
}
