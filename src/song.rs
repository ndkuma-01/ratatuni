use std::path::{PathBuf};
use std::time::{Duration, Instant};
use id3::{Tag, TagLike};



#[derive(Hash, Debug, Clone)]
pub struct Song {
    pub title: String,
    pub path: PathBuf,
    pub thumbnail_path: PathBuf,
    pub play_position: Duration,
    pub total_duration: Duration,
    pub start_time: Option<Instant>,
}


impl ToString for Song {
    fn to_string(&self) -> String {
        format!("title: {} ", self.title)
    }

}


impl Song {
    pub fn from(title: String, path: PathBuf, thumbnail_path: PathBuf) -> Self{


        let tag = Tag::read_from_path(path.as_path().clone()).unwrap();

        Self{
            title,
            path,
            thumbnail_path,
            play_position: Duration::from_secs(0),
            total_duration: Duration::from_secs(tag.duration().unwrap() as u64),
            start_time: None
        }

    }


}