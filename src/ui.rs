macro_rules! warn {
    ($fmt:literal, $ex:expr) => {{
        use std::env;
        use console::{style, Emoji};
        let formatstr = format!($fmt, $ex);
        match env::var("NO_EMOJI").is_ok() {
            true => {
                println!(
                    "{} {}",
                    style("!").red(),
                    style(formatstr).red()
                );
            },
            false => {
                println!(
                    "{} {}",
                    style(Emoji("⚠️ ", "!")).red(),
                    style(formatstr).red()
                );
            }
        }
    }};
}

macro_rules! success {
    ($fmt:literal, $ex:expr) => {{
        use std::env;
        use console::{style, Emoji};
        let formatstr = format!($fmt, $ex);
        match env::var("NO_EMOJI").is_ok() {
            true => {
                println!(
                    "{} {}",
                    style("✓").green(),
                    style(formatstr).green()
                );
            },
            false => {
                println!(
                    "{} {}",
                    style(Emoji("✅", "✓")).green(),
                    style(formatstr).green()
                );
            }
        }
    }};
}
