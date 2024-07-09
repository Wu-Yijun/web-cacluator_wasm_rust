// 导入 wasm 模块，以及 wasm 模块导出的类
import initSync, {MyStruct, Caculator } from './web_caculator_rust_wasm.js'
import {create_struct, parse, pares_and_print_html} from './web_caculator_rust_wasm.js'

// 初始化
const wasm = await initSync();

// 创建一个 MyStruct 实例
const myStruct = MyStruct.new(1234);
console.log('myStruct', myStruct.get_value());
myStruct.add_value(54321);
console.log('myStruct', myStruct.get_value());
myStruct.add_value(54321);
console.log('myStruct', myStruct.get_value());
console.log('myStruct flag', myStruct.flag);

// 调用 wasm 模块导出的函数
const myStruct2 = create_struct();
console.log('myStruct2', myStruct2.get_value());
console.log('myStruct2 flag', myStruct2.flag);


// 测试 Parser

console.log('>>> ', `parse('Plot(((x+0.1)^2)^cos(T)/* 绘图 */)', 2)`);
console.log(parse('Plot(((x+0.1)^2)^cos(T)/* 绘图 */)', 2));

function update_parser(){
    let value = document.getElementById('parser-input').innerText;
    document.getElementById('parser-output').innerText = parse(value, 3)
}
document.getElementById('parser-input').oninput = update_parser;
update_parser();

function update_parser_syntax(){
    let value = document.getElementById('parser-syntax-input').innerText;
    document.getElementById('parser-syntax-output').innerHTML = pares_and_print_html(value)
}
document.getElementById('parser-syntax-input').oninput = update_parser_syntax;
update_parser_syntax();

const mycalc = Caculator.new(document.getElementById('caculator-input').innerText);
document.getElementById('caculator-output').innerHTML = mycalc.get_html();
function update_caculator(){
    debugger;
    let value = document.getElementById('caculator-input').innerText;
    mycalc.new_parser(value);
    mycalc.parse();
    mycalc.calc();
    document.getElementById('caculator-output').innerHTML = mycalc.get_html();
}
document.getElementById('caculator-input').oninput = update_caculator;

// 设置点击折叠功能
for( let element of document.getElementsByClassName('card-header')) {
    element.addEventListener('click',()=>{
        element.classList.toggle('card-hide');
    });
}