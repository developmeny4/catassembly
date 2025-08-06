use std::iter::Peekable;
use crate::parser;
use serde_json::json;

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

#[derive(Debug)]
pub enum Actions {
    Log(String),
    Warn(String),
    Error(String),
    Wait(String),
    IfEqual(String, String, Vec<Actions>),
    IfNotEqual(String, String, Vec<Actions>),
    IfGreater(String, String, Vec<Actions>),
    IfLower(String, String, Vec<Actions>),
    IfContains(String, String, Vec<Actions>),
    IfNotContains(String, String, Vec<Actions>),
    IfAND(String, String, Vec<Actions>),
    IfOR(String, String, Vec<Actions>),
    IfNOR(String, String, Vec<Actions>),
    IfXOR(String, String, Vec<Actions>),
    RepeatTimes(String, Vec<Actions>),
    RepeatForever(Vec<Actions>),
    Break,
    DeleteVariable(String),
    SetVariable(String, String),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
    Exponentiation(String, String),
    Modulo(String, String),
    Round(String),
    Floor(String),
    Ceil(String),
    RunMathFunction(String, String),
    RandomIntBetween(String, String, String)
}

fn fnv1a_hash(s: &str) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET;
    for b in s.bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

pub struct EventNode {
    eventString: Events,
    Actions: Vec<Actions>,
    localVariables: Vec<u64>
}

