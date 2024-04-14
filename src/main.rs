use std::{error::{self, Error}, fs::{self, File}, io::{self, ErrorKind, Read}};

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    // panic!("Crash the program.");
    // Above line will force a panic, crashing the program instantly.

    // let v = vec![1, 2, 3];
    // v[99];
    // Above line will cause a panic at run time by trying to access an element of a vector that is unallocated


    //ERROR HANDLING USING MATCH
    // let greeting_file_result = File::open("notreal.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("notreal.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     }
    // };
    
    //ERROR HANDLNG USING .expect()
    //let new_greeting_file = File::open("Hello.txt")
    //    .expect("Hello.txt not found in project.");

    let res = read_username2();
}

fn read_username() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
