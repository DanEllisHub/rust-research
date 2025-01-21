#![allow(unconditional_panic)]
#![feature(isqrt)]
use rand::Rng;
use std::panic;
use std::io;

fn main() {
    unsafe_test_1();
    unsafe_test_2();
    unsafe_test_3();
    unsafe_test_4();
    unsafe_test_5();
    unsafe_test_6();
    unsafe_test_7();
    unsafe_test_8();
    unsafe_test_9();
    unsafe_test_10();
    unsafe_test_11();
    unsafe_test_12();
    unsafe_test_13();
    unsafe_test_14();
    safe_test_1();
    safe_test_2();
    safe_test_3();
    safe_test_4();
    safe_test_5();
    safe_test_6();
    safe_test_7();
    safe_test_8();
    safe_test_9();
    safe_test_10();

    println!("{}", unsafe_addition(i32::MAX, i32::MAX));
    println!("{}", safe_addition_1(i32::MAX, i32::MAX));
    println!("{}", safe_addition_2(i32::MAX, i32::MAX));
    println!("{}", safe_addition_3(i8::MAX, i8::MAX));
}


// Hardcoding the values to 255 and 1 will cause an overflow.
fn unsafe_test_1() {
    let x: u8 = 255;
    let y: u8 = 1;
    let sum = x + y; // Overflow occurs here
    println!("{} + {}\nSum: {}", x, y, sum);
}

// Randomly generated values can also cause an overflow.
fn unsafe_test_2() {
    let mut rng = rand::thread_rng();

    let x: u8 = rng.gen();
    let y: u8 = rng.gen();
    let sum = x + y; // Overflow occurs here
    println!("{} + {}\nSum: {}", x, y, sum);
}

// overflow occurs when adding two u8 values, before u8->u16 conversion.
fn unsafe_test_3() {
    let x: u8 = 255;
    let y: u8 = 1;
    let sum: u16 = (x + y) as u16; // Overflow occurs here
    println!("{} + {}\nSum: {}", x, y, sum);
}

fn unsafe_test_4() {
    let x: u32 = 1_000_000;
    let y: u32 = 1_000_000;
    let product = x * y; // Overflow occurs here
    println!("{} * {}\nProduct: {}", x, y, product);
}

// The pow method is executed using the base value type.
fn unsafe_test_5() {
    let base: u8 = 2;
    let exponent: u32 = 8;
    let result: u32 = base.pow(exponent).into(); // Integer Overflow occurs on pow method
    println!("The result of {} to the power of {} is {}", base, exponent, result);
}

// Division could also allow for overflow, but can't produce an underflow wraparound.
fn unsafe_test_6() {
    let x: i8 = i8::MIN; // Hardcoded constant value of -128
    let y: i8 = -1; // Hardcoded value

    let result = x / y; // Overflow happens in the division operation (-128 / -1)
    println!("{} / {} = {}", x, y, result);
}

// Overflow incorrectly sanitized
fn unsafe_test_7() {
    let mut rng = rand::thread_rng();
    let a: u8 = rng.gen();  // Input
    let b: u8 = 10;

    if a <= 246 {
        let sum = a + b; // sink for Integer Overflow (if a is 246)
        println!("{} + {}\nSum: {}", a, b, sum);
    } else {
        println!("{} is greater than 245", a);
    }
}

// User input can be used to trigger an underflow.
fn unsafe_test_8() {
    let user_input: u16 = get_user_input("Please enter a value: ").parse().unwrap();

    if user_input > 10 {
        let a = user_input.saturating_sub(10);
        let b = 10;

        println!("Underflow might happen: {}", a - b);
    } else {
        println!("Invalid input");
    }
}

// Adding to i16 MAX value causing Integer Overflow
fn unsafe_test_9() {
    let x: i16 = i16::MAX; // Hardcoded constant value of 32767
    let y: i16 = get_user_input("Please enter a value: ").parse().unwrap(); // Input

    if y < 10 {
        let result = x + y; // sink
        println!("{} + {} = {}", x, y, result);
    }
}

