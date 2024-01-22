use std::path::PathBuf;
use std::str::FromStr;
use image::GenericImageView;
use ratatui::Frame;
use ratatui::prelude::{Alignment, Color, Constraint, Direction, Layout, Line, Modifier, Rect, Span, Style, Stylize, Text};
use ratatui::widgets::{Block, Borders, BorderType, Cell, Gauge, HighlightSpacing, List, ListItem, Paragraph, Row, Table, Tabs};
use ratatuni::input_state::InputMode;
use ratatuni::input_state::InputState;
use crate::App;
use crate::TabActions;








pub fn ui(f: &mut Frame, app: &mut App){
    let area = f.size();
    let vert = Layout::new(Direction::Vertical,[Constraint::Length(3), Constraint::Min(0)]);
    let area_split_tab = vert.split(area).to_vec();
    let tabs_area = area_split_tab.get(0).unwrap().clone();
    let inner_area = area_split_tab.get(1).unwrap().clone();
    let block = Block::default().fg(Color::from_str("#339989").unwrap()).bg(Color::from_str("#131515").unwrap());
    f.render_widget(block, area);

    let ti = app.tabs.titles.clone().iter().map(|t| {
        let (first, rest) = t.split_at(1);
        Line::from(vec![first.fg(Color::from_str("#339989").unwrap()), rest.fg(Color::from_str("#339989").unwrap())])
    }).collect();


    let tabs = Tabs::new(ti)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.tabs.index.clone())
        .style(Style::default().fg(Color::from_str("#339989").unwrap()).bg(Color::from_str("#131515").unwrap()))
        .highlight_style(Style::default().bold().bg(Color::from_str("#B15E6C").unwrap()));
    f.render_widget(tabs, tabs_area);

    //tabs
    match app.tabs.index.clone(){
        0 => draw_player_tab(f,app,inner_area),
        1 => draw_download_tab(f,app,inner_area) ,
        _ => unreachable!(),


    }
}



