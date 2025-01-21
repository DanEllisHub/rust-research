// https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html
#![allow(unconditional_panic)]
#![allow(dead_code)]
#![allow(arithmetic_overflow)]
// Suppress all warnings
#![allow(warnings)]
// Supress overflowing literal errors
#![allow(overflowing_literals)]

use rand::Rng;
use std::io;

fn main() {
    unsafe_test_1();
    unsafe_test_2();
    unsafe_test_3();
    unsafe_test_4();
    unsafe_test_5();
    safe_test_1();
    safe_test_2();
    safe_test_3();
    safe_test_4();
    safe_test_5();
    intended_behavior();

    println!("DMS unsafe: {}", DMS_unsafe(i16::MAX, i16::MAX));
    println!("DMS safe 1: {}", DMS_safe1(i16::MAX, i16::MAX));
    println!("DMS safe 2: {}", DMS_safe2(i16::MAX, i16::MAX));
}

fn test_1() {
    let a = 200u8;  // Hardcoded decimal value (value 200) of type u8
    let b = 100u8;  // Hardcoded decimal value (value 100) of type u8
    
    let result = a + b; // Overflow happens in the sum operation (200 + 100)
    println!("Sum: {}", result);
}


// Hardcoding the values to 255 and 1 will cause an overflow.
fn unsafe_test_1() {
    let deadbeef: u128 = 0xdeadbeefdeadbeefdeadbeefdeadbeef;
    // ints
    let lll: u128 = deadbeef as u128;   // safe type conversion - no numeric general type is greater than u128
    let s_lll: i128 = deadbeef as i128;
    let ll: u64 = lll as u64;
    let s_ll: i64 = ll as i64;
    let l: u32 = ll as u32;
    let s_l: i32 = l as i32;
    let i: u16 = l as u16;
    let s_i: i16 = i as i16;
    let s: u8 = i as u8;
    let s_s: i8 = s as i8;

    // ints
    println!("lll   = 0x{:x} ({} bits)", lll,   std::mem::size_of_val(&lll)     * 8);
    println!("s_lll = 0x{:x} ({} bits)", s_lll, std::mem::size_of_val(&s_lll)   * 8);
    println!("ll    = 0x{:x} ({} bits)", ll,    std::mem::size_of_val(&ll)      * 8);
    println!("s_ll  = 0x{:x} ({} bits)", s_ll,  std::mem::size_of_val(&s_ll)    * 8);
    println!("l     = 0x{:x} ({} bits)", l,     std::mem::size_of_val(&l)       * 8);
    println!("s_l   = 0x{:x} ({} bits)", s_l,   std::mem::size_of_val(&s_l)     * 8);
    println!("i     = 0x{:x} ({} bits)", i,     std::mem::size_of_val(&i)       * 8);
    println!("s_i   = 0x{:x} ({} bits)", s_i,   std::mem::size_of_val(&s_i)     * 8);
    println!("s     = 0x{:x} ({} bits)", s,     std::mem::size_of_val(&s)       * 8);
    println!("s_s   = 0x{:x} ({} bits)", s_s,   std::mem::size_of_val(&s_s)     * 8);
}

// Casting floating point values to integer without checking for overflow will cause truncation (<v1.45) or saturation (v1.45 and above).
fn unsafe_test_2() -> i32 {
    let mut rng = rand::thread_rng();
    let a: f32 = rng.gen();
    let b: f32 = rng.gen();
    let c: i32 = (a * b) as i32;    // result for Type Conversion Error
    println!("a = {}, b = {}, c = {}", a, b, c);
    return c;
}

