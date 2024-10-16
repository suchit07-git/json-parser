use std::env;
use std::path::Path;
use std::fs::File;

fn main() {
    println!("A simple JSON Parser!");
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("Please specify the path of the json file\n\
                Usage: cargo run <filepath>");
    } else if args.len() == 2 {
        let file_path = Path::new(&args[1]);
        if !file_path.exists() {
            println!("The specified file doesn't exist!");
        }
        let file = match File::open(&file_path) {
            Err(why) => panic!("Couldn't read {}: {}", file_path.display(), why),
            Ok(file) => file,
        };
    } else {
        panic!("Wrong number of arguments specified!\n\
                Usage: cargo run <filepath>");
    }
}
