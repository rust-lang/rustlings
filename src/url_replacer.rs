pub struct UrlReplacer <'a> {
    base_url: &'a Option<String>
}

const EN_BASE_URL: &str = "https://doc.rust-lang.org/book";

impl <'a> UrlReplacer <'a> {
    pub fn new(base_url: &'a Option<String>) -> Self {
        Self {
            base_url
        }
    }

    pub fn replace(&self, hint: &str) -> String {
        if let Some(base_url) = self.base_url {
            hint.replace(EN_BASE_URL, base_url)
        } else {
            hint.to_owned()
        }
    }
}
