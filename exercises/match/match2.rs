// match2.rs
// Make me compile! Execute `rustlings hint match2` for hints :)

// You can add filters to match.

// I AM NOT DONE

///
/// Filters one match test
/// 
fn filters_one(){

    let pair = (6, -5);

    match pair {
        (x , y) if x == y => println!("The two values are equal"),
        (x , y) if x + y == 0 => println!("two values are added"),
        (x , y) if x - y == 11 => println!("positive number"),
        _ => println!("No match found"),
    }
}

///
/// Filters two match test
/// 
fn filters_two(){

    let num : u8 = 9;

    match num {
        i if i == 0 => println!("0"),
        i if i == 8 => println!("8"),
        _ => println!("No match found")
    }
}