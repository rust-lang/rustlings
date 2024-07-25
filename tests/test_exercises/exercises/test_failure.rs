fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn fails() {
        asset!(false);
    }
}
