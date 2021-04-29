/* tslint:disable */
/* eslint-disable */
/**
* @param {string} s
*/
export function say_hello(s: string): void;
/**
*/
export function display(): void;
/**
*/
export function draw(): void;
/**
* @param {number} n
*/
export function print_fibo(n: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly say_hello: (a: number, b: number) => void;
  readonly display: () => void;
  readonly draw: () => void;
  readonly print_fibo: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
