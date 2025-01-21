unsafe fn unsafe_fn(ptr: *const i32) -> i32 { // RESULT - use of `unsafe`
    return *ptr;
}

unsafe trait UnsafeTrait { // RESULT - use of `unsafe`
    fn unsafe_method(&self, ptr: *const i32) -> i32;
}

struct MyStruct {
    value: i32,
}


unsafe impl UnsafeTrait for MyStruct { // RESULT - use of `unsafe`
    fn unsafe_method(&self, ptr: *const i32) -> i32 {
        unsafe { *ptr } // RESULT - use of `unsafe`
    }
}

fn main() {
    let x = 10;
    let ptr = &x as *const i32;

    let my_struct = MyStruct { value: 20 };

    unsafe { // RESULT - use of `unsafe`
        println!("Unsafe function: {}", unsafe_fn(ptr));
        println!("Unsafe method: {}", my_struct.unsafe_method(ptr));
    }
}