fn unsafe_test_3() {
    let decimal = 65.4321_f32;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 1000 already fits in a u16 - safe
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 > 255 -> overflow
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is : {}", 1000 as u8); // result for Type Conversion Error
    // -1 < 0 -> overflow
    println!("  -1 as a u8 is : {}", (-1i8) as u8); // result for Type Conversion Error

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16); // safe

    // In boundary case 128 value in 8-bit two's complement representation is -128
    // 128 > 127 -> Overflow
    println!(" 128 as a i8 is : {}", 128 as i8); // result for Type Conversion Error

    // repeating the example above
    // 1000 > 255 -> overflow
    println!("1000 as a u8 is : {}", 1000 as u8); // result for Type Conversion Error
    // and the value of 232 in 8-bit two's complement representation is -24
    // 232 > 127 -> overflow
    println!(" 232 as a i8 is : {}", 232 as i8); // result for Type Conversion Error

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    // 300.0 > 255 -> overflow
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);  // result for Type Conversion Error
    // -100.0 < 0 -> overflow
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8); // result for Type Conversion Error
    // nan doesn't have reference as integer
    println!("   nan as u8 is : {}", f32::NAN as u8); // result for Type Conversion Error
}

fn unsafe_test_4() {
    let a = 10; // by default, this is i32
    let b = a as u64; // cast a to u64, safe

    let c = 300; // Input of Type Conversion Error
    let d = c as u8; // cast c to u8, result for Type Conversion Error

    let e: u64 = 100;
    let f: u32 = e as u32; // cast e to u32, safe

    println!("{} {} {}", b, d, f);
}

// Float casting must have upper bounds, lower bounds, and NaN checks. Missing NaN check.
fn unsafe_test_5() -> i32 {
    let mut rng = rand::thread_rng();
    let a: f32 = rng.gen();
    let b: f32 = rng.gen();
    let c: f32 = a * b;
    
    println!("a = {}, b = {}, c = {}", a, b, c);

    // Missing NaN check
    // lower bound check
    if (c < i32::MIN as f32) {
        println!("Underflow detected");
        return i32::MIN;    // returns the saturated value
    }
    // upper bound check
    if (c > i32::MAX as f32) {
        println!("Overflow detected");
        return i32::MAX;   // returns the saturated value
    }

    return c as i32;    // safe cast (sanitized)
}

// wrapping_add allows the developer to handle overflow as they see fit.
// https://www.reddit.com/r/rust/comments/n14gxp/noob_question_what_is_wrapping_add/
fn safe_test_1() {
    let mut rng = rand::thread_rng();

    let deadbeef: u8 = rng.gen();
    // ints
    let lll: u128 = deadbeef as u128;
    let s_lll: i128 = deadbeef as i128;
    let ll: u64 = lll as u64;
    let s_ll: i64 = ll as i64;
    let l: u32 = ll as u32;
    let s_l: i32 = l as i32;
    let i: u16 = l as u16;
    let s_i: i16 = i as i16;
    let s: u8 = i as u8;
    let s_s: i8 = s as i8;
    // chars
    let c: char = char::from_u32(ll as u32).unwrap();
    let ii: u16 = u32::from(c) as u16;

    // ints
    println!("lll   = 0x{:x} ({} bits)", lll,   std::mem::size_of_val(&lll)     * 8);
    println!("s_lll = 0x{:x} ({} bits)", s_lll, std::mem::size_of_val(&s_lll)   * 8);
    println!("ll    = 0x{:x} ({} bits)", ll,    std::mem::size_of_val(&ll)      * 8);
    println!("s_ll  = 0x{:x} ({} bits)", s_ll,  std::mem::size_of_val(&s_ll)    * 8);
    println!("l     = 0x{:x} ({} bits)", l,     std::mem::size_of_val(&l)       * 8);
    println!("s_l   = 0x{:x} ({} bits)", s_l,   std::mem::size_of_val(&s_l)     * 8);
    println!("i     = 0x{:x} ({} bits)", i,     std::mem::size_of_val(&i)       * 8);
    println!("s_i   = 0x{:x} ({} bits)", s_i,   std::mem::size_of_val(&s_i)     * 8);
    println!("s     = 0x{:x} ({} bits)", s,     std::mem::size_of_val(&s)       * 8);
    println!("s_s   = 0x{:x} ({} bits)", s_s,   std::mem::size_of_val(&s_s)     * 8);
    // chars
    println!("c     = 0x{:x} ({} bits)", u32::from(c), std::mem::size_of_val(&c)* 8);
    println!("ii    = 0x{:x} ({} bits)", ii,    std::mem::size_of_val(&ii)      * 8);
}