pub fn draw_player_tab(f: &mut Frame, app:&mut App, area: Rect){
    let vertical = Layout::new(Direction::Vertical, [Constraint::Min(1), Constraint::Length(3)]);
    let areas = vertical.split(area).to_vec();
    let (main_area, player_area) = (*areas.get(0).unwrap(), *areas.get(1).unwrap());

    let horizontal = Layout::new(Direction::Horizontal, [Constraint::Max(40), Constraint::Min(10)]);
    let areas = horizontal.split(main_area).to_vec();
    let (list_area, albulm_area) = (*areas.get(0).unwrap(), *areas.get(1).unwrap());


    //player_area
    let text = Paragraph::new(Text::from("placeholder"));

    // CURRENTLY PLAYING --- NAME OF SONG --- PROGRESS BAR --- (mode?)
    // let horizontal_play_area =  Layout::new(Direction::Horizontal, [Constraint::Length(10), Constraint::Length(20), Constraint::Min(1), Constraint::Length(1)]);
    // let player_areas = horizontal_play_area.split(player_area).to_vec();
    // let (currently_playing_player_area, title_player_area, progress_player_area, mode_player_area) = (*player_areas.get(0).unwrap(), *player_areas.get(1).unwrap(), *player_areas.get(2).unwrap(), *player_areas.get(3).unwrap());

    // let mut player_status = Text::from("DORMANT");
    // if app.sink.empty() {
    //
    // }else if app.sink.is_paused() {
    //     player_status = Text::from("PAUSED -->");
    // }else if !app.sink.is_paused() {
    //     player_status = Text::from("PLAYING -->");
    // }
    //
    // let player_status = Paragraph::new(player_status);
    // f.render_widget(player_status, currently_playing_player_area);

    let mut title = "DORMANT";
    let mut percent =0;
    let mut label = String::new();

    if let Some(music) = &app.playing_music {
        title = music.title.as_str();
        percent = ((music.play_position.as_secs_f32()/music.total_duration.as_secs_f32()) * (100 as f32)).round() as u16;
        if percent > 100 {
            percent = 100;
        }
        let play_dur = music.play_position.as_secs();
        let total_dur = music.total_duration.as_secs();

        let (min_play, sec_play ) = (&play_dur/60, &play_dur%60);
        let ( min_total, sec_total ) = (&total_dur/60, &total_dur%60);

        if !(min_play >= min_total && sec_play >= sec_total) {

        label = format!("|| {}m {}s | {}m {}s ||",
                        play_dur/60,
                        play_dur%60,
                        total_dur/60,
                        total_dur%60,
        );

        }else {

            label = format!("|| {}m {}s | {}m {}s ||",
                            min_total,
                            sec_total,
                            min_total,
                            sec_total,
            );

        }


    }



    let gauge = Gauge::default().block(Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_type(BorderType::Rounded)
        .title(title)
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::from_str("#B15E6C").unwrap())))
        .label(label)
        .gauge_style(Style::default().fg(Color::from_str("#B15E6C").unwrap())).percent(percent);
    // f.render_widget(gauge, progress_player_area);
    f.render_widget(gauge, player_area);
    // let placeholder = Paragraph::new(Text::from("ph"));
    // f.render_widget(placeholder, mode_player_area);



















    let vertical_search_and_tree = Layout::new(Direction::Vertical, [Constraint::Length(3), Constraint::Min(1)]);
    let vec_search_and_tree = vertical_search_and_tree.split(list_area).to_vec();
    let (search_area, tree_area) = (*vec_search_and_tree.get(0).unwrap(), *vec_search_and_tree.get(1).unwrap());




    f.render_widget(text.clone().block(Block::default().borders(Borders::ALL).title("search")), search_area);


    //creating the tree stateful widget
    let mut items_vector = Vec::new();
    for n in app.tree_list.items.clone()  {
        if !n.starts_with("937798798798798") {
            items_vector.push(Span::styled(n, Style::default().fg(Color::from_str("#B15E6C").unwrap())));
        }else{
            let n_unsalted = n.replace("937798798798798", "");
            items_vector.push(Span::from(n_unsalted));
        }
    }


    let items = List::new(items_vector).block(Block::default().borders(Borders::ALL).title(app.curr_directory.clone().replace("937798798798798", ""))).highlight_style(Style::default().add_modifier(Modifier::ITALIC)).highlight_symbol(">> ");
    f.render_stateful_widget(items, tree_area, &mut app.tree_list.state);



    let mut thumbnail_path = PathBuf::new();
    if let Some(music) = &app.playing_music {
        thumbnail_path = music.thumbnail_path.clone();
        app.thumbnail_widget.img.image = (thumbnail_path.to_str().unwrap().to_string(), thumbnail_path);
    }

    // println!("made it here");
    app.thumbnail_widget.setup(albulm_area.clone());
    // println!("{} {} \n {} {}", &app.thumbnail_widget.image.width(), app.thumbnail_widget.image.height().clone(), &albulm_area.width, &albulm_area.height);
    f.render_widget(app.thumbnail_widget.clone(), albulm_area);




}


