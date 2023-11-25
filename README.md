# Loom

[![Tests](https://github.com/raaymax/loom/actions/workflows/rust.yml/badge.svg)](https://github.com/raaymax/loom/actions/workflows/rust.yml)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


Programming language just for fun, will kill LUA some day.

Input:
```loom
fn gdc(a, b) {
  while(b!=0) {
    t = b;
    b = a % b;
    a = t;
  };
  a
};

fn findPow(x, z) {
  y = 1;
  while (pow(x, y) % z != 1) {
    y = y + 1;
  };
  y
};

fn shor(x) {
  a = 2;
  while (a < x) {
    if (gdc(a,x) == 1) {
      r = findPow(a, x);
      p = gdc(pow(a, r/2) + 1, x);
      q = gdc(pow(a, r/2) - 1, x);
      if (p*q == x) {
        return "[ " + p + ", " + q +" ]";
      };
    };
    a = a + 1;
  };
  ''
};
x = 21;
y = 49;
print("gdc("+x+","+y+") = " + gdc(x,y));
print("shor(15) = " + shor(15));
0
```
Output:
```
gdc(21,49) = 7
shor(15) = [ 5, 3 ]

Exited with code: 0
```
## How to run
```bash
# cargo run -p cli -- --help

Usage: cli [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to file to run

Options:
  -c, --no-colors  Disable colored output
  -v, --verbose    Enable verbose output
  -h, --help       Print help
  -V, --version    Print version
```

## Code highlights
Tree-sitter parser and highlighting included

## License
MIT License

Copyright (c) 2023 Mateusz Russak