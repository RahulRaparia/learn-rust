fn main() {
    /*
    These lines define a function named main. The main function is special: 
    it is always the first code that runs in every executable Rust program. 
    Here, the first line declares a function named main that has no parameters 
    and returns nothing. If there were parameters, they would go inside the parentheses ().

    The function body is wrapped in {}. Rust requires curly brackets around all function bodies. 
    It’s good style to place the opening curly bracket on the same line as the function declaration, 
    adding one space in between.
    */
    println!("Hello, World");
    /*
    1. Rust style is to indent with four spaces, not a tab.
    2. println! calls a Rust macro. using a ! means that you’re calling a macro instead of 
       a normal function.
    3. "Hello, world!" string. We pass this string as an argument to println!
    4. we end the line with a semicolon (;), Most lines of Rust code end with a semicolon.
    */
}