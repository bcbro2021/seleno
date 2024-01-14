// toky/lib.rs
use regex::Regex;

#[derive(Debug, Clone)] 
pub struct Token {
    pub t: String,
    pub val: String,
}


pub const CATHU_KEYWORD: &str = "seleno";
pub const ASSIGNMENT_KEYWORD: &str = "put";
pub const IDENTIFIER: &str = "Identifier";
pub const UNKNOWN: &str = "Unknown";

// std
pub const SAY_KEYWORD: &str = "say";
pub const LISTEN_KEYWORD: &str = "listen";
pub const POCKET_KEYWORD: &str = "pocket";
pub const ACQUIRE_KEYWORD: &str = "acquire";

pub const READ_KEYWORD: &str = "read";
pub const WRITE_KEYWORD: &str = "write";

// loop
pub const REPEAT_KEYWORD: &str = "repeat";
pub const END_KEYWORD: &str = "end";

// data types
pub const STRING_LITERAL: &str = "StringLiteral";
pub const NUMBER_LITERAL: &str = "NumberLiteral";

pub fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let for_re = format!(r#"{}|pocket|say|listen|repeat|acquire|read|write|\bput\b|"[^"]*"|\b\d+\b|[a-zA-Z_]\w*|[-+*/]|\)\("#,CATHU_KEYWORD);
    let re = Regex::new(for_re.as_str()).unwrap();

    for capture in re.captures_iter(input) {
        if let Some(value) = capture.get(0) {
            let token = value.as_str();
            let t_type = match token {
                CATHU_KEYWORD | POCKET_KEYWORD | SAY_KEYWORD | LISTEN_KEYWORD | ASSIGNMENT_KEYWORD | REPEAT_KEYWORD | END_KEYWORD | ACQUIRE_KEYWORD| READ_KEYWORD | WRITE_KEYWORD | "+" | "-" | "*" | "/" | ")" | "(" => {
                    token
                }
                s if s.starts_with('"') && s.ends_with('"') => STRING_LITERAL,
                s if s.chars().all(|c| c.is_digit(10)) => NUMBER_LITERAL,
                s => {
                    if s.chars().next().map_or(false, |c| c.is_alphabetic()) {
                        IDENTIFIER
                    } else {
                        UNKNOWN
                    }
                }
            };
            tokens.push(Token {
                t: t_type.to_string(),
                val: token.to_string(),
            });
        }
    }

    tokens
}