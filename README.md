# learn-rust
Parsing command-line arguments
A typical invocation of our CLI tool will look like this:


$ grrs foobar test.txt

We expect our program to look at test.txt and print out the lines that contain foobar. But how do we get these two values?

The text after the name of the program is often called the “command-line arguments”, or “command-line flags” (especially when they look like --this). Internally, the operating system usually represents them as a list of strings – roughly speaking, they get separated by spaces.

Getting the arguments
The standard library contains the function std::env::args() that gives you an iterator of the given arguments. The first entry (at index 0) will be the name your program was called as (e.g. grrs), the ones that follow are what the user wrote afterwards.

Getting the raw arguments this way is quite easy (in file src/main.rs, after fn main() {):


let pattern = std::env::args().nth(1).expect("no pattern given");

let path = std::env::args().nth(2).expect("no path given");