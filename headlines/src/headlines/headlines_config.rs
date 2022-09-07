pub struct HeadlinesConfig {
    pub dark_mode: bool,
}

impl HeadlinesConfig {
    pub fn new() -> Self {
        HeadlinesConfig { dark_mode: true }
    }
}
