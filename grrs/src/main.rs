// Basic:
// struct Cli {
//     pattern: String,
//     path: std::path::PathBuf,
// }

// let pattern = std::env::args().nth(1).expect("no pattern given");
// let path = std::env::args().nth(2).expect("no path given");

// let args = Cli {
//     pattern : pattern,
//     path: std::path::PathBuf::from(path),
// };

// Parsing CLI arguments with Clap
use clap::Parser;
use std::io::{BufRead, BufReader};
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    /* 
    In the modified version, BufReader is used to read the file line 
    by line efficiently. It wraps the file reader (File) and provides a 
    buffered interface for reading. Each line is checked for the pattern, 
    and if it matches, it's printed. The code also includes error 
    handling for file opening and reading.
    */
    let args = Cli::parse();
    // let file = File::open(&args.path).expect("could not open file");
    // let reader = BufReader::new(file);

    /*
        Following is the read to string implementation
     */
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    //read to string :
    for line in content.lines() {
        // println!("{}", line);
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }



    //Buff reader :
    // for line in reader.lines() {
    //     if let Ok(line) = line{
    //         if line.contains(&args.pattern) {
    //             println!("{}", line);
    //         }
    //     }
    // }

}

