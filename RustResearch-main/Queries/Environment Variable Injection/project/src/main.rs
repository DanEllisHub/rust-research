#[macro_use]
extern crate rocket;
use std::collections::HashMap;
use std::fs;

#[get("/env2?<interactive_input>")]
fn tokio_env_injection(interactive_input: String) -> String {
    use tokio::process::Command;
    let stored_input = get_stored_input();
    let mut interactive_envs = HashMap::new();
    let mut stored_envs = HashMap::new();

    // INPUTS FOR SINK #2:
    // interactive:
    interactive_envs.insert("envKey".to_string(), "envValue".to_string()); // NOT A RESULT (both values are hardcoded)
    interactive_envs.insert(interactive_input.clone(), "envValue".to_string()); // NOT A RESULT (value is not controllable)
    interactive_envs.insert("envKey".to_string(), interactive_input.clone()); // RESULT (value is controllable)
    interactive_envs.insert(interactive_input.clone(), interactive_input.clone()); // RESULT (value and key are controllable)

    // stored:
    stored_envs.insert("envKey".to_string(), "envValue".to_string()); // NOT A RESULT (both values are hardcoded)
    stored_envs.insert(stored_input.clone(), "envValue".to_string()); // NOT A RESULT (value is not controllable)
    stored_envs.insert("envKey".to_string(), stored_input.clone()); // RESULT (value is controllable)
    stored_envs.insert(stored_input.clone(), stored_input.clone()); // RESULT (value and key are controllable)
    // END OF INPUTS FOR SINK #2

    Command::new("someCommand.exe")
        .env("PATH", "VALUE") // SINK #1 - NOT A RESULT
        
        .env(interactive_input.clone(), "VALUE") // SINK #1 - NOT A RESULT
        .env("PATH", interactive_input.clone()) // SINK #1 - RESULT
        .env(interactive_input.clone(), interactive_input.clone()) // SINK #1 - RESULT
        
        .env(stored_input.clone(), "VALUE") // SINK #1 - NOT A RESULT
        .env("PATH", stored_input.clone()) // SINK #1 - RESULT
        .env(stored_input.clone(), stored_input.clone()) // SINK #1 - RESULT
        
        .envs(interactive_envs) // SINK #2 - 2 RESULTS AS PER THE HASHMAP INSERTS ABOVE
        .envs(stored_envs) // SINK #2 - 2 RESULTS AS PER THE HASHMAP INSERTS ABOVE
        .spawn()
        .expect("ls command failed to start");
    
    "".to_string()
}

fn get_stored_input() -> String{
    let path = "stored.txt".to_string();

    let contents = fs::read_to_string(path)
    .expect("Should have been able to read the file");

    return contents;
}



#[get("/env1?<interactive_input>")]
fn std_env_injection(interactive_input: String) -> String {
    use std::process::Command;
    let stored_input = get_stored_input();
    let mut interactive_envs = HashMap::new();
    let mut stored_envs = HashMap::new();

    // INPUTS FOR SINK #2:
    // interactive:
    interactive_envs.insert("envKey".to_string(), "envValue".to_string()); // NOT A RESULT (both values are hardcoded)
    interactive_envs.insert(interactive_input.clone(), "envValue".to_string()); // NOT A RESULT (value is not controllable)
    interactive_envs.insert("envKey".to_string(), interactive_input.clone()); // RESULT (value is controllable)
    interactive_envs.insert(interactive_input.clone(), interactive_input.clone()); // RESULT (value and key are controllable)

    // stored:
    stored_envs.insert("envKey".to_string(), "envValue".to_string()); // NOT A RESULT (both values are hardcoded)
    stored_envs.insert(stored_input.clone(), "envValue".to_string()); // NOT A RESULT (value is not controllable)
    stored_envs.insert("envKey".to_string(), stored_input.clone()); // RESULT (value is controllable)
    stored_envs.insert(stored_input.clone(), stored_input.clone()); // RESULT (value and key are controllable)
    // END OF INPUTS FOR SINK #2

    Command::new("someCommand.exe")
        .env("PATH", "VALUE") // SINK #1 - NOT A RESULT
        
        .env(interactive_input.clone(), "VALUE") // SINK #1 - NOT A RESULT
        .env("PATH", interactive_input.clone()) // SINK #1 - RESULT
        .env(interactive_input.clone(), interactive_input.clone()) // SINK #1 - RESULT
        
        .env(stored_input.clone(), "VALUE") // SINK #1 - NOT A RESULT
        .env("PATH", stored_input.clone()) // SINK #1 - RESULT
        .env(stored_input.clone(), stored_input.clone()) // SINK #1 - RESULT
        
        .envs(interactive_envs) // SINK #2 - 2 RESULTS AS PER THE HASHMAP INSERTS ABOVE
        .envs(stored_envs) // SINK #2 - 2 RESULTS AS PER THE HASHMAP INSERTS ABOVE
        .spawn()
        .expect("ls command failed to start");
    
    "".to_string()
}



#[launch]
fn rocket() -> _ {

    rocket::build()
        .mount("/", routes![std_env_injection])
        .mount("/", routes![tokio_env_injection])
}
