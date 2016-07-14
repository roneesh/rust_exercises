use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open("./src/tale.txt") {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", "./src/tale.txt",
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", "./src/tale.txt",
                                                   why.description()),
        Ok(_) => {}
    }

	let mut enumerate: Vec<_> = s.to_lowercase().chars().collect();
    let mut chars = Vec::new();

    for letter in &enumerate {
    	match letter {
            &'a' => {},
            &'e' => {},
            &'i' => {},
            &'o' => {},
            &'u' => {},
            &' ' => {},
            _ => chars.push(letter),
        }
    }

    println!("{:?}", chars);

}