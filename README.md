# Loom
Programming language just for fun, will kill LUA some day.

Input:
```
fn gdc(a, b) {
  while(b!=0) {
    t = b;
    b = a % b;
    a = t;
  };
  a
};

x = 21;
y = 49;
print("GCD("+x+","+y+") = " + gdc(x,y));
0
```
Output:
```
GCD(21,49) = 7
OUTPUT: 0
```

## Code highlights
Tree-sitter parser and highlighting included

## License MIT
