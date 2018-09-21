mod aqua;
mod ast;

fn main() {
    let parser = aqua::RegisterParser::new().parse("register RCC {
        u8 {
            REG1<1:0>;
            REG2<3:2>;
            REG3<5:4>;
            REG4<7:6>;
        }
        u16 {
            REG1<1:0>;
            REG2<3:2>;
            REG3<5:4>;
            REG4<7:6>;
        }
        u32 {
            REG1<1:0>;
            REG2<3:2>;
            REG3<5:4>;
            REG4<7>;
        }
    }").unwrap();
    println!("{:#?}", parser);
}