# Loom

[![Tests](https://github.com/raaymax/loom/actions/workflows/rust.yml/badge.svg)](https://github.com/raaymax/loom/actions/workflows/rust.yml)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


Programming language just for fun, will kill LUA some day.

Currently development of this language is algorithm driven. I'm trying to implement various algorithms in my language and then I'm adding missing parts to make them work :D 
Below you can find Shor's algorithm for finding prime factors of an integer.

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
Tree-sitter parser and highlighting included.
```bash
cd tree-sitter-loom
tree-sitter generate
tree-sitter test 
```

Configuration for NeoVim:

## Language server

Repository contains also LSP server for language. 
For now only simple diagnostics are supported.

## NeoVim configuration

Configuration to start development in NeoVim. 

### filetype
```lua
vim.filetype.add({
  extension = {
    lum = 'loom',
  }
})
```

### LSP
```
local configs = require 'lspconfig.configs'

if not configs.loom then
  configs.loom = {
    default_config = {
      cmd = { '~/Workspace/Loom/target/debug/lsp' },
      root_dir = lspconfig.util.root_pattern('.git'),
      filetypes = { 'loom' },
      name = { 'loom' },
    },
  }
end

lspconfig.loom.setup {}
```

### tree-sitter
```lua
local parser_config = require "nvim-treesitter.parsers".get_parser_configs()
parser_config.loom = {
  install_info = {
    url = "~/Workspace/Loom/tree-sitter-loom", -- path to location 
    files = {"src/parser.c"},
    branch = "main",
    generate_requires_npm = false,
    requires_generate_from_grammar = false,
  },

  filetype = "loom",
}
vim.treesitter.language.register('loom', 'loom')  
```

Loading new tree-sitter config in nvim:
```
:TSUpdate
```

## License
MIT License

Copyright (c) 2023 Mateusz Russak
