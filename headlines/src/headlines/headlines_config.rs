use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HeadlinesConfig {
    pub dark_mode: bool,
    pub api_key: String
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
        Self { dark_mode: true, api_key: String::new() }
    }
}
