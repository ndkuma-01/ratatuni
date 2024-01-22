



pub struct DirectoryCatalog{
    pub src_directory: String,
    pub music_directory: String,
    pub thumbnail_directory: String,
    pub database_file: String,
    pub temp_directory: String,
}

impl DirectoryCatalog{

    pub fn default() -> Self{
        let bound = homedir::get_my_home().unwrap().unwrap();
        let home_dir = bound.to_str().unwrap();
        Self {
            src_directory: (home_dir.to_owned() + "\\ratatuni").to_string(),
            music_directory: (home_dir.to_owned() + "\\ratatuni\\music").to_string(),
            thumbnail_directory: (home_dir.to_owned() + "\\ratatuni\\thumbnails").to_string(),
            database_file: (home_dir.to_owned() + "\\music_data.db").to_string(),
            temp_directory: (home_dir.to_owned() + "\\ratatuni\\temp").to_string(),
        }



    }


}
