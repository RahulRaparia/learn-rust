#![allow(unused)] //This line allows for unused code in the program without the compiler giving a warning.

use clap::Parser;
use std::io::Read;
use std::path::PathBuf;
use anyhow::{Context, Result};

#[derive(Parser)] //This line derives the Parser trait for the Cli struct
struct Cli {
    //This line defines a new struct named Cli.
    pattern: String, //This line defines a field named pattern of type String in the Cli struct.
    path: std::path::PathBuf, //This line defines a field named path of type std::path::PathBuf in the Cli struct.
} //This line defines a field named path of type std::path::PathBuf in the Cli struct.

#[derive(Debug)]    //This line derives the Debug trait for the CustomError struct.
struct CustomError(String);     //This line defines a new struct named CustomError.

fn main() -> Result<()> {
    //This line defines the main function of the program.
    let args = Cli::parse(); //This line creates an instance of the Cli struct and parses command line arguments using the derived implementation of the Parser trait.
                             //unwrap
                             // let content = std::fs::read_to_string(&args.path).unwrap(); //This line reads the file specified by the user and stores its content in the content variable.

    //?
    // let content = std::fs::read_to_string("../test.txt")?;

    //Custome Error  :
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    println!("Pattern: \n{}\n", args.pattern);
    for line in content.lines() {
        //This line iterates over each line in the file’s content.
        if line.contains(&args.pattern) {
            //This line checks if the current line contains the pattern specified by the user.
            println!("{}", line); //If it does, this line prints it to standard output.
        }
    }
    Ok(())
}
