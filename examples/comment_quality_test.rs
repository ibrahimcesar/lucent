// This is a test file to demonstrate comment quality detection

/// High quality comment: This function calculates the factorial because
/// we need recursive multiplication for combinatorial analysis. The algorithm
/// uses tail recursion optimization to prevent stack overflow for large inputs.
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Set x to 5  (obvious comment - low quality)
fn bad_comment_example() {
    let x = 5;  // increment x (obvious and wrong - low quality)

    // ========================================
    // This is just noise - low quality
    // ========================================

    println!("{}", x);
}

/// See https://doc.rust-lang.org/book/ch10-00-generics.html for more info
/// This is a high-quality reference comment with external documentation
fn with_reference<T>() {
    // TODO: implement this
}

// Commented out code - low quality
// fn old_implementation() {
//     let x = 5;
//     if x > 0 {
//         return x + 1;
//     }
// }

/// Purpose: This function handles edge cases for division because
/// we need to prevent divide-by-zero errors. The implementation uses
/// Option to safely return None when dividing by zero, which is important
/// for maintaining program stability in production environments.
fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    println!("Factorial of 5: {}", factorial(5));
    bad_comment_example();
    with_reference::<i32>();

    if let Some(result) = safe_divide(10, 2) {
        println!("Division result: {}", result);
    }
}
