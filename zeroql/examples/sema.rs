use std::io::Write;

use zeroql::{parser::Parser, sema::SemanticAnalyzer};

//--------------------------------------------------------------------------------------------------
// Main
//--------------------------------------------------------------------------------------------------

fn main() {
    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut parser = Parser::new(&input, 100);
        let Ok(ast) = parser.parse_program() else {
            continue;
        };

        if let Some(mut ast) = ast {
            let mut sema = SemanticAnalyzer::new(&mut ast);
            match sema.analyze() {
                Ok(_) => println!("Analysis successful: {:#?}", sema.get_ast()),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            println!("Failed to parse input");
        }
    }
}
