
// array4.rs
//
// Here is the code for how the array learns

// I AM NOT DONE


fn main(){
    // define  arrays a 
let a =[1,2,3,4,5];
     println!("please enter an array index.");
let mut  index = String::new();
     std::io::stdin()
             .read_line(&mut index)
             .expect("Fail to read line");
let index :usize =index
             .trim()
             .parse()
             .expect("index entered was not a number");
     
let element = a[index];
     println!("The value of is {}  element : {}",index,element);
     
 }
 