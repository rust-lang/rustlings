// match1.rs
// Make me compile! Execute `rustlings hint match1` for hints :)

// Use the match of the official library for basic matching, tuple matching, 
// structure matching and enumeration matching.

// I AM NOT DONE

///
/// Basic match test
/// 
fn basic_match_test(){
    let number = 70;

    println!("The number is:{}", number);
    match number {
        1 => println!("1"),
        5 | 7 | 8 | 9  => println!("include 5 7 8 9"),
        13..=22 => println!("include 20"),
        _ => println!("other"),
    }

    let flag = true;
    let status = match flag {
        false => 0,
        true => 1,
    };
    println!("state: {} -> {}", flag, status)
}

///
/// Match test of tuples tuple type
///
fn tuples_match_test(){
    let age = (15,22,33);
    match age {
        (7 , _y ,_z) => println!("{:?}:include age of 7", age),
        (15, ..) => println!("{:?}:age matching from over 15", age),
        _ => println!("{:?}:other age", age)
    }
}

///
/// Match test of enum type
///
enum Color{
    RED,
    BLUE,
    GREEN,

    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    CMY(u32, u32, u32),
}
enum HSV{
    HSV(u32, u32, u32),
}

fn enums_match_test(){
    let color = Color::RGB(178, 50, 32);
    let hsv = HSV::HSV(5, 32, 66);

    match color {
        Color::RED => println!("red"),
        Color::BLUE => println!("blue"),
        Color::GREEN => println!("green"),

        Color::RGB(r, g,b)  =>
            println!("RGB Color R:{}, G:{}, B:{}", r, g, b ),
        Color::HSV(h,s,v)   =>
            println!("HSV H:{:}, S:{:}, V:{:}", h, s, v),
        Color::CMY(c,m,y)   =>
            println!("CMY C:{:?}, M:{:?}, Y:{:?}",c, m, y)
    }

    match hsv { HSV::HSV(h,s,v) =>
            println!("HSV H:{:}, S:{:}, V:{:}", h, s, v),
    }
}

///
/// Structs structure match test
///
#[test]
fn structs_match_test(){
    struct Foo{
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo{
        x : (6, 8), y : 20
    };

    match foo {
        Foo { x: (1, b), y }
            => println!("The first number of X is 1, b = {},  y = {} ", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, It has nothing to do with X", y),
    }
}