import init from '@auto-palette/core';

export {
  type Algorithm,
  Color,
  Palette,
  Swatch,
  type Theme,
} from '@auto-palette/core';

/**
 * The input type for the WASM module.
 */
export type ModuleInput =
  | RequestInfo
  | URL
  | Response
  | BufferSource
  | WebAssembly.Module;

/**
 * Initialize the `@auto-palette/wasm` module
 *
 * @param module - The WASM module or path to the WASM module.
 * @returns The Promise that resolves when the WASM module is initialized.
 */
export async function initialize(
  module: ModuleInput | Promise<ModuleInput>,
): Promise<void> {
  await init({ module_or_path: module });
}
