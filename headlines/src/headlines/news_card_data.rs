pub struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}

impl NewsCardData {
    pub fn new(title: String, desc: String, url: String) -> Self {
        Self { title, desc, url }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn desc(&self) -> &String {
        &self.desc
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}
