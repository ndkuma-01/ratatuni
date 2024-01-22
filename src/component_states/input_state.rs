use std::env::home_dir;
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;
use id3::{Tag, TagLike, Version};
use ratatui::widgets::TableState;
use rusty_ytdl::{Author, Video, VideoDetails, VideoError, VideoInfo};
use crate::gauge_state::GaugeState;
use anyhow::{Error, Result};
use crate::{DirectoryCatalog, Song};


#[derive(Eq, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
    Downloading,
}

pub struct InputState {
    pub input: String,
    pub cursor_position: usize,
    pub input_mode: InputMode,
    pub urls_to_download_table: TableState,
    pub urls_to_download_table_data: Vec<Vec<String>>,
    pub download_index_and_length: (usize, Vec<usize>),
    pub gauge: GaugeState,
    pub progress_label: String,
}

impl InputState {
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }
    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }
    pub fn clamp_cursor(&mut self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }
    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position.clone(), new_char);
        self.move_cursor_right();
    }
    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_position.clone();
            let from_left = current_index - 1;
            let before = self.input.chars().take(from_left);
            let after = self.input.chars().skip(current_index.clone());
            self.input = before.chain(after).collect();
            self.move_cursor_left();
        }
    }


    pub fn submit_url_to_queue(&mut self) {
        self.urls_to_download_table_data.push(vec![self.input.clone(), "Dormant".to_string()]);
        self.input.clear();
        self.reset_cursor();
    }


    pub fn next(&mut self) {
        let i = match self.urls_to_download_table.selected() {
            Some(i) => {
                if self.urls_to_download_table_data.len() > 0 {
                    if i >= self.urls_to_download_table_data.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                } else { 0 }
            }
            None => 0,
        };
        self.urls_to_download_table.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.urls_to_download_table.selected() {
            Some(i) => {
                if self.urls_to_download_table_data.len() > 0 {
                    if i == 0 {
                        self.urls_to_download_table_data.len() - 1
                    } else {
                        i - 1
                    }
                } else {
                    0
                }
            }
            None => 0,
        };
        self.urls_to_download_table.select(Some(i));
    }


    pub fn grab_video(&mut self, index: usize) -> Option<Video> {
        let video = match Video::new(self.urls_to_download_table_data.get(index.clone()).unwrap().get(0).unwrap().trim().to_string()) {
            Ok(vid) => {
                vid
            }
            Err(_err) => {
                if let Some(element) = self.urls_to_download_table_data.get_mut(index.clone()).unwrap().get_mut(1) {
                    *element = "Failed".to_string();
                }
                self.download_index_and_length.0 = self.download_index_and_length.0 + 1;
                self.gauge.increase_numerator();
                return None;
            }
        };
        Some(video)
    }

    pub async fn download_thumbnail(&mut self, vid: Video) -> Result<String> {
        let mut vid_info = vid.get_info().await.unwrap();
        let (mut title, mut thumbnail_urls) = (self.sanitize_string(vid_info.video_details.title.as_str()), vid_info.video_details.thumbnails);
        let mut max_res_thumbnail = String::new();
        let mut max = u64::MIN;

        for (i, curr) in thumbnail_urls.iter().enumerate(){
            if &curr.height > &max {
                max_res_thumbnail = curr.url.clone();
                max = curr.height;
            }
        }

        let response = reqwest::get(max_res_thumbnail).await.unwrap();
        let mut file = File::create(PathBuf::from(format!("{}\\{}.jpg", PathBuf::from(home_dir().unwrap().to_str().unwrap().to_string() + "\\ratatuni\\thumbnails").to_str().unwrap(), title.trim()))).unwrap();

        let mut content = Cursor::new(response.bytes().await.unwrap());

        std::io::copy(&mut content, &mut file).unwrap();
        Ok(title)
    }


    //create metatdata
    pub fn create_metadata(&mut self, vid_details: VideoDetails, title: &String) -> Option<Tag> {
        let mut tag = Tag::new();
        tag.set_title(title);
        tag.set_duration(vid_details.length_seconds.parse().unwrap());
        tag.set_genre(vid_details.video_url);
        Some(tag)
    }

    pub async fn download_video_and_write_metadata(&mut self, video: Video, title: &String, tag: Tag
    ) -> Result<(String, Tag)>{
        let direcs = DirectoryCatalog::default();
        let (temp_direc, actual_direc) = (direcs.temp_directory, direcs.music_directory);
        let title = self.sanitize_string(title.as_str());
        let temp_path = PathBuf::from(format!("{}\\{}.mp3", temp_direc, &title));
        let actual_path = PathBuf::from(format!("{}\\{}.mp3",actual_direc, &title));
        video.download(temp_path.as_path()).await.unwrap();




        let status = Command::new("ffmpeg")
            .args(&["-hide_banner", "-loglevel", "quiet" ,"-i", temp_path.to_str().unwrap(), "-acodec", "libmp3lame", "-b:a", "192k", actual_path.to_str().unwrap()])
            .stdout( Stdio::null())
            .stdin(Stdio::from(Stdio::null()))
            .status()
            .expect("Failed to execute FFmpeg command");

        fs::remove_file(temp_path).unwrap();
        let path = actual_path;
        tag.write_to_path(path.as_path(), Version::Id3v23).unwrap();
        if let Some(element) = self.urls_to_download_table_data.get_mut(self.download_index_and_length.1
            .get(self.download_index_and_length.0.clone()).unwrap().clone()).unwrap().get_mut(0){
            *element = title.to_string().clone();

        }
        if let Some(element) = self.urls_to_download_table_data.get_mut(self.download_index_and_length.1
            .get(self.download_index_and_length.0.clone()).unwrap().clone()).unwrap().get_mut(1){
            *element = "Success".to_string();
        }
        Ok( (title, tag)  )

    }
    pub fn sanitize_string(&mut self,input: &str) -> String {
        let mut sanitized: String = input
            .chars()
            .map(|c| {
                if c.is_whitespace() {
                    ' '
                } else if c.is_alphanumeric() || c == '_' || c == '.' || c == '\'' ||  c == '$'{
                    c
                } else {
                    ' '
                }
            })
            .collect();
        // Reduce multiple underscores to a single underscore
        let mut last_char: Option<char> = None;
        let mut result = String::with_capacity(input.len());
        for c in sanitized.chars() {
            match (last_char, c) {
                (Some(' '), ' ') => {}
                (_, _) => result.push(c.clone()),
            }
            last_char = Some(c.clone());
        }
        // Replace common phrases
        result = result.replace("Official Music Video", "");
        result = result.replace("Official Audio", "");
        result = result.replace("Official Video", "");
        result = result.replace("Official Lyric Video", "");
        result = result.replace("Directed by Cole Bennett", "");
        result = result.replace("official lyric video", "");
        result = result.replace("Audio", "");
        result = result.replace("/", "|");
        result = result.replace("\\", "|");
        result.trim().to_string()
    }
}
