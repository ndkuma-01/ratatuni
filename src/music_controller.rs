// use rodio::Sink;
// use tokio::time::Instant;
// use crate::song::Song;
//
// pub struct music_controller {
//     sink: Sink,
//     playing_music: Option<Song>,
//     // song_length: ,
//     // time_played: Arc::new()
//
// }
//
//
// impl music_controller {
//     pub fn play_pause(&mut self) {
//         if self.sink.is_paused() {
//             self.sink.play();
//             if let Some(music) = &mut self.playing_music {
//                 if let Some(start_time) = &mut music.start_time {
//                     *start_time = Instant::now() - music.play_position;
//                 }
//             }
//         }else{
//             self.sink.pause()
//         }
//     }
//
//
//
//
//     pub fn change_volume(&mut self, volume: f32) {
//
//     }
//
// }