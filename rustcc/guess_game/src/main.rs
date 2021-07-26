use std::io;
// Rng is trait
// trait defines methods that random number generators implement,
use rand::Rng;

// type - enum like result
use std::cmp::Ordering;

// our code is binary crate, which is an executable.
// packages are library crate, which contains code .
fn main() {
    println!("Guess_Game");
    // local to the current thread of execution and seeded by the operating system.
    let secret_num = rand::thread_rng().gen_range(1..101);

    // println!("The Secret Number is: {} ", secret_num);
    
    loop {

        // new is an associated function of the String
        // An associated function is implemented on a type
        // Some languages call this a static method.
        let mut guess = String::new();
        
        println!("Please Guess a Number: ");

        // stdin function returns an instance of std::io::Stdin which is type
        // & means reference
        // like variables, references are immutable by default.
        // reasd_line return io::Result ,
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust allows us to shadow the previous value
        // let guess : u32 = guess.trim().parse().expect("Please a Number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed : {}", guess);

        // flow operator called match that allows you to compare a value against a
        // series of patterns and then execute code based on which pattern matches.
        // made up of arms. An arm consists of a pattern and the code that should be run
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Win");
                break;
            }
        }
    }
}

// Result is An enumeration
// enumeration is a type that can have a fixed set of values, and those values are called the enum’s variants.
fn get_string() -> io::Result<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}
