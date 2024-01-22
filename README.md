# ratatuni
An advanced music player coded in Rust using ratatui for displaying to the terminal!





Project Idea:
A user can specify a directory that contains music and thumbnails
The thumbnails will be used to either a) grab a cololr palette for the UI or b) create an ascii/tui art of that thumbnail
If a thumbnail is not provided then a default color palette (which would be defined, or it wouldn't be necessary if the ascii route is being taken) can be used, or a default ascii image would be used

So the tui will initially check if a config file or anything exists (this could be a json file), the config file would include the music directory path (default would be windows music directory), default UI colors (depending on previous design decision), and [INSERT STUFF THAT SHOULD BE PUT IN THE CONFIG]

Next, the navigator for the songs will be comprised of the song library and using a SQLite database the various playlists and Artists

measures need to be taken so that if something were to be deleted or corrupted a proper error message would be shown to the user before anything happens (use popup messages in ratatui)

there should be a way to create playlists (which would be editing the SQLite DB) and the SQLite DB should go ahead and take care of keeping track of artists and their songs



things figured out:

- how to play songs basically
- how to download videos from YouTube
- how to add tags to a mp3 file
- how to get the thumbnail of a URL link 
-  figure out how to do the ascii art stuff (or color stuff)


things need to be figured out:
- figure out how to play/pause and such using crate rodio::sink
- figure out how to use ratatui
- figure out ratatui app layout (intro window, settings window, add vimkeybindings (or something along those lines), player, a queue system, a youtube downloading section, create playlist section, and such)
- figure out how to manage songs, playlists, and artists through SQlite















