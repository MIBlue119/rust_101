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
    let query = &args[1];
    let file = &args[2];
    println!("Searching for {}", query);
    println!("File: {}",file);

    let contents = fs::read_to_string(file)
                    .expect("Something wrong when read file");
    println!("With text: \n{}",{contents});

}