// Multiplication causing Integer Overflow and Underflow
fn unsafe_test_10() {
    // Since there is user input, no result should start in this hardcoded input
    let x: i16 = 50;
    // Input for Integer Overflow and Integer Underflow
    let y: i16 = get_user_input("Please enter a value: ").parse().unwrap();

    let result = x * y; // sink
    println!("{} * {} = {}", x, y, result);
}

// Multiplication on unsigned integer causing Integer Overflow and Underflow
fn unsafe_test_11() {
    // Since there is user input, no result should start in this hardcoded input
    let x: u16 = 50;
    // Input for Integer Overflow
    let y: u16 = get_user_input("Please enter a value: ").parse().unwrap();

    let result = x * y; // sink for overflow and underflow
    println!("{} * {} = {}", x, y, result);
}

// Multiplication on unsigned integer causing Integer Underflow only
fn unsafe_test_12() {
    // Since there is user input, no result should start in this hardcoded input
    let x: i8 = 50;
    // Input for Integer Underflow
    let y: i8 = get_user_input("Please enter a value: ").parse().unwrap();

    let result = x * -y.abs(); // sink for underflow, as overflow is not possible
    println!("{} * -({}) = {}", x, y, result);
}

// 
fn unsafe_test_13() {
    let mut rng = rand::thread_rng();
    let a: i8 = rng.gen();
    let b: i8 = rng.gen();
    let c: i8 = rng.gen();

    // b outside [-11, 11] causes overflow, -6>a<6 and -6>c<6 causes overflow and underflow
    let d = b.pow(2) - 4 * a * c; // Integer Overflow in pow, Integer Overflow and Integer Underflow in both first * and second *
    let x1 = (-b + d.abs().isqrt()) / (2 * a); // d can be any value; a [-63, 63], b [-127, 127] (-128 causes overflow)
    let x2 = (-b - d.abs().isqrt()) / (2 * a); // same as above

    println!("The zeroes of the polinomial equation {}x^2 + {}x + {} are x1 = {} and x2 = {}", a, b, c, x1, x2);
}

// Partial sanitization against integer overflows and underflows
fn unsafe_test_14() {
    let mut rng = rand::thread_rng();
    let a: i8 = rng.gen();
    let b: i8 = rng.gen();
    let c: i8 = 6;  // Hardcoded value

    assert!(a >= -31 && a <= 32, "a can still cause overflow in a * c");
    assert!(b >= -127, "b can still cause overflow in b.pow(2)");

    // b outside [-11, 11] causes overflow, a<-6 causes underflow, a>6 causes overflow
    let d = b.pow(2) - 4 * a * c; // Integer Overflow in pow and the second *, Integer Underflow in the second *
    let x1 = (-b + d.abs().isqrt()) / (2 * a); // no result should appear in this line
    let x2 = (-b - d.abs().isqrt()) / (2 * a); // no result should appear in this line

    println!("The zeroes of the polinomial equation {}x^2 + {}x + {} are x1 = {} and x2 = {}", a, b, c, x1, x2);
}

// wrapping_add allows the developer to handle overflow as they see fit.
// https://www.reddit.com/r/rust/comments/n14gxp/noob_question_what_is_wrapping_add/
fn safe_test_1() {
    let mut rng = rand::thread_rng();

    let x: u8 = rng.gen();
    let y: u8 = rng.gen();
    let sum = x.wrapping_add(y); // No panic, wrapping_add is a sanitizer
    println!("{} + {}\nSum: {}", x, y, sum);
}

// u16 can be used to securely store the sum of two u8 values.
fn safe_test_2() {
    let x: u8 = 255;
    let y: u8 = 1;
    let sum: u16 = x as u16 + y as u16; // No panic
    println!("{} + {}\nSum: {}", x, y, sum);
}

fn safe_test_3() {
    let x: u8 = 0xff; // Hardcoded hex value
    let y: u8 = 'a' as u8; // Hardcoded char converted to u8

    let result = x.checked_add(y);

    match result {
        Some(v) => println!("Sum: {}", v),
        None => println!("Integer overflow occurred"),
    }
}

// The pow method is executed using the base value type.
fn safe_test_4() {
    let base: u8 = 2;
    let exponent: u32 = 7;
    let result: u8 = base.pow(exponent); // 2^7 = 128, which fits inside a u8 type
    println!("The result of {} to the power of {} is {}", base, exponent, result);
}

