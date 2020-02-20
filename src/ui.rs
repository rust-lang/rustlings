macro_rules! warn {
    ($fmt:literal, $ex:expr) => {{
        use console::{style, Emoji};
        let formatstr = format!($fmt, $ex);
        println!(
            "{} {}",
            style(Emoji("⚠️ ", "!")).red(),
            style(formatstr).red()
        );
    }};
}

macro_rules! success {
    ($fmt:literal, $ex:expr) => {{
        use console::{style, Emoji};
        let formatstr = format!($fmt, $ex);
        println!(
            "{} {}",
            style(Emoji("✅", "✓")).green(),
            style(formatstr).green()
        );
    }};
}
