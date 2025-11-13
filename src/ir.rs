use std::iter::Peekable;
use crate::parser;
use serde_json::json;

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
        other => panic!("invalid block argument"),
    }
}

fn parse_args(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<String> {
    if iter.next() != Some(parser::Token::LeftParen) {
        panic!("block args expected");
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

            _ => panic!("unexpected token in block args"),
        }
    }

    args
}

fn parse_func(name: &str, iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<serde_json::Value> {
    // i am sorry for all the horrible code i have written in this function and overall file

    let args = parse_args(iter);
    let mut actions: Vec<Vec<serde_json::Value>> = Vec::new();

    let template = parser::load_templates();
    let funcEntry = &template.actions[name];

    if args.len() == funcEntry["argc"] {
        if funcEntry["if_variant"] == true {
            if iter.next() != Some(parser::Token::Colon) {
                panic!("expected colon after if variant")
            }

            actions.push(parse_base(iter));

            match iter.peek() {
                Some(parser::Token::Word(val)) if val == "else" => {
                    iter.next();
                    if iter.next() != Some(parser::Token::Colon) {
                        panic!("expected colon after else keyword");
                    }
                    actions.push(vec![parser::substitute_args(&template.actions["else"]["code"], &Vec::new())]);
                    actions.push(parse_base(iter));
                    
                }
                _ => (), // do nothing
            }

            actions.push(vec![parser::substitute_args(&template.actions["end"]["code"], &Vec::new())]);
            return actions.into_iter().flatten().collect()

        } else if funcEntry["loop"] == true {
            if iter.next() != Some(parser::Token::Colon) {
                panic!("expected colon after loop declaration")
            }

            actions.push(parse_inside(iter));
            actions.push(vec![parser::substitute_args(&template.actions["end"]["code"], &Vec::new())]);
            return actions.into_iter().flatten().collect()

        } else {
            if iter.next() != Some(parser::Token::Semicolon) {
                panic!("expected semicolon after function call");
            }
            
            return vec![parser::substitute_args(&funcEntry["code"], &args)]
        }
    } else {
        panic!("less/more args than what the function wants");
    }

}

fn parse_base(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<serde_json::Value> {
    match iter.next() {
        Some(parser::Token::Word(func_name)) => {
            parse_func(&func_name, iter)
        }
        _ => panic!("unexpected token while parsing a block"),
    }
}

fn parse_inside(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<serde_json::Value> {
    let mut code: Vec<Vec<serde_json::Value>> = Vec::new();

    if iter.peek() == Some(&parser::Token::LeftBrace) {
        iter.next(); // skip brace
        while let Some(token) = iter.peek() {
            if *token == parser::Token::RightBrace {
                iter.next(); // skip brace
                break;
            } else {
                // parse_base now returns a Vec<serde_json::Value>
                code.push(parse_base(iter));
            }
        }
    } else {
        code.push(parse_base(iter));
    }

    code.into_iter().flatten().collect()
}

fn parse_event(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> serde_json::Value {
    let event_token = match iter.next() {
        Some(parser::Token::Word(name)) => name,
        _ => panic!("expected event name as word"),
    };

    let args = parse_args(iter);

    if iter.next() != Some(parser::Token::Colon) {
        panic!("missing colon after event declaration");
    }

    let actions_json = serde_json::json!(parse_inside(iter));

    // get template
    let template = parser::load_templates();
    if args.len() != template.events[&event_token]["argc"] {
        panic!("less/more args than what the event wants");
    }

    let mut code = parser::substitute_args(&template.events[&event_token]["code"], &args);

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
            _ => panic!("unexpected keyword while parsing file (\"event\" word expected)")
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
