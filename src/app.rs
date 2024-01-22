use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::time::{Duration, Instant};
use ratatui::widgets::TableState;
use ratatuni::gauge_state::GaugeState;
use ratatuni::input_state::{InputMode, InputState};
use ratatuni::tab_state::TabState;
use crate::database::Database;
use crate::directory_maintainer::DirectoryCatalog;
use crate::song::Song;
use crate::TabActions;
use anyhow::Result;
use id3::{Tag, TagLike};
use ratatui::layout::Rect;
use rodio::{Decoder, OutputStream, OutputStreamHandle, PlayError, Sink};
use sqlite::State;
use tokio::task::yield_now;
use crate::stateful_list::StatefulList;
use crate::stateful_list::PlayerState;
use crate::thumbnail_widget::ThumbnailWidget;

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabState<'a>,
    pub should_quit: bool,
    pub url_input: InputState,
    pub input_state: TabActions,
    pub directories: DirectoryCatalog,
    pub data_base: Database,
    pub music_table: HashMap<String, Song>,
    pub playlist_table: HashMap<String, Vec<String>>,
    pub tree_list: StatefulList<String>,
    pub play_state: PlayerState,
    pub curr_directory: String,
    pub sink: Sink,
    pub stream_handle: OutputStreamHandle,
    pub playing_music: Option<Song>,
    pub music_queue: Vec<Song>,
    pub thumbnail_widget: ThumbnailWidget,
}

impl<'a> App<'a> {
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        let bound = homedir::get_my_home().unwrap().unwrap();
        let direc = DirectoryCatalog::default();
        let home_dir = bound.to_str().unwrap();
        let mut playlist_hashmap = HashMap::new();
        // let mut vec = String::from("12345ALL OF THEM,").split(",").map(|s| s.to_string()).collect();
        // playlist_hashmap.insert(String::from("937798798798798music"), vec);

        let mut queue =Sink::try_new(&stream_handle).unwrap();



