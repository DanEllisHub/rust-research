use std::fs;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;


// Default file creation mode:
fn default(){
    let f = File::create("default.txt").unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();
    println!("Default Mode: {:o}", permissions.mode());
}


fn test1(){
    let filename = "foo644.txt";
    let f = File::create(filename).unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o644); // Read/write for owner and read for others.
    fs::set_permissions(filename, permissions.clone());
    println!("644 Mode: {:o}", permissions.mode());
}

fn test2(){
    let filename = "foo777.txt";
    let f = File::create(filename).unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o777); // Read/write for owner and read for others.
    fs::set_permissions(filename, permissions.clone());
    println!("777 Mode: {:o}", permissions.mode());
}

fn test3(){
    let filename = "foo640.txt";
    let f = File::create(filename).unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o640); // Read/write for owner and none for others.
    fs::set_permissions(filename, permissions.clone());
    println!("640 Mode: {:o}", permissions.mode());
}


fn test4(){
    let filename = "foo_ro.txt";
    let f = File::create(filename).unwrap();
    let metadata = f.metadata().unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_readonly(true);
    fs::set_permissions(filename, permissions.clone());
    println!("ro Mode: {:o}", permissions.mode());
}


fn default_dir(){
    let dirname = "default";
    fs::create_dir(dirname);
    let metadata = fs::metadata(dirname).unwrap();
    let mut permissions = metadata.permissions();
    println!("Default Dir Mode: {:o}", permissions.mode());
}

fn test1_dir() {
    let dirname = "test1_dir";
    fs::create_dir(dirname);
    let metadata = fs::metadata(dirname).unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(640); // Read/write for owner and read for others.
    fs::set_permissions(dirname, permissions.clone());
    println!("Default Dir Mode: {:o}", permissions.mode());
}



fn main() {
    default();
    test1();
    test2();
    test3();
    test4();
    default_dir();
    test1_dir();

    println!("Bye");
}