
pub struct TabState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabState<'a> {
    pub fn new() -> Self {
        TabState {
            titles: vec!["player", "downloader"],
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index.clone() + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}
