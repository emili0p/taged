use lofty::{
    config::WriteOptions,
    prelude::*,
    probe::Probe,
    tag::{ItemKey, Tag},
};
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
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
/*
TODO
i need to fix this shit

* */
// we left this temporaly unused
#[warn(dead_code)]
impl Track {
    pub fn save_metadata(&self) -> Result<(), anyhow::Error> {
        let mut tagged_file = Probe::open(&self.path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let tag = match tagged_file.primary_tag_mut() {
            Some(primary_tag) => primary_tag,
            None => {
                if let Some(first_tag) = tagged_file.first_tag_mut() {
                    first_tag
                } else {
                    let tag_type = tagged_file.primary_tag_type();
                    eprintln!("WARN: No tags found, creating a new tag of type `{tag_type:?}`");
                    tagged_file.insert_tag(Tag::new(tag_type));
                    tagged_file.primary_tag_mut().unwrap()
                }
            }
        };

        if let Some(title) = &self.title {
            tag.set_title(title.clone());
        }

        if let Some(artist) = &self.artist {
            tag.set_artist(artist.clone());
        }
        if let Some(album) = &self.album {
            tag.set_album(album.clone());
        }
        if let Some(genre) = &self.genre {
            tag.set_genre(genre.clone());
        }
        /*
                if let Some(year) = &self.year {
                    if let Ok(year_num) = year.parse::<u32>() {
                        tag.set_year(year_num);
                    }
                }
        */
        if let Some(track_num) = self.track_number {
            tag.set_track(track_num);
        }

        if let Some(disc_num) = self.disc {
            tag.set_disk(disc_num); // Usar set_disk, no set_disc
        }

        tag.save_to_path(&self.path, WriteOptions::default())
            .expect("ERROR: Failed to write the tag!");

        Ok(())
    }

    pub fn load_from_path(path: PathBuf) -> Result<Self, anyhow::Error> {
        let tagged_file = Probe::open(&path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let filename = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let mut track = Track {
            path,
            filename,
            ..Default::default()
        };

        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => tagged_file.first_tag().expect("ERROR: No tags found!"),
        };

        track.title = tag.title().map(|s| s.to_string());
        track.artist = tag.artist().map(|s| s.to_string());
        track.album = tag.album().map(|s| s.to_string());
        track.genre = tag.genre().map(|s| s.to_string());
        //  track.year = tag.year().map(|y| y.to_string());
        track.track_number = tag.track();
        track.disc = tag.disk(); // Usar disk(), no disc()

        if let Some(album_artist) = tag.get_string(ItemKey::AlbumArtist) {}

        let properties = tagged_file.properties();
        let duration = properties.duration();
        let seconds = duration.as_secs() % 60;
        let duration_display = format!("{:02}:{:02}", (duration.as_secs() - seconds) / 60, seconds);

        track.duration = Some(duration_display);
        track.format = Some(format!("{:?}", tagged_file.properties()));

        Ok(track)
    }
}
