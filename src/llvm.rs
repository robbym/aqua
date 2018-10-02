use std::ffi;
use std::fmt;
use std::ptr;
use std::marker;

use llvm_sys::prelude::*;
use llvm_sys::core;

trait ToCString {
    fn to_cstr(&self) -> ffi::CString;
}

impl ToCString for str {
    fn to_cstr(&self) -> ffi::CString {
        ffi::CString::new(self).unwrap()
    }
}

pub struct Module {
    ptr: LLVMModuleRef
}

impl Module {
    pub fn new(name: &str) -> Module {
        unsafe {
            Module {
                ptr: core::LLVMModuleCreateWithName(name.to_cstr().as_ptr())
            }
        }
    }

    pub fn add_function(&self, name: &str, fn_type: &FunctionType) -> Function {
        unsafe {
            Function {
                ptr: core::LLVMAddFunction(self.ptr, name.to_cstr().as_ptr(), fn_type.ptr),
                module: marker::PhantomData
            }
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unsafe {
            let text = ffi::CStr::from_ptr(core::LLVMPrintModuleToString(self.ptr));
            write!(f, "{}", text.to_string_lossy());
        }
        Ok(())
    }
}

pub struct Function<'a> {
    ptr: LLVMValueRef,
    module: marker::PhantomData<&'a Module>
}

impl<'a> Function<'a> {
    pub fn append_block(&self) -> Block {
        unsafe {
            Block {
                ptr: core::LLVMAppendBasicBlock(self.ptr, "entry".to_cstr().as_ptr()),
                function: marker::PhantomData
            }
        }
    }
}

pub struct FunctionType {
    ptr: LLVMTypeRef
}

impl FunctionType {
    pub fn new(param_types: &mut [Type], ret_type: Type) -> FunctionType {
        let mut param_types: Vec<_> = param_types.iter().map(|x| x.to_llvm_type()).collect();
        unsafe {
            FunctionType {
                ptr: core::LLVMFunctionType(ret_type.to_llvm_type(), param_types.as_mut_ptr(), param_types.len() as u32, 0)
            }
        }
    }
}

pub enum Type {
    Void,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
}

impl Type {
    fn to_llvm_type(&self) -> LLVMTypeRef {
        unsafe {
            match self {
                Type::Void => {core::LLVMVoidType()}
                Type::Int8 => {core::LLVMInt8Type()}
                Type::Int16 => {core::LLVMInt16Type()}
                Type::Int32 => {core::LLVMInt32Type()}
                Type::Int64 => {core::LLVMInt64Type()}
                Type::Float32 => {core::LLVMFloatType()}
                Type::Float64 => {core::LLVMDoubleType()}
            }
        }
    }
}

pub struct Block<'a> {
    ptr: LLVMBasicBlockRef,
    function: marker::PhantomData<&'a Function<'a>>
}

impl<'a> Block<'a> {
    pub fn builder(&self) -> Builder<'a> {
        unsafe {
            let builder = core::LLVMCreateBuilder();
            core::LLVMPositionBuilderAtEnd(builder, self.ptr);
            Builder {
                ptr: builder,
                block: marker::PhantomData
            }
        }
    }
}

pub struct Builder<'a> {
    ptr: LLVMBuilderRef,
    block: marker::PhantomData<&'a Block<'a>>
}

impl<'a> Builder<'a> {
    pub fn build_ret(&self, val: &Value) {
        unsafe {
            core::LLVMBuildRet(self.ptr, val.ptr);
        }
    }
}

pub struct Value {
    ptr: LLVMValueRef
}

impl Value {
    pub fn const_int32(val: u64) -> Value {
        unsafe {
            Value {
                ptr: core::LLVMConstInt(core::LLVMInt32Type(), val, 0)
            }
        }
    }
}