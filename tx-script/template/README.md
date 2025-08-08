# {{crate_name}}

A Miden rollup transaction script project.

## Overview

This is a transaction script that runs on the Miden rollup. Transaction scripts are executed client-side and can:
- Call functions on account contracts
- Create and send notes
- Move assets between accounts and notes
- Perform complex transaction logic

## Building

To build this transaction script, run:

```bash
cargo miden build
```

This will compile the Rust code to WebAssembly and then to Miden Assembly.

## Project Structure

- `src/lib.rs` - Main transaction script implementation
- `wit/` - WebAssembly Interface Type definitions
- `Cargo.toml` - Project configuration and dependencies

## Development

Transaction scripts implement the `transaction_script::Guest` trait with a `tx_script` function that receives a `Word` argument and executes the transaction logic.

For more information about developing on Miden, see the [Miden documentation](https://docs.polygon.technology/miden/).