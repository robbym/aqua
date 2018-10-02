extern crate llvm_sys;

mod aqua;
mod ast;
mod llvm;

fn main() {
    let module = llvm::Module::new("TestModule");

    let entry = module.add_function(
        "main", 
        &llvm::FunctionType::new(
            &mut [],
            llvm::Type::Int32
        )
    );

    let builder = entry.append_block().builder();
    builder.build_ret(&llvm::Value::const_int32(0));

    println!("{}", module);
}