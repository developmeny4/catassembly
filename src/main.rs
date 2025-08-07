mod parser;
mod ir;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use serde_json::json;

fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    println!("CatAssembly Transpiler.");
    
    // env::args() includes program name
    if args.len() != 2 { panic!("the transpiler takes one arg") }
    
    let mut file = File::open( args[1].clone() )?;
    let mut filecontents = String::new();
    file.read_to_string(&mut filecontents)?;

    println!("compiling...");

    let tokenized = parser::tokenize(filecontents);
    println!("{} tokens in code", tokenized.len());
   

    let compiled = ir::parse_code(tokenized);

    let jsonified = ir::jsonify(compiled);
    let stringified = serde_json::to_string(&jsonified).unwrap();
    
    println!("\ncheck the syntax before programming. there might be unexpected cases in this version. the syntax is simple anyways");

    println!("\n !!! THE TRANSPILER DOESN'T CHECK IF THE SCRIPT YOU HAVE CREATED WILL ERROR OUT AT RUNTIME !!! \n");
    println!("{}\n\n", stringified);

    Ok(())
}
