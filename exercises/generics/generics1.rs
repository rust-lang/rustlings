// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.

// ChatGPT
// Vec<_> is a shorthand syntax in Rust for declaring a vector with the element type left unspecified.

// In Rust, a vector is a dynamically resizable array that allows you to store and manipulate a collection of elements. When declaring a vector, you typically specify the element type that the vector will contain. For example, to create a vector of integers, you would declare it as Vec<i32>.

// However, if you want to declare a vector without specifying the element type, you can use the underscore (_) as a placeholder. This is useful when you want the compiler to infer the element type based on the values you insert into the vector.

// For example, the following code creates a vector of integers and inserts the values 1, 2, and 3 into it:

// let v: Vec<_> = vec![1, 2, 3];
// Here, the underscore allows the compiler to infer that the vector contains integers. If you were to remove the underscore, you would have to explicitly specify the element type as i32:

// let v: Vec<i32> = vec![1, 2, 3];

fn main() {
    let mut shopping_list: Vec<_> = Vec::new();
    // let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
