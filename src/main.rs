mod parser;
mod ir;
use serde_json::json;

fn main() {
    println!("CatAssembly Transpiler.");
    /*let mystr = "
        
        event WhenWebsiteLoaded(): log(\"hello wordl!\");
        event WhenWebsiteLoaded(): {
            wait(1); err(\"startin the day with an error\");
        }
        event WhenWebsiteLoaded(): {
            set(variable, \"if you can see this text variables are FUNCTIONAL\");
            log(variable);
        }
        ";*/

    let mystr = "event WhenWebsiteLoaded(): repeat 3 {
        wait(1);
        log(\"do repeattimes work???\");
        break;
    }";
    let tokenized = parser::tokenize(mystr);
    println!("{} tokens in code", tokenized.len());
   

    let compiled = ir::parse_code(tokenized);

    let jsonified = ir::jsonify(compiled);
    let stringified = serde_json::to_string(&jsonified).unwrap();
    
    println!("\ncheck the syntax before programming. there might be unexpected cases in this version. the syntax is simple anyways");

    println!("\n !!! THE TRANSPILER DOESN'T CHECK IF THE SCRIPT YOU HAVE CREATED WILL ERROR OUT AT RUNTIME !!! \n");
    println!("{}\n\n", stringified);
}
