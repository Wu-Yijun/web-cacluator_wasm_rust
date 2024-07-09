use my_parser::LexicalParser;
use wasm_bindgen::prelude::*;

// mod my_math;
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
    LexicalParser::new_inline(input.to_string()).print(level as usize)
}

#[wasm_bindgen]
pub fn pares_and_print_html(input: &str) -> String {
    let line1 = LexicalParser::new_inline(input.to_string())
        .parse()
        .print(11);
    let line2 = LexicalParser::new_inline(input.to_string())
        .parse()
        .tree(0, true);
    line1 + "\n<span class='tree_syntax'>" + &line2 + "</span>"
}

// rust 中的测试
#[test]
fn test() {
    let parser = LexicalParser::new_inline(
        // expressions
        "2\n2".to_string(),
        // "sin(x, y+ 2*(3+-5.3f32 -x));plot(X);{x;y+1}".to_string(),
        // "2(3+5 x)()".to_string(),
    );
    println!("{}", parser.print(3));
    let exp = parser.parse();
    println!("{}", exp.tree(0, false));
}
