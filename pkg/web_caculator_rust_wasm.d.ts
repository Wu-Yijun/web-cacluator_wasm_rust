/* tslint:disable */
/* eslint-disable */
/**
* @returns {MyStruct}
*/
export function create_struct(): MyStruct;
/**
* @param {string} input
* @param {number} level
* @returns {string}
*/
export function parse(input: string, level: number): string;
/**
* @param {string} input
* @returns {string}
*/
export function pares_and_print_html(input: string): string;
/**
*/
export class Caculator {
  free(): void;
/**
* @param {string} input
* @returns {Caculator}
*/
  static new(input: string): Caculator;
/**
* @param {string} input
*/
  new_parser(input: string): void;
/**
*/
  parse(): void;
/**
*/
  calc(): void;
/**
* @returns {string}
*/
  get_html(): string;
}
/**
*/
export class MyStruct {
  free(): void;
/**
* @param {number} value
* @returns {MyStruct}
*/
  static new(value: number): MyStruct;
/**
* @returns {number}
*/
  get_value(): number;
/**
* @param {number} v
*/
  add_value(v: number): void;
/**
*/
  flag: boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_mystruct_free: (a: number) => void;
  readonly __wbg_get_mystruct_flag: (a: number) => number;
  readonly __wbg_set_mystruct_flag: (a: number, b: number) => void;
  readonly mystruct_new: (a: number) => number;
  readonly mystruct_get_value: (a: number) => number;
  readonly mystruct_add_value: (a: number, b: number) => void;
  readonly create_struct: () => number;
  readonly parse: (a: number, b: number, c: number, d: number) => void;
  readonly pares_and_print_html: (a: number, b: number, c: number) => void;
  readonly __wbg_caculator_free: (a: number) => void;
  readonly caculator_new: (a: number, b: number) => number;
  readonly caculator_new_parser: (a: number, b: number, c: number) => void;
  readonly caculator_parse: (a: number) => void;
  readonly caculator_calc: (a: number) => void;
  readonly caculator_get_html: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
