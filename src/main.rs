use std::env;
use std::fs;
use walkdir::WalkDir;


fn main() {
    // Read command line args for the search text.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Wrong number of arguments");
    }
    let search_text = &args[1];
    // Get a tree of the files in the current directory.
    // Traverse the tree in breadth first order.
    let current_path = env::current_dir();

    for entry in WalkDir::new(current_path.unwrap().as_path()) {
        let entry = entry.unwrap();
        let path = entry.path();

        let is_directory = match fs::metadata(path) {
            Ok(meta) => meta.is_dir(),
            Err(_error) => false,
        };

        if is_directory != true {
            
            // Read the file.
            let contents = match fs::read_to_string(path) {
                Ok(file_content) => file_content,
                Err(_error) => std::string::String::from(""),
            };


            if contents.chars().count() > 0 {
                
                // Search for the text in the file contents.
                if contents.contains(search_text) == true {
                    
                    // Output found text.
                    println!("{}", path.display());
                }

            }
           
        }
    }
}
