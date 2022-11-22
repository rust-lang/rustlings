// lifetimes2.rs
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long"); //'a
    {
        //这意味着返回值的生命周期与参数生命周期中的较小值一致：
        let result; //返回值引用的生命周期，应该小于等于x，y 里面最短的那个
        let string2 = String::from("xyz"); //'b
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }
}
