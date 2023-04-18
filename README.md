This is a minimal reproduction of the "ld: library not found for -lsqlite3" issue I'm having in compiling scaffolding with holochain 0.1.4.

To see the issue, do

```
nix develop
cargo build
```

both of these worked for me (stefan) as of

```
(
    set -x
    for i in 1 2; do
        sed -i -E "s/resolver = \".+\"/resolver = \"${i:?}\"/" Cargo.toml
        grep resolver Cargo.toml
        CARGO_TARGET_DIR=wasm cargo build --target=wasm32-unknown-unknown 2>1 > /dev/null
        echo $?
        cargo test  2>1 > /dev/null
        echo $?
    done
)
```
