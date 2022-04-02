
//Use the io input/Output library
//The library comes from standard library, known as std
use std::io ;
//The `Rng` trait defines the methods
use rand::Rng;

//The `Ordering` type is another enum and has the variants `Less` `Greater` and `Equal`
use std::cmp::Ordering;

/* 
  main() is the entry point to the program
  fn is a syntax to declares a new function
  () indicate there are no parameters
  {} indicate the body of the function
*/
fn main() {
    println!("Gusess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your Guess.");
        
        // Use let toe declare a variable
        // In rust variable is immutable
        // We add mut to make the variable be mutable 
        // use `String:new()` to new a instance of a String 
        //  `String` is a string type, `::` syntax int the `::new` indicates the `new` is an associated function of String
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) 
            // `&` indicates that this argument is a reference, we need add `mut` to make this argument mutable, we use reference to avoid copy data
            .expect("Failed to read line");
        
        //Rust allow us to `shadow` the previous value of guess with a new one
        //The `trim` method on a `String` instance will eliminate any whitespace at the begin and end
        //The `parse` method parses a string into come kind of number. 
        // The colon `:` after guess tell rust we'll annotate the variable's type 
        // `u32` is an usigned 32-bit integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // {} is placehoder to fill the variable value
        println!("You guessed:{}",guess);


        match guess.cmp(&secret_number){
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"), 
                Ordering::Equal => {
                    println!("You win!");
                    break;

                }
        }
    }
}
