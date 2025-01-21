use std::num::NonZeroU32;
use std::ptr;
use std::mem::drop;

//vulnerable code
fn unsafe_ref(ptr: *const i32) -> i32 {
    unsafe { *ptr }
}

fn main() {
    println!("start");
    //vulnerable code
    let null_ptr: *const i32 = ptr::null(); //null_ptr has a static value which is none
    unsafe {print!("x: {:?}", *null_ptr);}
    //safe code
    unsafe{
        if null_ptr.is_null(){
            println!("null_ptr is null");
        }
        else{
            println!("null_ptr is {:?}", *null_ptr);
        }
    }
    //safe code
    let input: *const i32 = 42 as *const i32; //input has a static value which is not none
    unsafe {print!("x: {:?}", *input);}
}