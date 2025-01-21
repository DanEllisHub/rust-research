fn main() {
    #[derive(Debug)]
    struct Researcher {
        name: String,
        age: Box<u8>,
    }
    let researcher = Researcher{
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // PARTIAL MOVE - `name` is moved out of researcher, but `age` is referenced
    let Researcher{ name, ref age } = researcher;
    
    println!("The researcher's age is {}", age);
    println!("The researcher's name is {}", name);
    // `researcher` cannot be used but `researcher.age` can be used as it is not moved
    println!("The researcher's age from researcher struct is {}", researcher.age);

    // Not possible to:
    drop(researcher);
    // Sanitizer - But the user must drop:
    drop(researcher.age);
    drop(researcher.name);
}