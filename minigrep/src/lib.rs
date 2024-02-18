use std::{ fs,  error::Error, };


pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // pub fn build(args: &[String]) -> Result<Config, &'static str> {

    // }
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok( Config{
            query: query,
            file_path: file_path,
        })
    }

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    println!("content {}", content);

    Ok(())
}


#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}


struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self::most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut
         
    }
    
}