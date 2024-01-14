use toky::{self, tokenizer, Token, CATHU_KEYWORD, SAY_KEYWORD, IDENTIFIER, POCKET_KEYWORD, ASSIGNMENT_KEYWORD, STRING_LITERAL, NUMBER_LITERAL, REPEAT_KEYWORD, END_KEYWORD, LISTEN_KEYWORD, ACQUIRE_KEYWORD, READ_KEYWORD, WRITE_KEYWORD};
use std::{io::{self, Write, BufRead, BufReader}, collections::HashMap, fs::{self, OpenOptions}, path::PathBuf};
use std::fs::File;
use eval::eval;
use std::process;

// lib stuff
pub fn read_lines(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}

fn read_file_to_string(file_path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn write_to_file(file_path: &str, content: &str) {
    // Convert the file path to a PathBuf
    let path = PathBuf::from(file_path);

    // Check if the file already exists
    if path.exists() && !path.is_file() {
        process::exit(-1);
    }

    // Open the file in append mode, creating it if it doesn't exist
    let mut file = OpenOptions::new().write(true).create(true).open(&path).unwrap();

    // Write the content to the file
    file.write(content.as_bytes()).unwrap();
}

pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Flush the buffer to ensure the prompt is displayed
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// std functions
pub fn process_variables(tokens: &[Token], vars: &mut HashMap<String, String>) {
    if tokens.len() >= 5 {
        if tokens[0].t == CATHU_KEYWORD
            && tokens[1].t == POCKET_KEYWORD
            && tokens[2].t == ASSIGNMENT_KEYWORD
            && tokens[3].t == IDENTIFIER
        {
            if tokens[4].t == STRING_LITERAL {
                vars.insert(tokens[3].val.clone(), tokens[4].val.clone().replace('"', ""));
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

pub fn process_input(tokens: &Vec<Token>, vars: &mut HashMap<String, String>) {
    if tokens.len() >= 4 {
        if tokens[0].t == CATHU_KEYWORD
            && tokens[1].t == LISTEN_KEYWORD
            && tokens[2].t == IDENTIFIER
            && tokens[3].t == STRING_LITERAL
        {
            let response = input(&tokens[3].val.replace('"', ""));
            vars.insert(tokens[2].val.clone(), response);
        }
    }
}

pub fn process_read(tokens: &Vec<Token>, vars: &mut HashMap<String, String>) {
    if tokens.len() >= 4 {
        if tokens[0].t == CATHU_KEYWORD
            && tokens[1].t == READ_KEYWORD
            && tokens[2].t == IDENTIFIER
            && tokens[3].t == STRING_LITERAL
        {
            let path = &tokens[3].val.replace('"', "");
            let data = read_file_to_string(path.as_str());
            if let Ok(final_data) = data {
                vars.insert(tokens[2].val.clone(), final_data);
            } else {
                println!("{} says: you stoopid, {} prolly doesnt exist.", CATHU_KEYWORD, &tokens[3].val);
                process::exit(-1);
            }
        }
    }
}

pub fn process_write(tokens: &Vec<Token>, vars: &mut HashMap<String, String>) {
    if tokens.len() >= 4 {
        if tokens[0].t == CATHU_KEYWORD
            && tokens[1].t == WRITE_KEYWORD
            && tokens[2].t == IDENTIFIER
            && tokens[3].t == STRING_LITERAL
        {
            let path = &tokens[3].val.replace('"', "");
            let data = vars.get(&tokens[2].val).unwrap();
            write_to_file(path, data);
        }
    }
}

pub fn process_import(tokens: &Vec<Token>, mut vars: &mut HashMap<String, String>) {
    if tokens.len() >= 3 {
        if tokens[0].t == CATHU_KEYWORD
            && tokens[1].t == ACQUIRE_KEYWORD
            && tokens[2].t == STRING_LITERAL
        {
            let path = &tokens[2].val.replace('"', "");
            let mut new_path = path.clone();
            new_path.push_str(".tt");

            let lines = read_lines(new_path.as_str());
            if let Ok(new_lines) = lines {
                let mut i = 0;
                while i < new_lines.len() {
                    let text = &new_lines[i];
                    let itokens = tokenizer(text);

                    // process everything
                    run_program(i, &itokens, &mut vars, &new_lines);
                    i += 1;
                }
            } else {
                println!("{} says: you stoopid, the import of {} failed cause \nit prolly doesnt exist.", CATHU_KEYWORD, format!("'{}'",new_path));
                process::exit(-1);
            }
            
        }
    }
}

// main functions to handle program
pub fn process_loop(mut i: usize, tokens: &Vec<Token>, vars: &mut HashMap<String, String>, lines: &Vec<String>) {
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
                process_std(&inner_tokens, vars);
            }
        }

        // Move i to the next line after the loop
        i = j + 1;
    }
}

pub fn process_std(tokens: &Vec<Token>, vars: &mut HashMap<String, String>) {
    process_import(tokens, vars);
    process_variables(&tokens, vars);

    process_print(&tokens, &vars);
    process_input(tokens, vars);
    process_read(tokens, vars);
    process_write(tokens, vars);
}

// program executor
pub fn run_program(i:  usize,tokens: &Vec<Token>, mut vars: &mut HashMap<String, String>, lines: &Vec<String>) {
    process_std(&tokens, &mut vars);
    // loop checking
    process_loop(i, &tokens, &mut vars, &lines);
}