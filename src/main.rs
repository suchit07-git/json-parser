use crate::lexer::Lexer;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod lexer;
pub mod tokens;

fn main() {
    println!("A simple JSON Parser!");
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!(
            "Please specify the path of the json file\n\
                Usage: cargo run <filepath>"
        );
    } else if args.len() == 2 {
        let file_path = Path::new(&args[1]);
        if !file_path.exists() {
            println!("The specified file doesn't exist!");
        }
        let mut file = match File::open(file_path) {
            Err(why) => panic!("Couldn't read {}: {}", file_path.display(), why),
            Ok(file) => file,
        };
        let mut json = String::new();
        match file.read_to_string(&mut json) {
            Err(why) => panic!("Couldn't read file {}: {}", file_path.display(), why),
            Ok(_) => println!("Contents of {}: \n{}", file_path.display(), json),
        }
        let mut lex = Lexer::new(json);
        while let Some(token) = lex.next_token() {
            println!("{:?}", token);
        }
    } else {
        panic!(
            "Wrong number of arguments specified!\n\
                Usage: cargo run <filepath>"
        );
    }
}
