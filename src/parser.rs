use std::iter::Peekable;
use std::str::Chars;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{collections::HashMap, fs};
use once_cell::sync::OnceCell;

#[derive(PartialEq)]
pub enum Token {
    Number(f64),
    StringLiteral(String),
    Word(String),
    LeftBrace,
    RightBrace,
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Ampersand
}

fn parse_number(first: char, chars: &mut Peekable<Chars<'_>>) -> Token {
    let mut num_str = first.to_string(); // start with the first digit
    let mut seen_dot = false;

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                num_str.push(ch);
                chars.next();
            }
            '.' if !seen_dot => {
                seen_dot = true;
                num_str.push(ch);
                chars.next();
            }
            _ => break
        }
    }

    let parsed = num_str.parse::<f64>().unwrap_or_else(|_| {
        panic!("idk what a '{}' is. ain't no number for sure", num_str)
    });

    Token::Number(parsed)
}

fn parse_string(quote: char, chars: &mut Peekable<Chars<'_>>) -> Token {
    let mut content = String::new();
    let mut slash = false;

    while let Some(&ch) = chars.peek() {
        chars.next();

        if slash {
            content.push(ch) // THIS IS TEMPORARY!!!!!!!!!!!!!!!!!
        } else if ch == '\\' {
            slash = true;
        } else if ch == quote {
            break;
        } else {
            content.push(ch);
        }
    }
    Token::StringLiteral(content)
}

#[derive(serde::Deserialize)]
pub struct TemplateFile {
    pub actions: HashMap<String, Value>,
    pub events: HashMap<String, Value>,
}

pub fn load_templates() -> &'static TemplateFile {
    static CACHE: OnceCell<TemplateFile> = OnceCell::new();

    CACHE.get_or_init(|| {
        let data = std::fs::read_to_string("templates.json")
            .expect("failed to read template file");
        serde_json::from_str(&data)
            .expect("invalid json template")
    })
}

pub fn substitute_args(template: &Value, args: &[String]) -> Value {
    match template {
        Value::String(s) => {
            let mut result = s.clone();
            for (i, arg) in args.iter().enumerate() {
                let placeholder = format!("=arg{}=", i + 1);
                result = result.replace(&placeholder, arg);
            }
            Value::String(result)
        }
        Value::Array(arr) => {
            Value::Array(arr.iter().map(|v| substitute_args(v, args)).collect())
        }
        Value::Object(obj) => {
            let mut new_obj = serde_json::Map::new();
            for (k, v) in obj {
                new_obj.insert(k.clone(), substitute_args(v, args));
            }
            Value::Object(new_obj)
        }
        _ => template.clone(),
    }
}

fn parse_word(firstchar: char, chars: &mut Peekable<Chars<'_>>) -> Token {
    let mut word = firstchar.to_string();
    
    while let Some(&ch) = chars.peek() {
        if ch.is_alphanumeric() || ch == '_' ||
            ch == '-' || ch == '!' || ch == '.' 
            { word.push(ch) }
        else { break };

        chars.next();
    }

    Token::Word(word)
}

pub fn tokenize(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' | '\n' | '\t' => continue,
            '"' | '\'' => tokens.push(parse_string(ch, &mut chars)),
            '0'..='9' => tokens.push(parse_number(ch, &mut chars)),
            'a'..='z' | 'A'..='Z' => tokens.push(parse_word(ch, &mut chars)),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            ';' => tokens.push(Token::Semicolon),
            ':' => tokens.push(Token::Colon),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            ',' => tokens.push(Token::Comma),
            _ => continue
        }
    }
    tokens
}


