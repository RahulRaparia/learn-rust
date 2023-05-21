#![allow(unused)]               //This line allows for unused code in the program without the compiler giving a warning.

use clap::Parser;               //This line imports the Parser trait from the clap crate.

#[derive(Parser)]               //This line derives the Parser trait for the Cli struct
struct Cli {                    //This line defines a new struct named Cli.
    
    pattern: String,            //This line defines a field named pattern of type String in the Cli struct.
    path: std::path::PathBuf,   //This line defines a field named path of type std::path::PathBuf in the Cli struct.
}                               //This line defines a field named path of type std::path::PathBuf in the Cli struct.

fn main() {                                 //This line defines the main function of the program.
    let args = Cli::parse();                //This line creates an instance of the Cli struct and parses command line arguments using the derived implementation of the Parser trait.
    let content = std::fs::                 //This line reads the contents of the file specified by the user into a string.
        read_to_string(&args.path)
        .expect("could not read file");

    for line in content.lines() {           //This line iterates over each line in the file’s content.
        if line.contains(&args.pattern) {   //This line checks if the current line contains the pattern specified by the user.
            println!("{}", line);           //If it does, this line prints it to standard output.
        }
    }
}
