use std::{error, fs::File, io::ErrorKind};

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    // panic!("Crash the program.");
    // Above line will force a panic, crashing the program instantly.

    // let v = vec![1, 2, 3];
    // v[99];
    // Above line will cause a panic at run time by trying to access an element of a vector that is unallocated

    let greeting_file_result = File::open("notreal.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("notreal.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
}