fn safe_test_2() {
    let mut rng = rand::thread_rng();
    let lll: i128 = rng.gen();
    
    // sanitization: correctly apply upper and downward bounds
    let s_i: i16 = if lll <= 32767 {if lll >= -32768 {lll as i16} else {-32768}} else {32767};
    
    println!("lll   = 0x{:x} ({} bits)", lll,   std::mem::size_of_val(&lll)     * 8);
    println!("s_i   = 0x{:x} ({} bits)", s_i,   std::mem::size_of_val(&s_i)     * 8);
}

// Float casting must have upper bounds, lower bounds, and NaN checks.
fn safe_test_3() -> i32 {
    let mut rng = rand::thread_rng();
    let a: f32 = rng.gen();
    let b: f32 = rng.gen();
    let c: f32 = a * b;
    
    println!("a = {}, b = {}, c = {}", a, b, c);

    // f32::INFINITY * x = f32::NAN
    // NaN check
    if (c == f32::NAN) {
        println!("NaN detected");
        return 0;    // returns 0
    }
    // lower bound check
    if (c < i32::MIN as f32) {
        println!("Underflow detected");
        return i32::MIN;    // returns the saturated value
    }
    // upper bound check
    if (c > i32::MAX as f32) {
        println!("Overflow detected");
        return i32::MAX;   // returns the saturated value
    }

    return c as i32;    // safe cast (sanitized)
}

// Float calculations won't cause negative values. Casting must have upper bounds and NaN checks only.
fn safe_test_4() -> i32 {
    let mut rng = rand::thread_rng();
    let u_a: u32 = rng.gen();
    let u_b: u32 = rng.gen();
    // Integer -> Float casting causes percision loss instead of Type Conversion Error
    let a: f32 = u_a as f32;    // safe, float is contained in [0, 4294967295]
    let b: f32 = u_b as f32;    // safe, float is contained in [0, 4294967295]
    let c: f32 = a * b;
    
    println!("a = {}, b = {}, c = {}", a, b, c);

    // f32::INFINITY * x = f32::NAN
    // NaN check
    if (c == f32::NAN) {
        println!("NaN detected");
        return 0;    // returns 0
    }
    // lower bound check is not needed
    // upper bound check
    if (c > i32::MAX as f32) {
        println!("Overflow detected");
        return i32::MAX;   // returns the saturated value
    }

    return c as i32;    // safe cast (sanitized)
}

// using constants to define the bounds
fn safe_test_5() {
    let a: i32 = 50000;

    if a >= i16::MIN as i32 && a <= i16::MAX as i32 {
        let b: i16 = a as i16;
        println!("O valor de b é: {}", b);
    } else {
        println!("Erro: o valor é muito grande para ser convertido para i16");
    }
}

fn intended_behavior() {
    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**.
    // This can be seen as intended behavior and shouldn't present results.
    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>()); // no result
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>()); // no result
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>()); // no result
    }
}

fn DMS_unsafe(op1: i16, op2: i16) -> i16 {
    // A sum of two i16 fits into an i32
    let total: i32 = op1 as i32 + op2 as i32;
    return total as i16;
}

fn DMS_safe1(op1: i16, op2: i16) -> i16 {
    // A sum of two i16 fits into an i32
    let total: i32 = op1 as i32 + op2 as i32;
    
    // Saturating the result on casting
    if total < i16::MIN as i32 {
        return i16::MIN;
    }
    if total > i16::MAX as i32 {
        return i16::MAX;
    }
    return total as i16;
}

fn DMS_safe2(op1: i16, op2: i16) -> i16 {
    // A sum of two i16 fits into an i32
    let total: i32 = op1 as i32 + op2 as i32;
    return i16::try_from(total).unwrap_or_else(|_| {
        if total < i16::MIN as i32 {
            i16::MIN
        } else {
            i16::MAX
        }
    });
}