pub struct CompileTimeAppIR {
    events: Vec<EventNode>,
    globals: Vec<u64>
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

fn builtin_log(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<Actions> {
    let args = parse_args(iter);
    let mut code: Vec<Actions> = Vec::new();

    if args.len() != 1 || iter.next() != Some(parser::Token::Semicolon) { panic!("log takes one arg"); }
    
    code.push(Actions::Log(args[0].clone()));
    code
}

fn builtin_warn(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<Actions> {
    let args = parse_args(iter);
    let mut code: Vec<Actions> = Vec::new();

    if args.len() != 1 || iter.next() != Some(parser::Token::Semicolon) { panic!("warn takes one arg"); }
    
    code.push(Actions::Warn(args[0].clone()));
    code
}

fn builtin_err(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<Actions> {
    let args = parse_args(iter);
    let mut code: Vec<Actions> = Vec::new();
    
    if args.len() != 1 || iter.next() != Some(parser::Token::Semicolon) { panic!("err takes one arg"); }
    
    code.push(Actions::Error(args[0].clone()));
    code
}

fn builtin_set(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<Actions> {
    let args = parse_args(iter);
    let mut code: Vec<Actions> = Vec::new();

    if args.len() != 2 || iter.next() != Some(parser::Token::Semicolon) { panic!("set takes two args"); }
    
    code.push(Actions::SetVariable(args[0].clone(), args[1].clone()));
    code
}

fn builtin_wait(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<Actions> {
    let args = parse_args(iter);
    let mut code: Vec<Actions> = Vec::new();

    if args.len() != 1 || iter.next() != Some(parser::Token::Semicolon) { panic!("wait takes one arg"); }
    
    code.push(Actions::Wait(args[0].clone()));
    code
}

fn builtin_add(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<Actions> {
    let args = parse_args(iter);
    let mut code: Vec<Actions> = Vec::new();

    if args.len() != 2 || iter.next() != Some(parser::Token::Semicolon) { panic!("add takes two args"); }
    
    code.push(Actions::Add(args[0].clone(), args[1].clone()));
    code
}

fn builtin_sub(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<Actions> {
    let args = parse_args(iter);
    let mut code: Vec<Actions> = Vec::new();

    if args.len() != 2 || iter.next() != Some(parser::Token::Semicolon) { panic!("sub takes two args"); }
    
    code.push(Actions::Subtract(args[0].clone(), args[1].clone()));
    code
}

fn parse_base(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<Actions> {
    let code: Vec<Actions> = match iter.next() {
        Some(parser::Token::Word(val)) if val == "log".to_string() =>
            builtin_log(iter),
        Some(parser::Token::Word(val)) if val == "warn".to_string() =>
            builtin_warn(iter),
        Some(parser::Token::Word(val)) if val == "err".to_string() =>
            builtin_err(iter),
        Some(parser::Token::Word(val)) if val == "wait".to_string() =>
            builtin_wait(iter),
        Some(parser::Token::Word(val)) if val == "set".to_string() =>
            builtin_set(iter),
        Some(parser::Token::Word(val)) if val == "add".to_string() =>
            builtin_add(iter),
        Some(parser::Token::Word(val)) if val == "sub".to_string() =>
            builtin_sub(iter),
        Some(parser::Token::Word(val)) if val == "loop".to_string() =>
            vec![Actions::RepeatForever(parse_inside(iter))],
        Some(parser::Token::Word(val)) if val == "repeat".to_string() =>
            vec![Actions::RepeatTimes(parse_single_arg(iter), parse_inside(iter))],
        Some(parser::Token::Word(val)) if val == "break".to_string() =>
            if iter.next() == Some(parser::Token::Semicolon) { vec![Actions::Break] }
            else { panic!("stop and put the semicolon in front of break"); },
        _ => panic!("fym i didn't implement this function")
    };
    code
}

fn parse_inside(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> Vec<Actions> {
    let mut code: Vec<Actions> = Vec::new();

    if iter.peek() == Some(&parser::Token::LeftBrace) {
        iter.next();
        while let Some(token) = iter.peek() {
            if *token == parser::Token::RightBrace { iter.next(); break; }
            else {
                code.append(&mut parse_base(iter));
            }
        }
    } else { code.append(&mut parse_base(iter)); }
    
    code
}

fn parse_event(iter: &mut Peekable<impl Iterator<Item = parser::Token>>) -> EventNode {
    let event_token = match iter.next() {
        Some(parser::Token::Word(val)) => val,
        _ => panic!("Expected event name as word"),
    };

    let args = parse_args(iter);

    let eventString = match (event_token.as_str(), args.len()) {
        ("WhenWebsiteLoaded", 0) => Events::WhenWebsiteLoaded,
        ("WhenButtonPressed", 1) => Events::WhenButtonPressed(args[0].clone()),
        _ => panic!("Unknown event or wrong number of args"),
    };

    if iter.next() != Some(parser::Token::Colon) {
        panic!("WHERE IS THE COLON DUDE WTF");
    }

    let actions = parse_inside(iter);

    EventNode {
        eventString,
        Actions: actions,
        localVariables: Vec::new(), // init with nothing for now
    }
}

pub fn parse_code(code: Vec<parser::Token>) -> CompileTimeAppIR {
    let mut app = CompileTimeAppIR {
        events: Vec::new(),
        globals: Vec::new()
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

fn jsonify_action(action: &Actions) -> Vec<serde_json::Value> {
    match action {
        Actions::Log(txt) => vec![json!({
            "id": "0",
            "t": "0",
            "text": ["Log", {
                "value": txt,
                "t": "any"
            }]
        })],
        Actions::Warn(txt) => vec![json!({
            "id": "1",
                "t": "0",
                "text": ["Warn", {
                    "value": txt,
                    "t": "any"
                }]
            })],
        Actions::Error(txt) => vec![json!({
                "id": "2",
                "t": "0",
                "text": ["Error", {
                    "value": txt,
                    "t": "any"
                }]
            })],
        Actions::Wait(txt) => vec![json!({
                "id": "3",
                "t": "0",
                "text": ["Wait", {
                    "value": txt,
                    "t": "number"
                }]
            })],
        Actions::SetVariable(var, to) => vec![json!({
                "id": "11",
                "t": "0",
                "text": ["Set", {
                    "value": var,
                    "l": "variable",
                    "t": "string"
                }, "to", {
                    "value": to,
                    "l": "any",
                    "t": "string"
                }]
            })],
        Actions::Add(var, by) => vec![json!({
                "id": "12",
                "t": "0",
                "text": ["Increase", {
                    "value": var,
                    "l": "variable",
                    "t": "string"
                }, "by", {
                    "value": by,
                    "t": "number"
                }]
            })],
        Actions::Subtract(var, by) => vec![json!({
                "id": "13",
                "t": "0",
                "text": ["Decrease", {
                    "value": var,
                    "l": "variable",
                    "t": "string"
                }, "by", {
                    "value": by,
                    "t": "number"
                }]
            })],
        Actions::RepeatForever(actions) => {
            let mut out = vec![
                json!({
                    "id": "23",
                    "text": ["Repeat forever"], // haha simplest text surely it can't be that hard
                    "t": "0"                    // to implement this
                })
            ];
            out.extend(actions.iter().flat_map(jsonify_action));
            out.push(json!({"id": "25", "text": ["end"], "t": "0"})); // nvm it was actually pretty
            out                                                       // simple
        },
        Actions::RepeatTimes(var, actions) => {
            let mut out = vec![
                json!({
                    "id": "22",
                    "text": ["Repeat", {
                        "value": var,
                        "t": "number"
                    }, "times"],
                    "t": "0"
                })
            ];
            out.extend(actions.iter().flat_map(jsonify_action));
            out.push(json!({"id": "25", "text": ["end"], "t": "0"}));
            out
        },
        Actions::Break => vec![json!({
            "id": "24",
            "text": ["Break"]
        })],
        _ => panic!("action not supported yet")
    }
}

// i hate my life
pub fn jsonify(app: CompileTimeAppIR) -> serde_json::Value {
    json!([{
        "class": "script",
        "content": app.events.iter().enumerate().map(|(i, event)| {
            // these can not go in the json directly for some reason
            let (text, id) = match &event.eventString {
                Events::WhenWebsiteLoaded => (
                    json!(["When website loaded..."]),
                    json!("0")
                ),
                Events::WhenButtonPressed(button) => (
                    json!([
                        "When",
                        {"value": button, "l": "button", "t": "object"},
                        "pressed..."
                    ]),
                    json!("1")
                ),
                _ => panic!("idk what this event means in json")
            };

            let actions: Vec<serde_json::Value> = event.Actions
                .iter()
                .enumerate()
                .flat_map(|(j, act)| jsonify_action(act)) // <- this must return Vec<_>
                .collect();


            json!({
                "id": id,
                "x": (4780 + (i * 410)).to_string(),
                "y": "4780",
                "width": "400",
                "text": text,
                "actions": actions
            })
        }).collect::<Vec<_>>()
    }])
}
