use std::io::Write;

use zeroql::parser::Parser;

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
        let ast = parser.parse_op().unwrap();

        println!("ast: {:#?}", ast);
    }
}
