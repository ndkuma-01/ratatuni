use std::future::Future;
use std::io::Stdout;
use std::ops::Index;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use crate::app::App;
use anyhow::Result;
use arboard::Clipboard;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use id3::{Tag, TagLike};
use crate::ui::ui;
use ratatui::widgets::{Block, Borders, Tabs};
use rusty_ytdl::Video;
use sqlite::State;
use ratatuni::DirectoryCatalog;
use ratatuni::tab_state::TabState;
use ratatuni::input_state::{InputState, InputMode};
use crate::song::Song;
use crate::stateful_list::PlayerState;

pub async fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, mut app: App<'_>) -> Result<()> {
    loop {
        if !app.sink.is_paused() {
            if let Some(curr_playing)  = &mut app.playing_music {

                curr_playing.play_position = curr_playing.start_time.unwrap().elapsed();
                terminal.draw(|f| ui(f, &mut app))?;

            }
        }
        terminal.draw(|f| ui(f, &mut app))?;

        if crossterm::event::poll(Duration::from_millis(200))? {
            if app.tabs.index == 1 {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key {
                            KeyEvent { code: KeyCode::Char('d'), modifiers: KeyModifiers::ALT, .. } => {
                                match app.url_input.input_mode {
                                    InputMode::Normal => {
                                        if (app.url_input.urls_to_download_table_data.len() == 0) {
                                            continue;
                                        }

                                        //we need to figure out how many to download and compile the indices to a single vector:
                                        let mut res = Vec::new();

                                        for (i, row) in app.url_input.urls_to_download_table_data.clone().iter().enumerate() {
                                            if row.get(1).unwrap() != "Success" && row.get(1).unwrap() != "Failed" {
                                                res.push(i);
                                            }
                                        }
                                        println!("{:?}", res);

                                        app.url_input.download_index_and_length = (0, res);

                                        // for (i, _ ) in app.url_input.urls_to_download_table_data.clone().iter().enumerate(){
                                        // println!("{}", app.url_input.urls_to_download_table_data.clone().get(i.clone()).unwrap().get(1).unwrap());
                                        // }
                                        // println!("here");app.url_input.download().await;

                                        app.url_input.gauge.set_numerator(0.0);
                                        app.url_input.gauge.set_denominator(app.url_input.download_index_and_length.1.len() as f64 * 4.0);
                                        app.url_input.input_mode = InputMode::Downloading;
                                        app.url_input.progress_label = String::from("DOWNLOADING ---");
                                    }
                                    InputMode::Editing => {}
                                    _ => todo!()
                                }
                            }
                            KeyEvent { code: KeyCode::Char('v'), modifiers: KeyModifiers::CONTROL, .. } => {
                                match app.url_input.input_mode {
                                    InputMode::Editing => {
                                        let mut clip = Clipboard::new().unwrap();
                                        for c in clip.get_text().unwrap().chars() {
                                            app.url_input.enter_char(c);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }


                        match app.url_input.input_mode {
                            InputMode::Normal => match key.code {
                                KeyCode::Char('e') => {
                                    app.url_input.progress_label = String::from("EDITING MODE ---");
                                    app.url_input.input_mode = InputMode::Editing;
                                }
                                KeyCode::Char('q') => return Ok(()),
                                KeyCode::Char(']') => app.tabs.next(),
                                KeyCode::Char('[') => app.tabs.previous(),
                                KeyCode::Down => app.url_input.next(),
                                KeyCode::Up => app.url_input.previous(),
                                _ => {}
                            },
                            InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                                KeyCode::Backspace => { app.url_input.delete_char(); }
                                KeyCode::Enter => { app.url_input.submit_url_to_queue(); }
                                KeyCode::Char(to_insert) => { app.url_input.enter_char(to_insert); }
                                KeyCode::Left => { app.url_input.move_cursor_left(); }
                                KeyCode::Right => { app.url_input.move_cursor_right(); }
                                KeyCode::Esc => {
                                    app.url_input.progress_label = String::from("VIEW MODE ---");
                                    app.url_input.input_mode = InputMode::Normal;
                                }
                                _ => {}
                            },
                            InputMode::Downloading => match key.code {
                                KeyCode::Char(']') => app.tabs.next(),
                                KeyCode::Char('[') => app.tabs.previous(),
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }

                if app.url_input.input_mode == InputMode::Downloading {


                    //downloading 0 / 8;
                    // step 1 : fetching video
                    let mut video: Video;

                    app.url_input.progress_label = format!("DOWNLOADING {}/{} --> GRABBING VIDEO", app.url_input.download_index_and_length.0, app.url_input.download_index_and_length.1.len());
                    terminal.draw(|f| ui(f, &mut app))?;
                    match app.url_input.grab_video(
                        app.url_input.download_index_and_length.1
                            .get(app.url_input.download_index_and_length.0.clone()).unwrap().clone()
                    ) {
                        None => {
                            //couldn't grab the video so it needs to fail and go to the next video
                            app.url_input.progress_label = format!("DOWNLOADING {}/{} --> FAILED", app.url_input.download_index_and_length.0, app.url_input.download_index_and_length.1.len());
                            terminal.draw(|f| ui(f, &mut app))?;
                            app.url_input.download_index_and_length.0 = app.url_input.download_index_and_length.0 + 1;
                            app.url_input.gauge.increase_numerator_by(3.0);
                            if app.url_input.download_index_and_length.0 > app.url_input.download_index_and_length.1.len() {
                                app.url_input.input_mode = InputMode::Normal;
                                app.url_input.progress_label = String::from("VIEW ---");
                                continue;
                            }
                            continue;
                        },
                        Some(vid) => {
                            app.url_input.gauge.increase_numerator_by(1.0);
                            app.url_input.progress_label = format!("DOWNLOADING {}/{} --> FOUND VIDEO", app.url_input.download_index_and_length.0, app.url_input.download_index_and_length.1.len());
                            terminal.draw(|f| ui(f, &mut app))?;
                            video = vid;
                        },
                    };
                    // app.url_input.grab_video(); // need to check and increment
                    // step 2 : downloading thumbnail
                    let mut title = String::new();
                    app.url_input.progress_label = format!("DOWNLOADING {}/{} --> GRABBING THUMBNAIL", app.url_input.download_index_and_length.0, app.url_input.download_index_and_length.1.len());
                    terminal.draw(|f| ui(f, &mut app))?;
                    match app.url_input.download_thumbnail(video.clone()).await {
                        Ok(t) => {
                            title = t;
                            app.url_input.gauge.increase_numerator_by(1.0);
                            app.url_input.progress_label = format!("DOWNLOADING {}/{} --> DOWNLOADED THUMBNAIL", app.url_input.download_index_and_length.0, app.url_input.download_index_and_length.1.len());
                            terminal.draw(|f| ui(f, &mut app))?;
                        },
                        Err(_) => {
                            app.url_input.gauge.increase_numerator_by(3.0);
                            app.url_input.progress_label = format!("DOWNLOADING {}/{} --> FAILED", app.url_input.download_index_and_length.0, app.url_input.download_index_and_length.1.len());
                            terminal.draw(|f| ui(f, &mut app))?;
                            app.url_input.download_index_and_length.0 = app.url_input.download_index_and_length.0 + 1;
                            if app.url_input.download_index_and_length.0 > app.url_input.download_index_and_length.1.len() {
                                app.url_input.input_mode = InputMode::Normal;
                                app.url_input.progress_label = String::from("VIEW ---");
                                continue;
                            }
                            continue;
                        },
                    };

                    let mut tag: Tag;
                    app.url_input.progress_label = format!("DOWNLOADING {}/{} --> GRABBING METADATA", app.url_input.download_index_and_length.0, app.url_input.download_index_and_length.1.len());
                    terminal.draw(|f| ui(f, &mut app))?;
                    let vid_info = video.get_info().await.unwrap();
                    let vid_details = vid_info.video_details;
                    match app.url_input.create_metadata(vid_details.clone(), &title) {
                        None => {
                            app.url_input.gauge.increase_numerator_by(2.0);
                            app.url_input.progress_label = format!("DOWNLOADING {}/{} --> FAILED", app.url_input.download_index_and_length.0, app.url_input.download_index_and_length.1.len());
                            terminal.draw(|f| ui(f, &mut app))?;
                            app.url_input.download_index_and_length.0 = app.url_input.download_index_and_length.0 + 1;
                            if app.url_input.download_index_and_length.0 >= app.url_input.download_index_and_length.1.len() {
                                app.url_input.input_mode = InputMode::Normal;
                                app.url_input.progress_label = String::from("VIEW ---");
                                continue;
                            }
                            continue;
                        },
                        Some(t) => {
                            app.url_input.gauge.increase_numerator_by(1.0);
                            app.url_input.progress_label = format!("DOWNLOADING {}/{} --> DOWNLOADED METADATA", app.url_input.download_index_and_length.0, app.url_input.download_index_and_length.1.len());
                            terminal.draw(|f| ui(f, &mut app))?;
                            tag = t;
                        }
                    }

                    // app.url_input.download_thumbnail();
                    // step 3: fetching metadata
                    // app.url_input.create_metadata();
                    // step 4: downloading video and metadata
                    app.url_input.progress_label = format!("DOWNLOADING {}/{} --> DOWNLOADING VIDEO", app.url_input.download_index_and_length.0, app.url_input.download_index_and_length.1.len());
                    terminal.draw(|f| ui(f, &mut app))?;
                    let (title, tag) = app.url_input.download_video_and_write_metadata(video.clone(), &title, tag).await.unwrap();
                    let direcs = DirectoryCatalog::default();
                    let path = PathBuf::from(format!("{}\\{}.mp3", direcs.music_directory, title.clone()));
                    let x = Song {
                        title: title.to_string().clone(),
                        path,
                        thumbnail_path: PathBuf::from(format!("{}\\{}.jpg", direcs.thumbnail_directory, title.clone())),
                        play_position: Duration::from_secs(0),
                        total_duration: Duration::from_secs(tag.duration().unwrap() as u64),
                        start_time: None,
                    };
                    app.music_table.insert(title, x);
                    // app.data_base.add_song_to_music(&title);
                    app.url_input.progress_label = format!("DOWNLOADING {}/{} --> DONE", app.url_input.download_index_and_length.0, app.url_input.download_index_and_length.1.len());
                    app.url_input.gauge.increase_numerator_by(1.0);
                    terminal.draw(|f| ui(f, &mut app))?;
                    app.url_input.download_index_and_length.0 = app.url_input.download_index_and_length.0 + 1;
                    if app.url_input.download_index_and_length.0 >= app.url_input.download_index_and_length.1.len() {
                        app.url_input.input_mode = InputMode::Normal;
                        app.url_input.progress_label = String::from("VIEW ---");
                    }
                    // app.url_input.download_video_and_write_metadata(); //need to increment
                }
            } else if app.tabs.index == 0 {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match app.play_state {
                            PlayerState::EditMode => { todo!(); },
                            PlayerState::NavigateMode => match key.code {
                                KeyCode::Char('[') => app.tabs.next(),
                                KeyCode::Char(']') => app.tabs.previous(),
                                KeyCode::Up => app.tree_list.previous(),
                                KeyCode::Down => app.tree_list.next(),
                                KeyCode::Char('q') => { break; },
                                KeyCode::Char(' ') => app.play_pause(),
                                KeyCode::Char('s') => app.play_next_music(),
                                KeyCode::Enter => {
                                    app.add_music_to_list();
                                    println!("{:?}", app.sink.len());
                                },

                                KeyCode::Left => {
                                    if !app.curr_directory.eq("Playlists") {
                                        let temp = app.curr_directory.clone();

                                        app.tree_list.items.clear();

                                        let query = "SELECT name, song_ids FROM playlist";
                                        let mut stmt = app.data_base.link.prepare(query).unwrap();
                                        app.curr_directory = String::from("Playlists");
                                        while let Ok(State::Row) = stmt.next() {
                                            let x = stmt.read::<String, _>("name").unwrap();
                                            app.tree_list.items.push(x.clone());
                                        }
                                        if let Some(index) = app.tree_list.items.iter().position(|x| *x == temp) {
                                            app.tree_list.state.select(Some(index));
                                        } else {
                                            app.tree_list.state.select(Some(0 as usize));
                                        }
                                    }
                                },
                                KeyCode::Right => {
                                    let mut currI = 0;
                                    match app.tree_list.state.selected() {
                                        None => { continue; },
                                        Some(x) => { currI = x; },
                                    }
                                    //grab the entry
                                    let curr_entry = app.tree_list.items.get(currI.clone()).unwrap();
                                    //must be a playlist
                                    if curr_entry.starts_with("937798798798798") {
                                        let z = app.playlist_table.get(app.tree_list.items.get(currI).unwrap().clone().as_str()).unwrap();
                                        app.curr_directory = curr_entry.clone();
                                        if z.get(0).unwrap().eq("12345ALL OF THEM") {
                                            app.tree_list.items.clear();
                                            for (x, _) in &app.music_table {
                                                app.tree_list.items.push(x.clone());
                                            }
                                        } else {
                                            app.tree_list.items.clear();
                                            for song in z {
                                                app.tree_list.items.push(app.music_table.get(song.clone().as_str()).unwrap().title.clone());
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        if app.sink.empty() {
            if app.music_queue.len() > 0 {
                app.play_next_music();
            }
        }


    }
    Ok(())
}
