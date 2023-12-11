# Simple ISA
Implementation of a simple ISA in Rust. It supports SC, TS and PSO memory models as well as tracing mode for debugging.

## Instructions 
- `r = 1` - Put constant into register.
- `r1 = r2 # r3` - Binary operation on two registers.
- `if r goto L` - Conditional jump on label L.
- `load m #r1 r2` - Load value from memory by address stored in r1 into register r2.
- `store m #r1 r2` - Store value from register r2 into memory by address stored in r1.
- `r1 := cas m #r2 r3 r4` - Compare-and-swap value in memory by address stored in r2, expected value is stored in r3, desired value is stored in r4, should return the actually read value in register r1.
- `r1 := fai m #r2 r3` - Fetch-and-increment value in memory by address stored in r2, the value to increment by is stored in r3, should return the read value prior increment in register r1.
- `fence m` - Memory fence instruction.

## Parameters and flags
The console app supports the following flags:

- `--file` - the path to the file with commands.
- `--model` - the name of the model you want to use.
- `--trace` - flag for activating trace mode.

## Example
Different threads instructions should be separated in file by an empty line. For example:
```
✗ cat prog.txt
r1 = 1
r2 = 2
r3 = r1 + r2
store SEQ_CST #r1 r3

r1 = 1
load SEQ_CST #r1 r3
```
Now we can interpret this file:
```
✗ cargo run --bin main -- --file prog.txt --trace --model SC
1: r1 = 1
# REGISTERS
| Thread 0: {}
| Thread 1: {"r1": 1}
# MEMORY
| {}

1: load ACQ #r1 r3
# REGISTERS
| Thread 0: {}
| Thread 1: {"r1": 1, "r3": 0}
# MEMORY
| {}

0: r1 = 1
# REGISTERS
| Thread 0: {"r1": 1}
| Thread 1: {"r1": 1, "r3": 0}
# MEMORY
| {}

0: r2 = 2
# REGISTERS
| Thread 0: {"r1": 1, "r2": 2}
| Thread 1: {"r1": 1, "r3": 0}
# MEMORY
| {}

0: r3 = r1 + r2
# REGISTERS
| Thread 0: {"r1": 1, "r3": 3, "r2": 2}
| Thread 1: {"r1": 1, "r3": 0}
# MEMORY
| {}

0: store REL #r1 r3
# REGISTERS
| Thread 0: {"r1": 1, "r3": 3, "r2": 2}
| Thread 1: {"r1": 1, "r3": 0}
# MEMORY
| {1: 3}
```