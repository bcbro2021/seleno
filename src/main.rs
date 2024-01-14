// main.rs
use toky::{self, tokenizer};
use std::{ collections::HashMap, env};

pub mod base;
use base::*;

fn main() {
    // program stuff
    let mut vars: HashMap<String, String> = HashMap::new();

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let lines = read_lines(args[1].as_str()).unwrap();

        let mut i = 0;
        while i < lines.len() {
            let text = &lines[i];
            let tokens = tokenizer(text);

            // process everything
            run_program(i, &tokens, &mut vars, &lines);
            i += 1;
        }
    } else {
        loop {
            let text = input(">");
            let tokens = tokenizer(&text);
            println!("{:#?}", &vars);

            // process everything
            process_std(&tokens, &mut vars);
        }
    }
    
}