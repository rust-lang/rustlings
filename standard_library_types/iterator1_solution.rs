
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
        return Err(DivisionError::DivideByZero);
    }
    match a % b {
        0 => Ok(a/b),
        _ => Err(DivisionError::NotDivisible(NotDivisibleError{divident:a, divisor:b})),
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
    let numbers = vec![27,297,38502,81];
    let numbers_iterator = numbers.into_iter();
    let division_results = numbers_iterator.map(|n| divide(n, 27));
    let operation_mode = OperationMode::ResultWithList; // you may change this
    match operation_mode {
        OperationMode::ResultWithList => {
            let x : Result<Vec<_>,_> = division_results.collect();
            print_result_with_list(x);
        },
        OperationMode::ListOfResults => {
            let x : Vec<_> = division_results.collect();
            print_list_of_results(x);
        },
    }
}
