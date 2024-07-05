fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    // Solution1: You can move `strings2` out of the inner block so that it is
    // not dropped before the print statement.
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
    // `string2` dropped at the end of the function.

    // =========================================================================

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        // Solution2: You can move the print statement into the inner block so
        // that it is executed before `string2` is dropped.
        println!("The longest string is '{result}'");
        // `string2` dropped here (end of the inner scope).
    }
}
