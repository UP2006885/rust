fn main() {
    println!("Hello, world!"); // Macro
    println!("Number: {}", 25); // Single Placeholder
    println!("My name is {} I am {} years old.", "Harry", 20); // Multiple Placeholders
    println!("My name is {0}. I am {1} years old. {0} is my name.", "Harry", 20); // Positional Arguments
    println!("My name is {name}. I am {age} years old.", name = "Harry", age = 20); // Named Arguments

    // {:b},{:x},{:o}
    println!("The number {0} is {0:b} in binary.", 67);
    println!("The number {0} is {0:x} in hexadecimal.", 67);
    println!("The number {0} is {0:o} in octal.", 67);
    // ----------------------------------------------------------------
    println!("Number : {0} \nBinary:{0:b} Hexadecimal:{0:x} Octal:{0:o}", 97);

    println!("{0} + {0} = {1}",10, 10 + 10); // Basic math.

    println!("This is a multiple value placeholder {:?}", ("Hello there", 92, "Bye")); // Placeholder for a Debug Trait
    // ----------------------------------------------------------------
    // print! = Prints anything inside the parentheses to console.
    print!("Hello, ");
    print!("World!\n");// \n ADDS a new line. 
    // println! = Prints anything inside the parentheses to console, but adds a new line after.
    println!("Hello, "); 
    println!("World!");
    // eprint! = Prints anything inside the parentheses to console as an error msg.
    eprint!("Hello, ");
    eprint!("World!\n");// \n ADDS a new line. 
    // eprintln! = Prints anything inside the parentheses to console as an error msg, but adds a new line after.
    eprintln!("Hello, "); 
    eprintln!("World!");

}
