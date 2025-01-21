use std::arch::asm;
use std::arch::global_asm;
use cty;

// Using the #[Link] attribute to link the C library
#[link(name = "my_c_library")]  // Sink
extern "C" {
    fn my_c_function(x: i32) -> bool;
}

extern "C" {    // Sink
    fn abs(input: i32) -> i32;
}

fn main() {

    // Using Inline Assembly
    // Multiply x by 6 using shifts and adds (asm! macro)
    let mut x: u64 = 4;
    unsafe {
        asm!(       // Sink
            "mov {tmp}, {x}",
            "shl {tmp}, 1",
            "shl {x}, 2",
            "add {x}, {tmp}",
            x = inout(reg) x,
            tmp = out(reg) _,
        );
    }
    assert_eq!(x, 4 * 6);

    // Include the assembly code from a file
    unsafe {
        asm!(include_str!("something_neato.s"));     // Sink
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// Define a function to be called by inline assembly, THIS ISN'T A SINK
extern "C" fn pt() {    // FP
    println!("This func is called by inline asm");
}

// Use inline assembly
global_asm!(        // Sink
    "
.global myfunc
myfunc:
call {}
ret",
sym pt,
);

// Include the assembly code from a file
global_asm!(include_str!("something_neato.s"));     // Sink

#[repr(C)]  // Not a sink
pub struct CoolStruct {
    pub x: cty::c_int,
    pub y: cty::c_int,
}

extern "C" {        // Sink
    pub fn cool_function( 
        i: cty::c_int,
        c: cty::c_char,
        cs: *mut CoolStruct
    );
}