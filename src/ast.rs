#[derive(Debug)]
pub enum NumTypeAST {
    U8,
    U16,
    U32,
    U64
}

#[derive(Debug)]
pub struct BitFieldAST {
    pub name: String,
    pub lo: u8,
    pub hi: u8
}

#[derive(Debug)]
pub struct RegisterFieldAST {
    pub field_type: NumTypeAST,
    pub bit_fields: Vec<BitFieldAST>
}

#[derive(Debug)]
pub struct RegisterAST {
    pub name: String,
    pub register_fields: Vec<RegisterFieldAST>
}