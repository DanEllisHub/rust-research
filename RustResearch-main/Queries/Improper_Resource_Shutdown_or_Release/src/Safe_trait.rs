struct CustomResource {
    // fields representing the custom resource
    name: String,
}

// Sanitizer 
impl Drop for CustomResource {
    fn drop(&mut self) {
        println!("Cleaning up custom resource");
    }
}

fn main() {
    let _res = CustomResource {
        name: String::from("Custom Resource"),

    };
    println!("{}", _res.name);
    other_actions_not_related_to_the_Custom_Resources();
}