# Auto-Palette Development Guidelines

This document provides essential information for developing and contributing to the Auto-Palette project.

## Project Overview

Auto-Palette is a Rust library designed for color manipulation and palette extraction. It provides a command-line interface (CLI) and WebAssembly (WASM) bindings for use in web applications.

## Project Structure

Auto-Palette is a monorepo with the following components:

- **crates/auto-palette**: Core Rust library implementing color manipulation and palette extraction
- **crates/auto-palette-cli**: Command-line interface for the library
- **crates/auto-palette-wasm**: WebAssembly bindings for the library
- **packages/auto-palette-wasm**: JavaScript/TypeScript wrapper for the WebAssembly bindings

## Build/Configuration Instructions

### Prerequisites

- Rust (see rust-toolchain.toml for version)
- Node.js (>=18.0.0)
- pnpm (10.8.0)

### Building the Rust Components

1. Build all Rust components:
   ```bash
   cargo build
   ```

2. Build specific crates:
   ```bash
   cargo build -p auto-palette
   cargo build -p auto-palette-cli
   cargo build -p auto-palette-wasm
   ```

3. Build for release:
   ```bash
   cargo build --release
   ```

### Building the WebAssembly Components

1. Build the WebAssembly bindings:
   ```bash
   cargo build --package auto-palette-wasm --target wasm32-unknown-unknown
   ```

2. Build the JavaScript/TypeScript wrapper:
   ```bash
   cd packages/auto-palette-wasm
   pnpm install
   pnpm build
   ```

## Testing Information

### Running Rust Tests

1. Run all tests:
   ```bash
   cargo test
   ```

2. Run tests for a specific crate:
   ```bash
   cargo test --package auto-palette
   ```

3. Run a specific test:
   ```bash
   cargo test --test palette
   ```

### Running JavaScript/TypeScript Tests

1. Run all tests:
   ```bash
   cd packages/auto-palette-wasm
   pnpm test
   ```

2. Run unit tests only:
   ```bash
   pnpm test:unit
   ```

3. Run end-to-end tests:
   ```bash
   pnpm test:e2e
   ```

### Adding New Tests

#### Rust Tests

1. **Unit Tests**: Add tests within the source files in a `mod tests` block:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       use rstest::rstest;

       #[test]
       fn test_some_function() {
           // Test code here
       }

       #[rstest]
       #[case(input1, expected1)]
       #[case(input2, expected2)]
       fn test_with_parameters(#[case] input: Type, #[case] expected: Type) {
           // Parameterized test code here
       }
   }
   ```

2. **Integration Tests**: Add test files in the `tests` directory:
   ```rust
   // tests/my_test.rs
   use auto_palette::some_module::SomeStruct;

   #[test]
   fn test_some_functionality() {
       // Test code here
   }
   ```

#### JavaScript/TypeScript Tests

1. Add test files in the `test` directory with a `.test.ts` extension:
   ```typescript
   // test/my_component.test.ts
   import { MyComponent } from '@auto-palette/wasm';
   import { describe, expect } from 'vitest';

   describe('@auto-palette/wasm/my-component', () => {
     describe('someMethod', () => {
       it('should do something expected', () => {
         // Arrange
         const component = new MyComponent();
         
         // Act
         const result = component.someMethod();
         
         // Assert
         expect(result).toEqual(expectedValue);
       });
     });
   });
   ```

## Code Style and Development Guidelines

### Rust Code Style

- Follow the Rust style guide enforced by `rustfmt`
- Run `cargo fmt` before committing changes
- Use the `.rustfmt.toml` configuration in the project root

### JavaScript/TypeScript Code Style

- Follow the style guide enforced by Biome
- Run `pnpm format` before committing changes
- Use the `biome.json` configuration in the project root

### Commit Guidelines

- Use the husky pre-commit hooks to ensure code quality
- Follow [conventional commit](https://www.conventionalcommits.org/en/v1.0.0/) message format

### Development Workflow

1. Create a feature branch from `main`
2. Make your changes
3. Run tests to ensure everything works
4. Format your code using the provided tools
5. Submit a pull request

## Debugging Tips

- Use `println!("[DEBUG_LOG] message")` in Rust code for debugging
- Use `console.log("[DEBUG_LOG] message")` in JavaScript/TypeScript code
- For WebAssembly debugging, use the browser's developer tools with source maps