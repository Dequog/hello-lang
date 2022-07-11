// Import file system and environment variables
use std::fs;
use std::env;


fn main() {
    // Command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // If there are no arguments, then they haven't specified a file
        println!("Error: Please specify file");
    }else if args.len() > 2 {
        // There can only be one argument
        println!("Error: Too many arguments");
    }else if args.len() == 2 {
        // If there is one argument, then they have specified a file

        // Check if the file exists
        let check = || -> Result<(), std::io::Error> {
            fs::read_to_string(&args[1])?;
            Ok(())
        };
    
        if let Err(_err) = check() {
            println!("Error: File does not exist");
        }else {
            // Read file
            let code = fs::read_to_string(&args[1]).expect("Error: File does not exist");
    
            // Evaluate code
            if code.split("\n").collect::<Vec<&str>>().contains(&"Hello World!") {
                // If the file contains the string "Hello World!" on a single line, then print "Hello World!"
                println!("Hello World!");
            }else {
                // If the file does not contain the string "Hello World!", then print out and error
                println!("Error: Code must contain phrase: \"Hello World!\" on it's own line");
            }
        }
    }else {
        // An unknown error has occurred
        println!("Error: Unknown error");
    }
}
