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


    // let parser = aqua::RegisterParser::new().parse("
    // register RCC {
    //     u8 {
    //         REG1<1:0>;
    //         REG2<3:2>;
    //         REG3<5:4>;
    //         REG4<7:6>;
    //     }
    //     u16 {
    //         REG1<1:0>;
    //         REG2<3:2>;
    //         REG3<5:4>;
    //         REG4<7:6>;
    //     }
    //     u32 {
    //         REG1<1:0>;
    //         REG2<3:2>;
    //         REG3<5:4>;
    //         REG4<7>;
    //     }
    // }").unwrap();
    // println!("{:#?}", parser);