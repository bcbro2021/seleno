use toky::{self, tokenizer, Token, CATHU_KEYWORD, SAY_KEYWORD, IDENTIFIER, POCKET_KEYWORD, ASSIGNMENT_KEYWORD, STRING_LITERAL, NUMBER_LITERAL, REPEAT_KEYWORD, END_KEYWORD};
use std::{io::{self, Write, BufRead, BufReader}, collections::HashMap};
use std::fs::File;
use eval::eval;
use std::process;

pub fn read_lines(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Flush the buffer to ensure the prompt is displayed
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn process_variables(tokens: &[Token], vars: &mut HashMap<String, String>) {
    if tokens.len() >= 5 {
        if tokens[0].t == CATHU_KEYWORD
            && tokens[1].t == POCKET_KEYWORD
            && tokens[2].t == ASSIGNMENT_KEYWORD
            && tokens[3].t == IDENTIFIER
        {
            if tokens[4].t == STRING_LITERAL {
                vars.insert(tokens[3].val.clone(), tokens[4].val.clone());
            }
            else if tokens[4].t == NUMBER_LITERAL {
                if tokens.len() > 5 {
                    let mut exp = "".to_string();
                    for x in 4..tokens.len() {
                        if tokens[x].t != IDENTIFIER {
                            exp.push_str(&tokens[x].val);
                        } else {
                            exp.push_str(&vars[&tokens[x].val]);
                        }
                    }
                    let result = eval(exp.as_str()).unwrap();
                    println!("{}",exp.as_str());
                    vars.insert(tokens[3].val.clone(), result.to_string());
                } else {
                    vars.insert(tokens[3].val.clone(), tokens[4].val.to_string());
                }
            }
        }
    }
}

pub fn process_print(tokens: &Vec<Token>, vars: &HashMap<String, String>) {
    let mut print_confirm = false;
    if tokens.len() >= 3 {
        if tokens[0].t == CATHU_KEYWORD
            && tokens[1].t == SAY_KEYWORD
            && tokens[2].t == IDENTIFIER
        {
            print_confirm = true;
        }
    }

    if print_confirm {
        if let Some(value) = vars.get(&tokens[2].val) {
            println!("{} says {}",CATHU_KEYWORD, value);
        } else {
            println!("{} says Variable '{}' not found",CATHU_KEYWORD, &tokens[2].val);
            process::exit(-1);
        }
    }
}

pub fn run_program(tokens: &Vec<Token>, vars: &mut HashMap<String, String>) {
    process_variables(&tokens, vars);
    process_print(&tokens, &vars);
}

pub fn run_loop(mut i: usize, tokens: &Vec<Token>, vars: &mut HashMap<String, String>, lines: &Vec<String>) {
    if tokens.len() >= 3
        && tokens[0].t == CATHU_KEYWORD
        && tokens[1].t == IDENTIFIER
        && tokens[2].t == REPEAT_KEYWORD
    {
        // getting the pos of the end keyword
        let mut j = i + 1; // Start from the next line
        while j < lines.len() {
            let inner_tokens = tokenizer(&lines[j]);
            if inner_tokens.len() >= 2
                && inner_tokens[0].t == CATHU_KEYWORD
                && inner_tokens[1].t == END_KEYWORD
            {
                break;
            }
            j += 1;
        }

        // actual loop
        while vars.get(&tokens[1].val) == Some(&"1".to_string()) {
            for x in i + 1..j {
                let inner_tokens = tokenizer(&lines[x]);
                run_program(&inner_tokens, vars);
            }
        }

        // Move i to the next line after the loop
        i = j + 1;
    }
}