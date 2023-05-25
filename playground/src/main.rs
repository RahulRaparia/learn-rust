#![allow(unused)]

use ansi_term::Colour::{Red, Green, Blue, Cyan, Purple, Yellow};
use ansi_term::Style;

fn main() {
    let enabled = ansi_term::enable_ansi_support();

    let x = 42;
    let red_string = Red.paint("a red string").to_string();
    println!("{}: I like {}", red_string, red_string);


    println!("How about some {}  {}?",
         Style::new().bold().paint("tHICXK ASSS --->"),
         Style::new().underline().paint("ASS CRACK"));

         println!("Demonstrating {} and {}!",
         Blue.bold().paint("blue bold"),
         Yellow.underline().paint("yellow underline"));

println!("Yellow on blue: {}", Purple.underline().bold().on(Green).paint("wow!"));


println!("Also yellow on blue: {}", Cyan.on(Blue).fg(Yellow).paint("zow!"));
    
    println!("My lucky number is {}.", x);
    // declare a variable s to hold a list of strings  and numbers.
    let s = vec!["123".to_string(), "ramen".to_string(), "soba".to_string()];
    
    let xs = vec![1, 2, 3, 4, 5];
    println!("The list is: {:?}", xs);
    //Write println!() statements to print xs in all possible formatting trains.
    println!("The list is: {:X?}", s);
}
    