# ci
`ci` is a command line interface for building and testing competitive programming tasks. Currently, it can test any executable and build C++ code.

## Installation

```bash
cargo install --git https://github.com/matcegla/ci
```
If you do not have `cargo` installed, run `curl https://sh.rustup.rs | sh`.

### Bash autocomplete
```bash
ci generate-autocomplete bash | sudo tee /usr/share/bash-completion/completions/ci
```

## Usage

### `ci build`

```bash
ci build kitties.cpp
```
Compiles `kitties.cpp` to `./kitties.e`. The code is built by compiler `clang++` with C++17(`-std=c++17`) and warnings enabled(`-Wall` `-Wextra` `-Wconversion` `-Wno-sign-conversion`). This is the debug build, which includes debugging info(`-g`) and enables C++ standard library debug configuration (`-D_GLIBCXX_DEBUG`) as well as UB sanitizer(`-fno-sanitize-recover=undefined`), which means that you can debug it and also it will crash on UB.

```bash
ci build --release kitties.cpp
```
Compiles `kitties.c` to `./kitties.e`. The `--release` flag changes the debug build to release build, which enables optimisations(`-O2`).

```bash
ci build --standard 11
```
Compiles with C++11 instead of default(C++17), with the flag `-std=c++11`. Possible values are:
`11`(`-std=c++11`),
`17`(`-std=c++17`).

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

```bash
ci multitest ./kitties-gen.py ./kitties-brut.e ./kitties.e ./kitties-alternative.e -n 200
```
Instead of breaking on first failing test, this will run 200 tests and only print one with greatest fitness. Fitness function is `- (test char count)` by default.

```bash
ci multitest ./kitties-gen.py ./kitties-brut.e ./kitties.e -n 200 --fitness ./kitties-fit.e
```
Uses a supplied fitness function instead of the default. `@bytelen` is a special value that is `-(test char count)`.

### `ci vendor`

```bash
ci vendor kitties.cpp
```
When running your code in testing environments, you will not have access to various helpful header-only libraries like your personal algorithm collections, [mc](https://github.com/matcegla/mc) or [Boost.Graph](http://www.boost.org/doc/libs/1_66_0/libs/graph/doc/table_of_contents.html). This command will only run the preprocessor, copy-pasting these libraries into your code so you can send it to an online judge system.

To use the command, a directory `/usr/share/ci/dummy-includes` must be created, with all the includes you do not want copy-pasted(like `<iostream>`). To do this, create a file in this directory, like `sudo touch /usr/share/ci/dummy-includes/iostream`.

### `ci init`

```bash
ci init https://codeforces.com/contest/960/problem/D
```
To save time on entering example tests, this command will download and save them to `./tests/example` directory for you. Few task formats are supported.

### `ci submit`

```bash
ci submit code.cpp https://codeforces.com/contest/960/problem/D
```
To save time on submitting your code, this command will submit them for you! Few sites are suppported. The URL should be the URL to problem description(same as in `ci init`).
