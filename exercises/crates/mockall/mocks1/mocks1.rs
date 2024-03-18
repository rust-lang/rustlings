// mocks1.rs
//
// Mockall is a powerful mock object library for Rust. It provides tools to create 
// mock versions of almost any trait or struct. They can be used in unit tests as 
// a stand-in for the real object.
// 
// These tests each contain an expectation that defines some behaviour we expect on
// calls to the function "foo". Add the "foo" function call and get the tests to pass 
//
// I AM NOT DONE
 
use mockall::*;
use mockall::predicate::*;

#[automock]
trait MyTrait {
    fn foo(&self) -> bool;
}

fn follow_path_from_trait(x: &dyn MyTrait) -> String {
    if ??? {
        String::from("Followed path A")
    }
    else {
        String::from("Followed path B")
    }
}

#[test]
fn can_follow_path_a() {
    let mut mock = MockMyTrait::new();
    mock.expect_foo()
        .times(1)
        .returning(||true);
    assert_eq!(follow_path_from_trait(&mock), "Followed path A");
}

#[test]
fn can_follow_path_b() {    
    let mut mock = MockMyTrait::new();
    mock.expect_foo()
        .times(1)
        .returning(||false);
    assert_eq!(follow_path_from_trait(&mock), "Followed path B");
}