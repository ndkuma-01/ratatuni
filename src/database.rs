use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use sqlite::{Connection, State};
use crate::directory_maintainer::DirectoryCatalog;
use anyhow::Result;
use crate::app;
use crate::app::App;
use crate::song::Song;

pub struct Database {
    pub path: PathBuf,
    pub link: Connection,
}

impl Database{


    pub fn new(path: PathBuf) -> Self{
        let connect= Connection::open(path.as_path().clone()).unwrap();
        Self{
            path,
            link: connect,
        }
    }

    //creates tables in case they don't exist
    pub fn init(&mut self) {
        let query_music = "CREATE TABLE music (
            title TEXT,
            path TEXT,
            thumbnail_path TEXT,
            total_duration INTEGER
        )";

        let query_playlists = "CREATE TABLE playlist (
        name TEXT,
        song_ids TEXT)"; // this would be a comma separated list of names


        match self.link.execute(query_music){
            Ok(_) => {}
            Err(_) => {}
        };

        match self.link.execute(query_playlists){
            Ok(_) => {}
            Err(_) => {}
        };


        let query_init_music_playlist = format!("INSERT OR IGNORE INTO playlist (name, song_ids) SELECT '{}', '{}' WHERE NOT EXISTS (SELECT 1 FROM playlist WHERE name = '{}')", "937798798798798music", "12345ALL OF THEM,", "937798798798798music");
        match self.link.execute(query_init_music_playlist) {
            Ok(_) => {println!("executed")}
            Err(err) => {println!("{err}")}
        };



    }





    pub fn clean_database(&mut self) -> Result<()>{
        let query = "SELECT title, path FROM music";
        let mut stmt = self.link.prepare(query).unwrap();
        while let Ok(State::Row) = stmt.next() {
            let x = stmt.read::<String, _>("title").unwrap();
            let y = stmt.read::<String, _>("path").unwrap();
            if !Path::new(&y).exists() {
                let delete_query = format!("DELETE FROM music WHERE title = {}",x);
                self.link.execute(delete_query);
            }
        };
        Ok(())
    }




    pub fn add_song_to_music(&mut self, song: Song) -> Result<()>{

        let query = format!("INSERT OR IGNORE INTO music (title, path, thumbnail_path, total_duration) SELECT '{}', '{}', '{}', {} WHERE NOT EXISTS (SELECT 1 FROM music WHERE title = '{}')", &song.title, song.path.to_str().unwrap(), song.thumbnail_path.to_str().unwrap(), song.total_duration.as_secs(),&song.title);
        if let Err(err) = self.link.execute(&query){
            eprintln!("{err}");
        };
        Ok(())
    }





}