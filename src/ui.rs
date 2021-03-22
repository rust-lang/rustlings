macro_rules! warn {
    ($fmt:literal, $ex:expr) => {{
        use std::env;
        use console::{style, Emoji};
        let formatstr = format!($fmt, $ex);
        if env::var("NO_EMOJI").is_ok() {
            println!(
                "{} {}",
                style("!").red(),
                style(formatstr).red()
            );
        } else {
            println!(
                "{} {}",
                style(Emoji("⚠️ ", "!")).red(),
                style(formatstr).red()
            );
        }
    }};
}

macro_rules! success {
    ($fmt:literal, $ex:expr) => {{
        use std::env;
        use console::{style, Emoji};
        let formatstr = format!($fmt, $ex);
        if env::var("NO_EMOJI").is_ok() {
            println!(
                "{} {}",
                style("✓").green(),
                style(formatstr).green()
            );
        } else {
            println!(
                "{} {}",
                style(Emoji("✅", "✓")).green(),
                style(formatstr).green()
            );
        }
    }};
}
