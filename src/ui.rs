macro_rules! print_emoji {
    ($emoji:expr, $sign:expr, $color: ident, $fmt:literal, $ex:expr) => {{
        use console::{style, Emoji};
        use std::env;
        let formatstr = format!($fmt, $ex);
        if env::var("NO_EMOJI").is_ok() {
            println!("{} {}", style($sign).$color(), style(formatstr).$color());
        } else {
            println!(
                "{} {}",
                style(Emoji($emoji, $sign)).$color(),
                style(formatstr).$color()
            );
        }
    }};
}

macro_rules! warn {
    ($fmt:literal, $ex:expr) => {{
        print_emoji!("⚠️ ", "!", red, $fmt, $ex);
    }};
}

macro_rules! success {
    ($fmt:literal, $ex:expr) => {{
        print_emoji!("✅ ", "✓", green, $fmt, $ex);
    }};
}
