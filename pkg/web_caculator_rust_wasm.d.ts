/* tslint:disable */
/* eslint-disable */
/**
* @returns {MyStruct}
*/
export function create_struct(): MyStruct;
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
