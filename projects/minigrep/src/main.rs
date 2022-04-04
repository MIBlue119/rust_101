use std::env;
use std::process;
use minigrep::{Config, load_file};

fn main() {
    /*
    - We use the function  std::env::args() to get the iterator of the command line arguments.
    - We call `collect` to turn into collection
    - We turn the iterator to vector
    - Print the vector with debug formatter `:?`
    */
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    //Declare variable to get the cli arguments vector value
    let config= Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}",err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("File: {}",config.file);

    /*
    use if let rather than unwrap_or_else to check whether run returns an Err value 
    and call process::exit(1) if it does. The run function doesn’t return a value 
    that we want to unwrap in the same way that Config::new returns the Config instance.
    Because run returns () in the success case, we only care about detecting an error,
    so we don’t need unwrap_or_else to return the unwrapped value because it would only be ().    
    */
    if let Err(e) =load_file(config){
        println!("Application error: {}",e);
        process::exit(1);
    }

}

