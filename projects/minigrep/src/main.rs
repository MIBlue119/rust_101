use std::env;
use std::fs;
use std::process;

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

    let contents = fs::read_to_string(config.file)
                    .expect("Something wrong when read file");
    println!("With text: \n{}",{contents});

}

struct Config{
    query: String,
    file: String
}

impl Config{
    fn new(args: &[String])-> Result<Config, &'static str>{
        if args.len() <3 {
            return Err("Not enough arguments.")
        }
        //Declare variable to get the cli arguments vector value
        let query = args[1].clone();
        let file = args[2].clone();

        Ok(Config{query, file})

    }
}