fn safe_test_5() {
    let base: u16 = 2;
    let exponent: u32 = 8;
    let result: u16 = base.pow(exponent); // 2^8 = 256, which fits inside a u16 type
    println!("The result of {} to the power of {} is {}", base, exponent, result);
}

// Overflow sanitized by value limitation.
fn safe_test_6() {
    let mut rng = rand::thread_rng();
    let a: u8 = rng.gen();
    let b: u8 = 10;

    if a < 246 {
        let sum = a + b;
        println!("{} + {}\nSum: {}", a, b, sum);
    } else {
        println!("{} is greater than 245", a);
    }
}

// quadratic equation solving with sanitization against integer overflows and underflows
fn safe_test_7() {
    let mut rng = rand::thread_rng();
    let a: i8 = rng.gen();
    let b: i8 = rng.gen();
    let c: i8 = rng.gen();

    if (a >= -6 && a <= 6) // a would cause overflow
        && (b >= -11 && b <= 11) // b would cause overflow
        && (c >= -6 && c <= 6) { // c would cause overflow

        let d = b.pow(2) - 4 * a * c; // b [-11, 11]; a [-6, 6]; c [-6, 6]
        let x1 = (-b + d.abs().isqrt()) / (2 * a); // d can be anything; a [-63, 63], b [-127, 127] (-128 causes overflow)
        let x2 = (-b - d.abs().isqrt()) / (2 * a); // same as above

        println!("The zeroes of the polinomial equation {}x^2 + {}x + {} are x1 = {} and x2 = {}", a, b, c, x1, x2);
    } else {
        println!("Invalid input");
    }
}

// No power exponent will cause 1 to overflow. It's always 1.
fn safe_test_8() {
    let base: u32 = 1;
    let exponent: u32 = get_user_input("Please enter a value: ").parse().unwrap();
    let result: u32 = base.pow(exponent); // 1^x = 1, which fits inside a u32 type
    println!("The result of {} to the power of {} is {}", base, exponent, result);
}

// No value to the power of 1 will cause an overflow. It's always the base value.
fn safe_test_9() {
    let base: u32 = get_user_input("Please enter a value: ").parse().unwrap();
    let exponent: u32 = 1;
    let result: u32 = base.pow(exponent); // x^1 = x, which fits inside a u32 type
    println!("The result of {} to the power of {} is {}", base, exponent, result);
}

// Any value multiplied by 1 will not cause an overflow.
fn safe_test_10() {
    let a: u32 = 1;
    let b: u32 = get_user_input("Please enter a value: ").parse().unwrap();
    let result: u32 = a * b; // 1 x b = b, which fits inside a u32 type
    println!("{} * {} = {}", a, b, result);
}


// Function to get user input
fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    // Create a mutable string to store the user input
    let mut input = String::new();

    // Read input from the console
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // Remove trailing newline character
    input.trim().to_string()
}

// DMS Samples

fn unsafe_addition(a: i32, b: i32) -> i32 {
    let sum = a + b;    // Buffer Underflow and Buffer Overflow

    if a > 0 && b > 0 && sum < 0 {
        panic!("Integer overflow!");
    }
    
    return sum;
}

fn safe_addition_1(a: i32, b: i32) -> i32 {
    let sum = a + b;    // No results here - sum is not used without validation

    if a > 0 && b > 0 && sum < 0 {
        panic!("Integer overflow!");
    }
    if a < 0 && b < 0 && sum > 0 {
        panic!("Integer underflow!");
    }
    return sum;
}

fn safe_addition_2(a: i32, b: i32) -> i32 {
    let sum = a.checked_add(b); // No results here - checked_add is safe

    if sum.is_none() {
        panic!("Wraparound occurred!");
    }
    return sum.unwrap();
}

fn safe_addition_3(a: i8, b: i8) -> i8 {
    let sum = a as i16 + b as i16;  // i16 fits the sum of two i8 values

    if sum > i8::MAX as i16 {
        panic!("Integer overflow!");
    }
    if sum < i8::MIN as i16 {
        panic!("Integer underflow!");
    }

    return sum as i8;
}
