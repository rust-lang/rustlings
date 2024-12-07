pub struct UrlReplacer {
    base_url: String
}

const EN_BASE_URL: &str = "https://doc.rust-lang.org/book";

impl UrlReplacer {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url
        }
    }

    pub fn replace(&self, hint: &str) -> String {
        hint.replace(EN_BASE_URL, &self.base_url)
    }
}
