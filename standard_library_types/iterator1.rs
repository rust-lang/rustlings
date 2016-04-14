// 1. Complete the divide function
// 2. Uncomment and complete the second part of the main function
// For part 2 there is a minor hint around line 100 and a major hint around line 128
// There is a minor side quest in the "print_result_with_list" function
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
    if b == 0 {

    }
    match a % b {

    }
}

// these print functions exist, so you have to satisfy their input types in the main function
fn print_list_of_results(l: Vec<Result<i32,DivisionError>>) {
    println!("{:?}", l);
}
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
    println!("Your divide function seems to work! Good Job.");
    
    /* Second part of main. Uncomment to continue.
    let numbers = vec![27,297,38502,81];
    
    let division_results = // Do not convert the results into a Vec yet. Leave them iterable for now.
    let operation_mode = OperationMode::ResultWithList;
    match operation_mode {
        OperationMode::ResultWithList => {
            
            print_result_with_list(x);
        },
        OperationMode::ListOfResults => {
            
            print_list_of_results(x);
        },
    } */
}

























// Minor hint: In each of the two cases in the match in main, you can create x with either a 'turbofish' or by hinting the type of x to the compiler. You may try both.



























// Major hint: Have a look at the Iter trait and at the explanation of its collect function. Especially the part about Result is interesting.
