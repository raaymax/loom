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
TOKENS:
        Fn [0:2]
        Id [3:3] ( name: gdc )
        LParen [6:1]
        Id [7:1] ( name: a )
        Comma [8:1]
        Id [10:1] ( name: b )
        RParen [11:1]
        LBrace [13:1]
        While [17:5]
        LParen [22:1]
        Id [23:1] ( name: b )
        Neq [24:2]
        Number [26:2] ( value: 0 )
        RParen [27:1]
        LBrace [29:1]
        Id [35:1] ( name: t )
        Assign [37:1]
        Id [39:1] ( name: b )
        Semi [40:1]
        Id [46:1] ( name: b )
        Assign [48:1]
        Id [50:1] ( name: a )
        Mod [52:1]
        Id [54:1] ( name: b )
        Semi [55:1]
        Id [61:1] ( name: a )
        Assign [63:1]
        Id [65:1] ( name: t )
        Semi [66:1]
        RBrace [70:1]
        Semi [71:1]
        Id [75:1] ( name: a )
        RBrace [77:1]
        Semi [78:1]
        Id [81:1] ( name: x )
        Assign [83:1]
        Number [85:3] ( value: 21 )
        Semi [87:1]
        Id [89:1] ( name: y )
        Assign [91:1]
        Number [93:3] ( value: 49 )
        Semi [95:1]
        Id [97:5] ( name: print )
        LParen [102:1]
        String [103:6] ( value: "GCD(")
        Plus [109:1]
        Id [110:1] ( name: x )
        Plus [111:1]
        String [112:3] ( value: ",")
        Plus [115:1]
        Id [116:1] ( name: y )
        Plus [117:1]
        String [118:6] ( value: ") = ")
        Plus [125:1]
        Id [127:3] ( name: gdc )
        LParen [130:1]
        Id [131:1] ( name: x )
        Comma [132:1]
        Id [133:1] ( name: y )
        RParen [134:1]
        RParen [135:1]
        Semi [136:1]
        Number [138:2] ( value: 0 )
        EOF


TREEs:  {
  fn gdc((a,b)) {
    while ((b != 0)) {
      (t = b);
      (b = (a % b));
      (a = t);
      ()
    };
    a
  };
  (x = 21);
  (y = 49);
  print((((('GCD(' + x) + ',') + y) + ') = ') + gdc(x,y));
  0
}

EXECUTING:
GCD(21,49) = 7

OUTPUT: 0
```
## How to run
```
cargo run -p cli <source_file>
```

## Code highlights
Tree-sitter parser and highlighting included

## License MIT
