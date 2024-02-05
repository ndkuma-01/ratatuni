# ratatuni (WIP CURRENTLY NOT DONE AND POLISHED!)
An advanced music player coded in Rust using ratatui for displaying to the terminal!





Project Idea:
A user can specify a directory that contains music and thumbnails
The thumbnails will be used to either a) grab a cololr palette for the UI or b) create an ascii/tui art of that thumbnail
If a thumbnail is not provided then a default color palette (which would be defined, or it wouldn't be necessary if the ascii route is being taken) can be used, or a default ascii image would be used

So the tui will initially check if a config file or anything exists (this could be a json file), the config file would include the music directory path (default would be windows music directory), default UI colors (depending on previous design decision), and [INSERT STUFF THAT SHOULD BE PUT IN THE CONFIG]

Next, the navigator for the songs will be comprised of the song library and using a SQLite database the various playlists and Artists

measures need to be taken so that if something were to be deleted or corrupted a proper error message would be shown to the user before anything happens (use popup messages in ratatui)

there should be a way to create playlists (which would be editing the SQLite DB) and the SQLite DB should go ahead and take care of keeping track of artists and their songs


things to do:
-create error handling popup box (similar to showMessageDialog from Java swing) 
- refactor codebase to make it easier to work in. Delete unused files, make it so that the app is a universal container for grabbing the app data and config. Maybe make a config struct that holds the colors and the directories of the database and such. Go through current code and clean it up. Use better and more efficient coding practices. Implement better error handling using custom error popup box
- figure out partial rendering so the thumbnail rendering is less expensive, potentially figure out a better default thing, maybe a gif of a vinyl being played could be cool
- refactor keybindings and mode enums
- change audio backend from rodio to kittyaudio
- implement seeking and volume support (maybe a volume slider but idk)
- figure out scroll bars and see if a gradient progress bar is possible
- figure out themes and UI
- change how the viewport for songs is displayed (think of a method that will make search queries easier)
- add the ability to search through songs and playlists and compose them ranked in the viewfinder
- make it so that the overall music folder will sort its music in chronological order (with the newest added ones at the top), which means that the date added must also be added to the database (make sure to account for different ways of ranking playlists. For example a user can create a playlist with a custom order). 
- Make it so that the queue can be randomized and such
- add a tab to create playlists
- add a settings tab that allows for choosing the color scheme
- 















