use std::env;
use std::fs;

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
    let config= parse_config(&args);
    println!("Searching for {}", config.query);
    println!("File: {}",config.file);

    let contents = fs::read_to_string(config.file)
                    .expect("Something wrong when read file");
    println!("With text: \n{}",{contents});

}

struct Config{
    query: String,
    file: String
}

fn parse_config(args: &[String]) -> Config{
    //Declare variable to get the cli arguments vector value
    let query = args[1].clone();
    let file = args[2].clone();

    Config{query, file}

}
