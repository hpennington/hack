use std::env;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use walkdir::WalkDir;


fn main() {
    // Read command line args for the search text.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of arguments");
    }
    let search_text = &args[1];
    println!("{}", search_text);
    // Get a tree of the files in the current directory.
    // Traverse the tree in breadth first order.
    let current_path = env::current_dir();

    for entry in WalkDir::new(current_path.unwrap().as_path()) {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{}", path.display());

        if fs::metadata(path).unwrap().is_dir() != true {
            let contents = fs::read_to_string(path).unwrap();
            if contents.chars().count() > 0 {
                println!("{}", contents);
            }
            // Read the file.
            // Search for the text in the file contents.
            // Output found text.
        }
    }
}
