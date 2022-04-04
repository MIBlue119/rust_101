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

    for line in search(&config.query,&contents){
        println!("{}",line);
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a> (query:&str, contents:&'a str)->Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}