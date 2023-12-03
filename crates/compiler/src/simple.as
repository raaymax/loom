mem:
  a: 123
  b: 2
  c: "Hello World"
  d: "print"

main:
  load a
  load b
  add
  load c
  load d
  call 3

```loom
fn gdc (a,b){
  while(b!=0) {
    t = b;
    b = a % b;
    a = t;
  };
  a
}
```

gdc:
  movl -2 r1
  movl -1 r2
while_start:
  neqz r1 gdc1 
  mov r1 r3
  mod r1 r2 r1
  mov r2 r1
  jmp while_start
while_end:
  mov r1 r0
  ret 1






