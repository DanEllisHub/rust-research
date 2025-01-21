use std::ascii::AsciiExt;

#[deprecated = "this is a 'custom' deprecated module"]
mod custom_deprecated_module {
    pub struct custom_module_struct {
        pub my_field: i32,
    }

    pub(crate) fn custom_module_function() {
        println!("Hello, world!");
    }
}

trait NonDeprecatedTrait {
    fn my_trait_method(&self);
}

enum NonDeprecateEnum {
    Dog(String, f64),
    Cat { name: String, weight: f64 },
}

#[deprecated = "this is a 'custom' deprecated enum"]
enum DeprecateEnum {
    Dog(String, f64),
    Cat { name: String, weight: f64 },
}

struct NonDeprecatedStruct{
}
struct AnotherNonDeprecatedStruct{
}
#[deprecated = "this is a 'custom' deprecated struct"]
struct DeprecatedStruct{
}

#[deprecated = "this is a 'custom' deprecated trait"]
trait DeprecatedTrait {
    fn is_ascii(&self) -> bool;
    fn eq_ignore_ascii_case(&self, o: &Self) -> bool;
    fn make_ascii_uppercase(&mut self);

    #[deprecated = "this is a 'custom' deprecated function"]
    fn make_ascii_lowercase(&mut self);

}

impl DeprecatedTrait for DeprecatedStruct { // 2 RESULTS - use of `DeprecatedStruct` and `DeprecatedTrait`
    fn is_ascii(&self) -> bool {
        true
    }

    fn eq_ignore_ascii_case(&self, o: &Self) -> bool {
        true
    }

    fn make_ascii_uppercase(&mut self) {
        print!("make_ascii_uppercase");
    }

    fn make_ascii_lowercase(&mut self) {
        print!("make_ascii_uppercase");
    }
}

// implement NonDeprecatedTrait for NonDeprecatedStruct
impl NonDeprecatedTrait for NonDeprecatedStruct { // SAFE - NO RESULTS
    fn my_trait_method(&self) {
        println!("my_trait_method");
    }
}

// implement DeprecatedTrait for NonDeprecatedStruct
impl DeprecatedTrait for AnotherNonDeprecatedStruct { // RESULT - use of `DeprecatedTrait`
    fn is_ascii(&self) -> bool {
        true
    }

    fn eq_ignore_ascii_case(&self, o: &Self) -> bool {
        true
    }

    fn make_ascii_uppercase(&mut self) {
        print!("make_ascii_uppercase");
    }

    fn make_ascii_lowercase(&mut self) {
        print!("make_ascii_uppercase");
    }
}

// implement NonDeprecatedTrait for DeprecatedStruct
impl NonDeprecatedTrait for DeprecatedStruct { // RESULT - use of `DeprecatedStruct`
    fn my_trait_method(&self) {
        println!("my_trait_method");
    }
}

impl AsciiExt for NonDeprecatedStruct {  // RESULT - use of `std::ascii::AsciiExt`
    type Owned = NonDeprecatedStruct;

    fn is_ascii(&self) -> bool {
        self.is_ascii() // RESULT - use of `std::ascii::AsciiExt::is_ascii`
    }

    fn to_ascii_uppercase(&self) -> Self::Owned { // RESULT - use of `std::ascii::AsciiExt::Owned`
        self.to_ascii_uppercase() // RESULT - use of `std::ascii::AsciiExt::to_ascii_uppercase`
    }

    fn to_ascii_lowercase(&self) -> Self::Owned {
        self.to_ascii_lowercase() // RESULT - use of `std::ascii::AsciiExt::to_ascii_lowercase`
    }

    fn eq_ignore_ascii_case(&self, o: &Self) -> bool {
        self.eq_ignore_ascii_case(o) // RESULT - use of `std::ascii::AsciiExt::eq_ignore_ascii_case`
    }

    fn make_ascii_uppercase(&mut self) {
        self.make_ascii_uppercase(); // RESULT - use of `std::ascii::AsciiExt::make_ascii_uppercase`
    }

    fn make_ascii_lowercase(&mut self) {
        self.make_ascii_lowercase(); // RESULT - use of `std::ascii::AsciiExt::make_ascii_lowercase`
    }
}

impl NonDeprecatedTrait for custom_deprecated_module::custom_module_struct { // RESULT - use of `custom_module_struct`
    fn my_trait_method(&self) {
        println!("my_trait_method");
    }
}

fn main(){

    // VULNERABLE
    custom_deprecated_module::custom_module_function(); // RESULT - use of `custom_module_function`
    let mut a: DeprecateEnum = DeprecateEnum::Dog("Cocoa".to_string(), 37.2);  // Result - use of `DeprecateEnum`
    
    
    
    // SAFE:
    let mut b: NonDeprecateEnum = NonDeprecateEnum::Dog("Cocoa".to_string(), 37.2); // SAFE
}

