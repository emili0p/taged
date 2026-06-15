use std::path::PathBuf;

use lofty::{
    config::WriteOptions,
    file::{AudioFile, TaggedFileExt},
    prelude::{Accessor, TagExt},
    tag::{Tag, TagType},
};

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

impl Track {
    pub fn save_metadata(&self) -> Result<(), anyhow::Error> {
        let mut tagged_file = lofty::read_from_path(&self.path)?;

        if tagged_file.primary_tag_mut().is_none() {
            let new_tag = Tag::new(TagType::Id3v2);
            tagged_file.set_primary_tag(new_tag);
        }

        let tag = tagged_file.primary_tag_mut().unwrap();

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

        if let Some(year) = &self.year {
            if let Ok(year_num) = year.parse::<u32>() {
                tag.set_year(year_num);
            }
        }

        if let Some(track_num) = self.track_number {
            tag.set_track(track_num);
        }

        if let Some(disc_num) = self.disc {
            tag.set_disc(disc_num);
        }

        // Guardar cambios
        let write_options = WriteOptions::default();
        tagged_file.save_to_path(&self.path, write_options)?;

        Ok(())
    }

    pub fn load_from_path(path: PathBuf) -> Result<Self, anyhow::Error> {
        let tagged_file = lofty::read_from_path(&path)?;
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

        if let Some(tag) = tagged_file.primary_tag() {
            track.title = tag.title().map(|s| s.to_string());
            track.artist = tag.artist().map(|s| s.to_string());
            track.album = tag.album().map(|s| s.to_string());
            track.genre = tag.genre().map(|s| s.to_string());

            track.year = tag.year().map(|y| y.to_string());
            track.track_number = tag.track();
            track.disc = tag.disc();
        }

        let properties = tagged_file.properties();
        track.duration = Some(format!("{:.2}", properties.duration().as_secs_f32()));

        let format_str = format!("{:?}", properties.container());
        track.format = Some(format_str);

        Ok(track)
    }
}

