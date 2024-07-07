use my_parser::Parser;
use wasm_bindgen::prelude::*;

mod my_parser;

// 导入函数
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    // console log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    /// 使用 js 中的自定义的函数
    fn addValue(value: &str);
}

#[derive(Debug, Clone, Copy)]
// 自定义一个结构体可以被 js 访问
#[wasm_bindgen]
pub struct MyStruct {
    // 可以被 js 访问
    pub flag: bool,
    // 不可以被 js 直接访问
    value: i32,
}

// 实现结构体的方法（可以在 js 中调用）
#[wasm_bindgen]
impl MyStruct {
    pub fn new(value: i32) -> MyStruct {
        MyStruct { flag: false, value }
    }
    pub fn get_value(&self) -> i32 {
        self.value
    }
    pub fn add_value(&mut self, v: i32) {
        self.value += v;
        self.add_str();
    }
}

// 实现结构体的方法（不能在 js 中调用）
impl MyStruct {
    pub fn add_str(&self) {
        addValue(self.value.to_string().as_str());
    }
}

// 导出函数
#[wasm_bindgen]
pub fn create_struct() -> MyStruct {
    // Parser::new_inline("sin(x) + 12".to_string());
    MyStruct {
        flag: true,
        value: 42,
    }
}

#[wasm_bindgen]
pub fn parse(input: &str, level: u32) -> String {
    Parser::new_inline(input.to_string()).print(level as usize)
}

// rust 中的测试
#[test]
fn test() {
    let parser = Parser::new_inline(
        "y: \"string \\\\ \\n \\t \\' \\\" $ \"".to_string(),
        // "Plot(((x+0.1)^2)^cos(T)) /* 使用 Shift + 删除 消去一行 */ ".to_string(),
        
    );
    // stdout().write_all(parser.print(5).as_bytes());
    // stdout().write_all(b"\n-------------------------\n");
    // stdout().write_all(parser.print(4).as_bytes());
    // stdout().write_all(b"\n-------------------------\n");
    // stdout().write_all(parser.print(3).as_bytes());
    // stdout().write_all(b"\n-------------------------\n");
    // stdout().write_all(parser.print(2).as_bytes());
    // stdout().write_all(b"\n-------------------------\n");
    // stdout().write_all(parser.print(1).as_bytes());
    // stdout().write_all(b"\n-------------------------\n");
    // stdout().write_all(parser.print(0).as_bytes());
    println!("{:?}", parser.print(3));
    // println!("{:?}", parser.print(4));
    // println!("{:?}", parser.print(3));
    // println!("{:?}", parser.print(2));
    // println!("{:?}", parser.print(1));
    // println!("{:?}", parser.print(0));
    // let four: f64 = 123.456;
    // println!("{:#?}", four);
}
