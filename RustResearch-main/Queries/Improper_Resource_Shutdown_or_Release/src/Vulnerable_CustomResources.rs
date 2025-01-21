struct CustomResource {
    name: String,
}

fn main() {
    // Sink - no drop trait or drop function.
    let _res = CustomResource {
        name: String::from("Custom Resource"),

    };
    println!("{}", _res.name);

    other_actions_not_related_to_the_Custom_Resources();
}