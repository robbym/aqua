use std::collections::HashMap;
use ast::*;

pub trait Traversal {
    fn walk_function(&mut self, function: &FunctionAST) -> Result<(), ()>;
    fn walk_expression(&mut self, expression: &ExpressionAST, expected: NumTypeAST) -> Result<NumTypeAST, ()>;
    fn walk_function_call(&mut self, function_call: &FunctionCallAST) -> Result<(), ()>;
}

struct FunctionTCK {
    name: String,
    ret_type: NumTypeAST,
    param_type: Vec<(NumTypeAST, String)>,
}

pub struct TypeCheckContext {
    outer_scope: Option<Box<TypeCheckContext>>,
    vars: HashMap<String, NumTypeAST>,
    functions: HashMap<String, ()>
}

impl TypeCheckContext {
    pub fn new() -> TypeCheckContext {
        TypeCheckContext {
            outer_scope: None,
            vars: HashMap::new(),
            functions: HashMap::new()
        }
    }
}

impl Traversal for TypeCheckContext {
    fn walk_function(&mut self, function: &FunctionAST) -> Result<(), ()> {
        for var in &function.param_type {
            self.vars.insert(var.1.clone(), var.0);
        }

        for stmt in &function.stmts {
            match stmt {
                StatementAST::Assignment(assignment) => {
                    let expected_type: NumTypeAST;
                    let evaluated_type: NumTypeAST;

                    let eval_type = if let Some(type_) = assignment.type_ {
                        expected_type = type_;
                        evaluated_type = self.walk_expression(&*assignment.value, type_)?;
                        self.vars.insert(assignment.name.clone(), type_);
                    } else {
                        let type_ = self.vars.get(&assignment.name).expect("TODO: Return error for using variable before declaration").clone();
                        expected_type = type_;
                        evaluated_type = self.walk_expression(&*assignment.value, type_)?;
                    };

                    if expected_type != evaluated_type {
                        panic!("TODO: Return error for type mismatch")
                    }
                }
            }
        }

        Ok(())
    }

    fn walk_expression(&mut self, expression: &ExpressionAST, expected: NumTypeAST) -> Result<NumTypeAST, ()> {
        Ok(expected)
    }

    fn walk_function_call(&mut self, function_call: &FunctionCallAST) -> Result<(), ()> {
        Ok(())
    }
}