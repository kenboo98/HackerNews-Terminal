pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}

impl TabsState {
    pub fn new() -> TabsState {
        TabsState {
            titles: vec![
                "Top Stories".to_string(),
                "New Stories".to_string(),
                "Best Stories".to_string(),
                "Ask Stories".to_string(),
                "Show Stories".to_string(),
                "Job Stories".to_string()
            ],
            index: 0,
        }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}