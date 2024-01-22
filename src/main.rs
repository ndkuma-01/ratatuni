mod app;
mod ui;
mod event;
mod database;
mod directory_maintainer;
mod song;
mod stateful_list;
mod entry;
mod music_controller;
mod music;
mod colour_maintainer;
mod
thumbnail_widget;

use std::env::home_dir;
use std::{io, thread};
use std::io::{Stdout, Write};
use std::path::{Path, PathBuf};
use std::fs;
use std::str::FromStr;
use std::time::Duration;
use anyhow::Result;
use arboard::Clipboard;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{event::{DisableMouseCapture}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::backend::CrosstermBackend;
use ratatui::{Frame, Terminal};
use ratatui::layout::{Constraint, Layout};
use ratatui::layout::Rect;
use app::App;
use ratatui::prelude::*;
use ratatui::prelude::Stylize;
use ratatui::widgets::{Block, Borders, Tabs};
use rodio::OutputStream;
use ratatuni::tab_state::TabState;
use ratatuni::input_state::{InputMode, InputState};
use crate::database::Database;
use crate::event::run_app;
use crate::ui::ui;


pub enum TabActions{
    Downloader,
    Player,
}



#[tokio::main]
async fn main() -> Result<()>{

    //Intro/Config Sequence
   let  (working_direct, first_run ) = check_direct_and_first_run();
    println!("{} \n {}", working_direct.to_str().unwrap(), first_run);
    if first_run {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let dir = PathBuf::from(home_dir().unwrap().to_str().unwrap().to_string() + "\\ratatuni");
        writeln!(handle, "\n ####Welcome to ratatuni!##### \n
        This seems to be your first time.\n\
        A directory will be created at {} \n\
        this will hold the config files, database, music, and thumbnails!", dir.to_str().unwrap()).unwrap();
        fs::create_dir(dir.as_path()).unwrap();
        fs::create_dir(PathBuf::from(dir.to_str().unwrap().to_string() + "\\music").as_path()).unwrap();
        fs::create_dir(PathBuf::from(dir.to_str().unwrap().to_string() + "\\thumbnails").as_path()).unwrap();
        fs::create_dir(PathBuf::from(dir.to_str().unwrap().to_string() + "\\temp").as_path()).unwrap();
        thread::sleep(Duration::from_millis(100));
        writeln!(handle,"done!");
    }


   enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut app = App::new(stream_handle);
    app.data_base.init();
    app.update_music_directory().expect("couldn't update properly");
    app.update_playlist_directory().expect("couldn't update properly");
    app.data_base.clean_database();
    app.clean_temp();
    app.update_entries();

    println!("{:?}", app.music_table);






    let res = run_app(&mut terminal, app).await;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}


pub fn check_direct_and_first_run() -> (PathBuf, bool) {
    let mut first_run = true;
    let mut working_direct = PathBuf::new();
    if let Some(mut directory) = home_dir() {
       working_direct = PathBuf::from(directory.to_str().unwrap().to_string() + "\\ratatuni");
        // let config_file = PathBuf::from(working_direct.to_str().unwrap().to_string() + "\\config.json");
        if working_direct.exists() //&& config_file.exists()
         {
            first_run = false;
        }
    }
    // if working_direct.eq(&PathBuf::new()) {
    //     panic!("Couldn't find Home Directory Doesn't Exist At All");
    // }
    (working_direct, first_run)
}








