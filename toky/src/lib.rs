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

pub const SAY_KEYWORD: &str = "say";
pub const POCKET_KEYWORD: &str = "pocket";
pub const REPEAT_KEYWORD: &str = "repeat";

pub const IF_KEYWORD: &str = "if";
pub const THEN_KEYWORD: &str = "then";
pub const END_KEYWORD: &str = "end";

pub const STRING_LITERAL: &str = "StringLiteral";
pub const NUMBER_LITERAL: &str = "NumberLiteral";

pub fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let re = Regex::new(r#"cathu|pocket|say|repeat|\bput\b|"[^"]*"|\b\d+\b|[a-zA-Z_]\w*|[-+*/]|\)\("#).unwrap();

    for capture in re.captures_iter(input) {
        if let Some(value) = capture.get(0) {
            let token = value.as_str();
            let t_type = match token {
                CATHU_KEYWORD | POCKET_KEYWORD | SAY_KEYWORD | ASSIGNMENT_KEYWORD | REPEAT_KEYWORD | END_KEYWORD | "+" | "-" | "*" | "/" | ")" | "(" => {
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