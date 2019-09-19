mod aqua;
mod ast;
mod typecheck;

use typecheck::{Traversal, TypeCheckContext};

fn main() {
    if let Ok(function) = aqua::FunctionParser::new().parse("
        u32 main() {
            u8 my_var = -(-cake || bottle(cake, 1, 2, 3)) && (a+b > c) && (z<y);

            u8 cake = 2;
            cake = my_var;

        }
    ") {
        let mut tc = TypeCheckContext::new();
        tc.walk_function(&function).unwrap();
        println!("Hey!");
    }
}