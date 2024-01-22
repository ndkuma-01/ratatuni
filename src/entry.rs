// use std::path::PathBuf;
// pub struct Entry {
//     song_or_playlist: String,
//     name: String,
//     path: Some(PathBuf),
//     thumbnail_path: Some(PathBuf),
//     songs_in_playlist: Some(Vec<String>),
// }
// impl Entry {
//     fn new(s_or_p: String, name: String, path: Option<PathBuf>, thumbnail_path: Option<PathBuf>, songs_in_playlist: Option<Vec<String>>) -> Self{
//         if s_or_p.eq("song") {
//             Self {
//                 song_or_playlist: "song".to_string(),
//                 name,
//                 path: path.unwrap(),
//                 thumbnail_path: thumbnail_path.unwrap(),
//                 songs_in_playlist: None,
//             }
//
//
//         }else if s_or_p.eq("playlist") {
//             Self {
//                 song_or_playlist: "playlist".to_string(),
//                 name,
//                 path: None,
//                 thumbnail_path: None,
//                 songs_in_playlist: songs_in_playlist.unwrap(),
//             }
//         }else{
//             panic!("");
//         }
//     }
// }
//
//
//