pub fn draw_download_tab(f: &mut Frame, app: &mut App, area: Rect){
    let vertical = Layout::new(Direction::Vertical, [Constraint::Length(1), Constraint::Length(3), Constraint::Length(1), Constraint::Min(1)]);
    let areas = vertical.split(area).to_vec();
    let (help_area, input_area,progress_area , messages_area) = (areas.get(0).unwrap().clone(), areas.get(1).unwrap().clone(), areas.get(2).unwrap().clone(), areas.get(3).unwrap().clone());

    let (msg, style) = match app.url_input.input_mode {
        InputMode::Normal => (vec!["Press ".into(), "q".bold(), " to exit, ".into(), "e".bold(), " to start editing.".bold(),], Style::default().add_modifier(Modifier::RAPID_BLINK)),
        InputMode::Editing => (vec!["Press ".into(), "Esc".bold(), " to stop editing, ".into(), "Enter".bold(), " to submit the URL to the Queue.".into()], Style::default()),
        InputMode::Downloading => (vec!["Wait for the download to finish".into()], Style::default()),
    };

    let text = Text::from(Line::from(msg));
    let help_msg = Paragraph::new(text).fg(Color::from_str("#339989").unwrap()).bg(Color::from_str("#131515").unwrap());
    f.render_widget(help_msg, help_area);

    let input = Paragraph::new(app.url_input.input.as_str())
        .style(match app.url_input.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::LightCyan),
            InputMode::Downloading => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Input").fg(Color::from_str("#339989").unwrap()).bg(Color::from_str("#131515").unwrap()));
    f.render_widget(input, input_area);




    let horiztonal_progress_split = Layout::new(Direction::Horizontal, [Constraint::Min(40), Constraint::Min(10)]);
    let progress_area_vec = horiztonal_progress_split.split(progress_area).to_vec();
    let (progress_label_area, progress_area) = (*progress_area_vec.get(0).unwrap(), *progress_area_vec.get(1).unwrap());

    let progress_label = Paragraph::new(app.url_input.progress_label.clone()).block(Block::default()).style(Style::default().fg(Color::from_str("#B15E6C").unwrap()).bg(Color::from_str("#131515").unwrap()));
    f.render_widget(progress_label, progress_label_area);

    let progress = Gauge::default().block(Block::default()).gauge_style(Style::default().fg(Color::from_str("#B15E6C").unwrap()).bg(Color::from_str("#123b3b").unwrap()))
        .ratio(app.url_input.gauge.get_progress()).label(Span::styled(app.url_input.gauge.get_label(), Style::new().fg(Color::from_str("#339989").unwrap()))).use_unicode(true);
    f.render_widget(progress, progress_area);




    match app.url_input.input_mode {
        InputMode::Normal => {},
        InputMode::Editing => {
            f.set_cursor(
                input_area.x + app.url_input.cursor_position.clone() as u16 + 1,
                input_area.y + 1,

            )
        }
        InputMode::Downloading => {},
    }
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::from_str("#B15E6C").unwrap());
    let header_cells=  ["urls", "status"].iter().map(|h| Cell::from(*h).style(Style::default().fg(Color::from_str("#339989").unwrap())));
    let header = Row::new(header_cells).style(normal_style).height(1).bottom_margin(1);
    let rows = app.url_input.urls_to_download_table_data.iter().map(|item|  {
        let height = item.iter().map(|content| content.chars().filter(|c| *c == '\n').count()).max().unwrap_or(0) + 1 ;

        let cells = item.iter().map(|c| {
            let mut cell = Cell::from(c.as_str());
            match c.as_str(){
                "Dormant" => {cell = cell.style(Style::default().fg(Color::Gray));},
                "Failed" =>
                    {cell = cell.style(Style::default().fg(Color::Red));},
                "Success" => {cell = cell.style(Style::default().fg(Color::LightGreen));},

                _ => {},
            }
            cell
        });
        Row::new(cells).height(height as u16).bottom_margin(1)
    } );
    let t = Table::new(rows, &[Constraint::Percentage(50), Constraint::Min(7)]).header(header).block(Block::default().borders(Borders::ALL).title("Table").fg(Color::from_str("#339989").unwrap()).bg(Color::from_str("#131515").unwrap()))
        .highlight_style(selected_style).highlight_symbol(">> ");
    f.render_stateful_widget(t, messages_area, &mut app.url_input.urls_to_download_table);

    // let messages: Vec<ListItem> = app.url_input.urls_to_download.iter().enumerate()
    //     .map(|(i,m)| {
    //        let status = app.url_input.urls_download_status.get(i).unwrap();
    //         let mut content = Line::default();
    //         match status.to_string().as_str(){
    //             "Dormant" => {
    //                 content = Line::from(vec![i.to_string().fg(Color::from_str("#339989").unwrap()).bg(Color::from_str("#131515").unwrap())
    //         ,  ": ".into(), m.to_string().fg(Color::from_str("#339989").unwrap()).bg(Color::from_str("#131515").unwrap()),
    //         "        ".into(), "        ".into(), status.to_string().gray(),
    //         ]);
    //             },
    //             "Success" => {
    //                  content = Line::from(vec![i.to_string().fg(Color::from_str("#339989").unwrap()).bg(Color::from_str("#131515").unwrap())
    //         ,  ": ".into(), m.to_string().fg(Color::from_str("#339989").unwrap()).bg(Color::from_str("#131515").unwrap()),
    //         "        ".into(), "        ".into(), status.to_string().green(),
    //         ]);
    //             },
    //             "Failed" => {
    //               content = Line::from(vec![i.to_string().fg(Color::from_str("#339989").unwrap()).bg(Color::from_str("#131515").unwrap())
    //         ,  ": ".into(), m.to_string().fg(Color::from_str("#339989").unwrap()).bg(Color::from_str("#131515").unwrap()),
    //         "        ".into(), "        ".into(), status.to_string().red(),
    //         ]);
    //             }
    //             _ => {}
    //         }
    //
    //         // let content = Line::from(Span::raw(format!("{i}: {m} \t \t {}", status.gray())));
    //         ListItem::new(content)
    //     }).collect();
    // let messages = List::new(messages).block(Block::default().borders(Borders::ALL).title("Videos to Download")).fg(Color::from_str("#339989").unwrap()).bg(Color::from_str("#131515").unwrap());
    // f.render_widget(messages, messages_area);



}



