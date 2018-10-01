extern crate llvm_sys;

mod aqua;
mod ast;
mod llvm;

fn main() {
    let module = llvm::Module::new("TestModule");
    let fadd = module.add_function(
        "fadd", 
        &llvm::FunctionType::new(
            &mut [llvm::Type::Float64, llvm::Type::Float64],
            llvm::Type::Float64
        )
    );
    let builder = fadd.append_block().builder();
    builder.test_add(&fadd);

    println!("{}", module);
}