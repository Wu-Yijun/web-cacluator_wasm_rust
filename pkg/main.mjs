// 导入 wasm 模块，以及 wasm 模块导出的类
import initSync, {MyStruct} from './web_caculator_rust_wasm.js'
import {create_struct} from './web_caculator_rust_wasm.js'

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