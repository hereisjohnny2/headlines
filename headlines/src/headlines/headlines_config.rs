use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HeadlinesConfig {
    dark_mode: bool,
}

impl HeadlinesConfig {
    pub fn dark_mode(&self) -> bool {
        self.dark_mode
    }
    
    pub fn toggle_dark_mode(&mut self) {
        self.dark_mode = !self.dark_mode
    }
}

impl Default for HeadlinesConfig {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}
