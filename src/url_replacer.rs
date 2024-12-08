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

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DOMAIN: &str = "https://doc.rust-kr.org";

    #[test]
    fn non_rustbook_url() {
        let replacer = UrlReplacer {
            base_url: String::from(TEST_DOMAIN)
        };

        let hint = "\
hints (...) lines (...)
link: https://example.com/ch03-02-data-types.html";

        assert_eq!(hint, replacer.replace(hint));
    }


    #[test]
    fn replace_rustbook_url() {
        let replacer = UrlReplacer {
            base_url: String::from(TEST_DOMAIN)
        };

        let hint = "\
hints (...) lines (...)
link: https://doc.rust-lang.org/book/ch03-02-data-types.html";

        assert_eq!("\
hints (...) lines (...)
link: https://doc.rust-kr.org/ch03-02-data-types.html", replacer.replace(hint));
    }
}