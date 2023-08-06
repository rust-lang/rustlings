// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.

<<<<<<< HEAD
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.len() > 0 { 
        Ok(format!("Hi! My name is {}", name))
    } else {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".into())
=======
// I AM NOT DONE

pub fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        None
    } else {
        Some(format!("Hi! My name is {}", name))
>>>>>>> 11d8aea96f2c744d970ed1ffb38785cf5b511e5e
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
