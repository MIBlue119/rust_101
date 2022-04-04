use std::error::Error;
use std::fs;


pub struct Config{
    pub query: String,
    pub file: String
}

impl Config{
    pub fn new(args: &[String])-> Result<Config, &'static str>{
        if args.len() <3 {
            return Err("Not enough arguments.")
        }
        //Declare variable to get the cli arguments vector value
        let query = args[1].clone();
        let file = args[2].clone();

        Ok(Config{query, file})

    }
}

pub fn load_file(config: Config) -> Result<(), Box<dyn Error> >{
    let contents = fs::read_to_string(config.file)?;
    println!("With text: \n{}",{contents});
    Ok(())
}