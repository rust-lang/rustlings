// 1. Complete the divide function
// 2. Uncomment and complete the second part of the main function
// For part 2 there is a minor hint around line 100 and a major hint around line 128
// There are some final comments for when you are done around line 150
// Have fun :-)


// In production code you would not derive Debug, but implement it manually to get a better error message.
#[derive(Debug,PartialEq,Eq)]
enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}
#[derive(Debug,PartialEq,Eq)]
struct NotDivisibleError {
    divident: i32,
    divisor: i32,
}

// This function calculates a/b if a is divisible by b.
// Otherwise it returns a suitable error.
fn divide(a: i32, b: i32) -> Result<i32,DivisionError> {
}

#[allow(dead_code)]
enum OperationMode {
    ListOfResults,
    ResultWithList,
}

fn main() {
    // These asserts check that your `divide` function works.
    // In production code these would be tests
    assert_eq!(divide(81,9),Ok(9));
    assert_eq!(divide(81,6),Err(DivisionError::NotDivisible(NotDivisibleError{divident:81,divisor:6})));
    assert_eq!(divide(81,0),Err(DivisionError::DivideByZero));
    assert_eq!(divide(0,81),Ok(0));
    println!("Your divide function seems to work! Good Job.");
    
    /* Second part of main. Uncomment to continue.
    // Don't change these numbers. It will break the assertions later in the code.
    let numbers = vec![27,297,38502,81];
    
    let division_results = // Do not convert the results into a Vec yet. Leave them iterable for now.
    let operation_mode = OperationMode::ResultWithList;
    match operation_mode {
        OperationMode::ResultWithList => {

            println!("{:?}", x);
            assert_eq!(format!("{:?}",x), "Ok([1, 11, 1426, 3])");
        },
        OperationMode::ListOfResults => {

            println!("{:?}", x);
            assert_eq!(format!("{:?}",x), "[Ok(1), Ok(11), Ok(1426), Ok(3)]");
        },
    }
    */
}







































// Minor hint: In each of the two cases in the match in main, you can create x with either a 'turbofish' or by hinting the type of x to the compiler. You may try both.



























// Major hint: Have a look at the Iter trait and at the explanation of its collect function. Especially the part about Result is interesting.





















// Final comments
// When you call the function `print_result_with_list` with x, you don't need any type annotations on x anymore.
// The compiler can infer its type through the function's input type.

#[allow(dead_code)]
// Don't use this function to solve the exercise
fn print_result_with_list(r: Result<Vec<i32>,DivisionError>) {
    // side quest: why is there no semicolon in this function?
    match r {
        Ok(v) => println!("All numbers were successfully divided: {:?}", v),
        Err(e) => match e {
            DivisionError::NotDivisible(nde) => println!("Failed to divide {} by {}: Not divisible!", nde.divident, nde.divisor),
            DivisionError::DivideByZero => println!("Can't divide by zero"),
        },
    };
}
