// An imaginary magical school has a new report card generation system written
// in Rust! Currently, the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct `ReportCard` and the impl
// block to support alphabetical report cards in addition to numerical ones.

use std::fmt::Display;

// Make the struct generic over `T`.
//
// Note: We could also add the Display trait bound here at the struct level, i.e.,
// `struct ReportCard<T: Display> { ... }`. That would enforce that every `ReportCard` instance
// must have a grade that is displayable. However, by not placing the bound here, we allow
// the creation of `ReportCard` instances with non-displayable grades (but then we cannot use the `print` method).
// Since the main purpose of this struct is to be printable, it would also be reasonable to add the bound at the struct level.
// In this solution, we choose to put the bound only in the `impl` block to show that we can conditionally
// implement methods only when the type satisfies certain traits. This offers more flexibility in some advanced scenarios.
struct ReportCard<T> {
    //           ^^^
    grade: T,
    //     ^
    student_name: String,
    student_age: u8,
}

// To be able to print the grade, it has to implement the `Display` trait.
//
// Note: We implement the `print` method only for `ReportCard<T>` where `T` implements `Display`.
// This means that if we have a `ReportCard` with a grade that doesn't implement `Display`,
// we can still create the instance, but we cannot call the `print` method on it.
impl<T: Display> ReportCard<T> {
    //  ^^^^^^^ require that `T` implements `Display`.
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