        App {
            title: "ratatuni",
            tabs: TabState::new(),
            should_quit: false,
            url_input: InputState {
                input: "".to_string(),
                cursor_position: 0,
                input_mode: InputMode::Normal,
                urls_to_download_table: TableState::default(),
                urls_to_download_table_data: Vec::new(),
                download_index_and_length: (0, Vec::new()),
                gauge: GaugeState{numer: 0.0 , denom: 1.0},
                progress_label: "VIEW ---".to_string(),
            },
            input_state: TabActions::Player,
            directories:DirectoryCatalog::default(),
            data_base: Database::new(PathBuf::from(direc.database_file)),
            music_table: HashMap::new(),
            playlist_table: playlist_hashmap,
            tree_list: StatefulList::new(),
            play_state: PlayerState::NavigateMode,
            curr_directory: "Playlists".to_string(),
            sink: queue,
            stream_handle,
            playing_music: None,
            music_queue: Vec::new(),
            thumbnail_widget: (ThumbnailWidget::new (PathBuf::from(direc.thumbnail_directory))),
        }
    }

    pub fn play_pause(&mut self) {
        if self.sink.is_paused() {
            self.sink.play();
            if let Some(music) = &mut self.playing_music {
                if let Some(start_time) = &mut music.start_time {
                    // println!("start time before: {:?}", start_time.clone());
                    *start_time = Instant::now() - music.play_position;
                    // println!("start time after: {:?}", start_time.clone());
                }
            }
        }else{
            self.sink.pause();
        }
    }

    pub fn new_sink(&mut self) -> Result<(), PlayError>{
        self.sink = Sink::try_new(&self.stream_handle)?;
        Ok(())

    }

    pub fn play_next_music(&mut self) {
        if !self.sink.empty() {
            // println!("it was empty D:");
            self.new_sink().unwrap();
        }
        if self.music_queue.len() > 0 {

            let curr_song = self.music_queue.get_mut(0).unwrap();
            let file = File::open(curr_song.path.as_path().clone()).unwrap();
            let source = match Decoder::new_mp3(file) {
                Ok(decoded) => decoded,
                Err(err) => panic!("The error is: {:?}", err),
            };
            self.sink.append(source);
            let mut music = self.music_queue.remove(0);
            music.start_time = Some(Instant::now());
            self.playing_music = Some(music);
            self.thumbnail_widget.last_size = Rect::default();
        }else {
            self.playing_music = None;
            self.thumbnail_widget.last_size = Rect::default();

        }
    }

    pub fn add_music_to_list(&mut self ) {
        let currIndex = match self.tree_list.state.selected() {
            None => { return; }
            Some(i) => { i }
        };
        // println!("found index: {}", &currIndex);
        // println!("is paused: {}", self.sink.is_paused());
        let string = self.tree_list.items.get(currIndex).unwrap();
        if string.starts_with("937798798798798") {
            //adding all of the songs in a playlist
            let z = self.playlist_table.get(string).unwrap();
            if z.get(0).unwrap().eq("12345ALL OF THEM") {
                //if this is the all music playlist we need to add everything differently
                for (_, y) in self.music_table.iter() {
                    self.music_queue.push(y.clone());
                }
            } else {
                for song in z {
                    let y = self.music_table.get(song).unwrap().clone();
                    self.music_queue.push(y);
                }
            }
        } else {
            //adding only a single song
            self.music_queue.push(self.music_table.get(string.clone().as_str()).unwrap().clone());
        }
    }











    pub fn update_music_directory(&mut self) -> Result<()>{
        let d =  DirectoryCatalog::default();
        let files = std::fs::read_dir(PathBuf::from(d.music_directory).as_path()).unwrap();
        for f in files {
            let dir_entry = f.unwrap();
            let bind = dir_entry.file_name();
            let mut name = bind.to_str().unwrap().replace(".mp3", "");
            //check for duplicates
            let bind = dir_entry.path();
            let music_path = bind.to_str().unwrap();
            let thumbnail_path = format!("{}\\{}.jpg", &d.thumbnail_directory, name.to_owned());
            let tag = Tag::read_from_path(bind.as_path().clone()).unwrap();
            let song = Song{
                title:name.clone(),
                path:bind.clone(),
                thumbnail_path: PathBuf::from(thumbnail_path.clone()),
                play_position: Duration::from_secs(0),
                total_duration:  Duration::from_secs(tag.duration().unwrap() as u64),
                start_time: None };
            let duration = song.total_duration.as_secs().clone();
            self.music_table.insert(name.clone(), song);
            let query = format!("INSERT OR IGNORE INTO music (title, path, thumbnail_path, total_duration) SELECT '{}', '{}', '{}', {} WHERE NOT EXISTS (SELECT 1 FROM music WHERE title = '{}')", name, music_path, thumbnail_path,duration,name);
            if let Err(err) = self.data_base.link.execute(&query){
                eprintln!("{err}");
            }
        }

        Ok(())
    }

    pub fn update_playlist_directory(&mut self) -> Result<()>{

            let query = "SELECT name, song_ids FROM playlist";
            let mut stmt = self.data_base.link.prepare(query).unwrap();
            while let Ok(State::Row) = stmt.next() {
                let x = stmt.read::<String, _>("name").unwrap();
                let y:String = stmt.read::<String, _>("song_ids").unwrap();
                let mut res = Vec::new();
                for song_name in y.split(","){
                    res.push(song_name.to_string());
                }
                self.playlist_table.insert(x, res);
            };
        Ok(())
    }

    pub fn update_entries(&mut self) -> Result<()>{
        for (x,_) in &self.playlist_table {
            self.tree_list.items.push(x.clone());
        }
        Ok(())

    }


    pub fn clean_temp(&mut self) -> Result<()>{
        let temp_direc = DirectoryCatalog::default().temp_directory;
        let entries = fs::read_dir(temp_direc)?;
        for entry in entries {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.is_file()  {
                fs::remove_file(file_path)?;
            }
        }
        Ok(())
    }









    // pub fn create_music_table(&mut self) -> Result<()>{
    //
    //
    // }
    // pub fn create_playlist_table(&mut self) -> Result<()>  {
    //
    //
    //
    //
    // }
}