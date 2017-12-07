# ci
`ci` is a command line interface for building and testing competitive programming tasks. Currently, it can test any executable and build C++ code.

## Installation

```bash
cargo install --git https://github.com/matcegla/ci
```
If you do not have `cargo` installed, run `curl https://sh.rustup.rs | sh`.

### Bash autocomplete
```bash
ci internal-autocomplete | sudo tee /usr/share/bash-completion/completions/ci
```

## Usage

### `ci build`

```bash
ci build kitties.cpp
```
Compiles `kitties.cpp` to `./kitties.e`. The code is built by compiler `c++`(possibly will change to always use clang) with C++11(`-std=c++11`) and warnings enabled(`-Wall` `-Wextra`) This is the debug build, which includes debugging info(`-g`) and enables C++ standard library debug configuration (`-D_GLIBCXX_DEBUG`), which means that you can debug it and also it will crash on using standard library incorrectly, instead of doing UB.

```bash
ci build --release kitties.cpp
```
Compiles `kitties.c` to `./kitties.e`. The `--release` flag changes the debug build to release build, which enables optimisations(`-O2`).

### `ci test`

```bash
ci test ./kitties.e kitties.test/
```
Recursively finds every file with `.in` extension in `kitties.test/` directory, runs `./kitties.e` with it as input, and compares if result is same as a matching file with `.out` extensions(ignoring some whitespace).

```bash
ci test ./kitties.e kitties.test/ --no-print-success
```
As before, but doesn't print information about tests that succeded. Useful when you have a lot of tests.

```bash
ci test ./kitties.e kitties.test/ --checker ./kitties-checker.e
```
Instead of comparing the output more or less char-by-char, uses a supplied program to do the checking. Program will be called like `./kitties-checker.e kitties.test/1.in <(./kitties.e < ./kitties.test/1.in) kitties.test/1.out` and should return 0 status code if solution is valid(this will change).

### `ci multitest`

```bash
ci multitest ./kitties-gen.py ./kitties-brut.e ./kitties.e ./kitties-alternative.e
```
Instead of using already generated tests, this uses `./kitties-gen.py` to generate the inputs and runs them against every executable. First executable is assumed to always produce correct output.

```bash
ci multitest ./kitties-gen.py ./kitties-brut.e ./kitties.e ./kitties-alternative.e --checker ./kitties-checker.e
```
Instead of comparing char-by-char minus whitespace, this uses a checker like `--checker` in `ci test` does.
