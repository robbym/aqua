extern crate llvm_sys;

mod aqua;
mod ast;
mod llvm;

fn main() {
    let res = aqua::ExpressionParser::new().parse("
        -(-cake || bottle) && (a+b > c) && (z<y)
    ");
    println!("{:#?}", res);
}