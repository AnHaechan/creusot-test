# How to install

## 1. Install Creusot
- Follow https://github.com/xldenis/creusot#installing-creusot-as-a-user
- To check if the installation succeed, try `cargo-creusot` (not `cargo creusot`!!)
- If `cargo-creusot` crashes with "cargo-creusot: error while loading shared libraries: libstd-c1ef87628285f700.so: cannot open shared object file: No such file or directory",
  - Reason is: `LD_LIBRARY_PATH` is not set up with your path to shared libraries
    - To confirm this, `env | grep LD_LIBRARY_PATH`
  - Fix by: Adding the path to shared library of the rustup toolchain currently being used
    - Check toolchain version in `creusot/rust-toolchain`
    - Add e.g. `export LD_LIBRARY_PATH=/home/haechan.an/.rustup/toolchains/nightly-2023-06-29-x86_64-unknown-linux-gnu/lib:$LD_LIBRARY_PATH`

## 2. Set up your cargo crate to use Cresuot
- Follow https://github.com/xldenis/creusot#verifying-with-creusot-and-why3
- Copy `creusot/rust-toolchain` to your project repo
- Copy `creusot/prelude` to your project repo

# How to use

## 1. Run `cargo-creusot` to transform annotated Rust file to why3 mlcfg file

## 2. Find generated mlcfg file in `target/debug/`

## 3. Use why3 (command-line or ide) to prove the mlcfg file.
  - `why3 prove *.mlcfg -L prelude/ --prover Alt-Ergo,2.4.3`
  - `why3 ide *.mlcfg -L prelude"
  - More about command: https://why3.lri.fr/doc/manpages.html#options

## 4. Why3 will report syntax error. Manually fix it for now.
  - Syntax error on `invariant { ... }`
  - It's because, according to [mlcfg syntax](https://why3.lri.fr/doc/input_formats.html#mlcfg), `invariant <id> { ... }` is correct syntax. (Invariants need identifier)
  - Creusot does not automatically add identifier, so, for now, manually add identifiers to every `invariant`.
    - E.g. `invariant { ... }` -> `invariant I1 { ... }`