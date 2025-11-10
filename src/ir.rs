use std::iter::Peekable;
use crate::parser;
use serde_json::json;

    /*
    pub enum Events {
        DefineFunction(u64, u64), // 1st arg being hashed id, 2nd arg being arg count
        WhenWebsiteLoaded,
        WhenButtonPressed(String),
        WhenKeyPressed(String),
        WhenMouseEntersObject(String),
        WhenMouseLeavesObject(String),
        WhenDonationBought(String),
        WhenInputSubmitted(String),
        WhenMessageReceived
    }
    */

pub struct EventNode {
    eventString: String,
    code: serde_json::Value
}

pub struct CompileTimeAppIR {
    events: Vec<serde_json::Value>
}

fn parse_single_arg(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> String {
    match iter.next() {
        Some(parser::Token::StringLiteral(s)) => s,
        Some(parser::Token::Number(n)) => n.to_string(),
        Some(parser::Token::Word(w)) => {
            if w == "object" {
                match iter.next() {
                    Some(parser::Token::StringLiteral(s)) => s,
                    Some(parser::Token::Word(s)) => format!("{{{}}}", s),
                    other => panic!("expected name after 'object'"),
                }
            } else {
                format!("{{{}}}", w)
            }
        }
        other => panic!("invalid function argument"),
    }
}

fn parse_args(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<String> {
    if iter.next() != Some(parser::Token::LeftParen) {
        panic!("these ain't function args what you on");
    }

    let mut args = Vec::new();
    let mut expecting_arg = true;

    while let Some(token) = iter.peek() {
        match token {
            parser::Token::RightParen => {
                iter.next();
                break;
            }

            parser::Token::Comma => {
                iter.next(); // skip comma
                expecting_arg = true;
            }

            _ if expecting_arg => {
                args.push(parse_single_arg(iter));
                expecting_arg = false;
            }

            _ => panic!("unexpected token in function args"),
        }
    }

    args
}

fn parse_func(name: &str, iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> serde_json::Value {
    let args = parse_args(iter);
    if iter.next() != Some(parser::Token::Semicolon) {
        panic!("expected semicolon after function call");
    }

    // look up the template
    let template = parser::load_templates();
    parser::substitute_args(&template.actions[name], &args)
}

fn parse_base(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> serde_json::Value {
    match iter.next() {
        Some(parser::Token::Word(func_name)) => {
            parse_func(&func_name, iter)
        }
        _ => panic!("unexpected error while parsing a function or sm"),
    }
}

fn parse_inside(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> serde_json::Value {
    let mut code: Vec<serde_json::Value> = Vec::new();

    if iter.peek() == Some(&parser::Token::LeftBrace) {
        iter.next(); // skip brace
        while let Some(token) = iter.peek() {
            if *token == parser::Token::RightBrace {
                iter.next(); // skip brace
                break;
            } else {
                // parse_base now returns a single serde_json::Value
                code.push(parse_base(iter));
            }
        }
    } else {
        code.push(parse_base(iter));
    }

    serde_json::json!(code)
}

fn parse_event(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> serde_json::Value {
    let event_token = match iter.next() {
        Some(parser::Token::Word(name)) => name,
        _ => panic!("Expected event name as word"),
    };

    let args = parse_args(iter);

    if iter.next() != Some(parser::Token::Colon) {
        panic!("WHERE IS THE COLON DUDE WTF");
    }

    let actions_json = parse_inside(iter);

    // get template
    let template = parser::load_templates();
    let mut code = parser::substitute_args(&template.events[&event_token], &args);

    code["actions"] = actions_json;

    serde_json::json!(code)
}

pub fn parse_code(code: Vec<parser::Token>) -> CompileTimeAppIR {
    let mut app = CompileTimeAppIR {
        events: Vec::new()
    };
    let mut code_iter = code.into_iter().peekable();
    
    while let Some(token) = code_iter.next() {
        match token {
            parser::Token::Word(val) if val == "event".to_string() =>
                app.events.push(parse_event(&mut code_iter)),
            _ => panic!("can NOT parse this dumbass keyword")
        }
    }
    app
}

// i hate my life
pub fn jsonify(app: CompileTimeAppIR) -> serde_json::Value {
    json!([{
        "class": "script",
        "content": serde_json::json!(app.events)
    }])
}
