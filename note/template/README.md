# {{crate_name}}

# Before compiling 

This project depends on the counter contract which can be created with `cargo miden new counter-contract` and put in the adjacent folder.
If you already have counter contract under different name all `counter-contract` references in `Cargo.toml`, `interface.wit`, `lib.rs` (`counter_contract` import) should be updated to point to your counter contract (name/path).

## Useful commands

`{{crate_name}}` is built using the [Miden compiler](https://github.com/0xPolygonMiden/compiler).  

`cargo miden` is a `cargo` cargo extension. Check out its [documentation](https://0xpolygonmiden.github.io/compiler/usage/cargo-miden/#compiling-to-miden-assembly)
for more details on how to build and run the compiled programs.


