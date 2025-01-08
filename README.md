[![Rust](https://github.com/Zitronenjoghurt/simple-turing-machine/actions/workflows/rust.yml/badge.svg)](https://github.com/Zitronenjoghurt/simple-turing-machine/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/Zitronenjoghurt/simple-turing-machine/graph/badge.svg?token=UM6T22YO17)](https://codecov.io/gh/Zitronenjoghurt/simple-turing-machine)
![](https://tokei.rs/b1/github/Zitronenjoghurt/simple-turing-machine?category=code&type=Rust&logo=https://simpleicons.org/icons/rust.svg)

# simple-turing-machine
A simple turing machine written in Rust. I'll see where this will take me xD

# Examples
## Mark a field first, reset head, then move right till you reach a marked field
```rust
let mut compiler = TuringCompiler::default();

let move_right_x = compiler.allocate_state();
let set_one = compiler.allocate_state();
let move_left_x = compiler.allocate_state();
let scan_start = compiler.allocate_state();
let done = compiler.allocate_state();

compiler.move_right_x(x, Some(move_right_x), Some(set_one));
compiler.mark(Some(set_one), Some(move_left_x));
compiler.move_left_x(x, Some(move_left_x), Some(scan_start));
compiler.scan_simple(true, Movement::Right, Some(scan_start), Some(done));
compiler.halt(Some(done));

compiler.get_program()
```
```
[0] 0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  | Head: 0 | (q=0, σ=0) => (q'=5, σ'=0, D=R)
 0 [0] 0  0  0  0  0  0  0  0  0  0  0  0  0  0  | Head: 1 | (q=5, σ=0) => (q'=6, σ'=0, D=R)
 0  0 [0] 0  0  0  0  0  0  0  0  0  0  0  0  0  | Head: 2 | (q=6, σ=0) => (q'=7, σ'=0, D=R)
 0  0  0 [0] 0  0  0  0  0  0  0  0  0  0  0  0  | Head: 3 | (q=7, σ=0) => (q'=8, σ'=0, D=R)
 0  0  0  0 [0] 0  0  0  0  0  0  0  0  0  0  0  | Head: 4 | (q=8, σ=0) => (q'=9, σ'=0, D=R)
 0  0  0  0  0 [0] 0  0  0  0  0  0  0  0  0  0  | Head: 5 | (q=9, σ=0) => (q'=10, σ'=0, D=R)
 0  0  0  0  0  0 [0] 0  0  0  0  0  0  0  0  0  | Head: 6 | (q=10, σ=0) => (q'=11, σ'=0, D=R)
 0  0  0  0  0  0  0 [0] 0  0  0  0  0  0  0  0  | Head: 7 | (q=11, σ=0) => (q'=12, σ'=0, D=R)
 0  0  0  0  0  0  0  0 [0] 0  0  0  0  0  0  0  | Head: 8 | (q=12, σ=0) => (q'=13, σ'=0, D=R)
 0  0  0  0  0  0  0  0  0 [0] 0  0  0  0  0  0  | Head: 9 | (q=13, σ=0) => (q'=14, σ'=0, D=R)
 0  0  0  0  0  0  0  0  0  0 [0] 0  0  0  0  0  | Head: 10 | (q=14, σ=0) => (q'=15, σ'=0, D=R)
 0  0  0  0  0  0  0  0  0  0  0 [0] 0  0  0  0  | Head: 11 | (q=15, σ=0) => (q'=16, σ'=0, D=R)
 0  0  0  0  0  0  0  0  0  0  0  0 [0] 0  0  0  | Head: 12 | (q=16, σ=0) => (q'=1, σ'=0, D=R)
 0  0  0  0  0  0  0  0  0  0  0  0  0 [0] 0  0  | Head: 13 | (q=1, σ=0) => (q'=2, σ'=1, D=S)
 0  0  0  0  0  0  0  0  0  0  0  0  0 [1] 0  0  | Head: 13 | (q=2, σ=1) => (q'=17, σ'=1, D=L)
 0  0  0  0  0  0  0  0  0  0  0  0 [0] 1  0  0  | Head: 12 | (q=17, σ=0) => (q'=18, σ'=0, D=L)
 0  0  0  0  0  0  0  0  0  0  0 [0] 0  1  0  0  | Head: 11 | (q=18, σ=0) => (q'=19, σ'=0, D=L)
 0  0  0  0  0  0  0  0  0  0 [0] 0  0  1  0  0  | Head: 10 | (q=19, σ=0) => (q'=20, σ'=0, D=L)
 0  0  0  0  0  0  0  0  0 [0] 0  0  0  1  0  0  | Head: 9 | (q=20, σ=0) => (q'=21, σ'=0, D=L)
 0  0  0  0  0  0  0  0 [0] 0  0  0  0  1  0  0  | Head: 8 | (q=21, σ=0) => (q'=22, σ'=0, D=L)
 0  0  0  0  0  0  0 [0] 0  0  0  0  0  1  0  0  | Head: 7 | (q=22, σ=0) => (q'=23, σ'=0, D=L)
 0  0  0  0  0  0 [0] 0  0  0  0  0  0  1  0  0  | Head: 6 | (q=23, σ=0) => (q'=24, σ'=0, D=L)
 0  0  0  0  0 [0] 0  0  0  0  0  0  0  1  0  0  | Head: 5 | (q=24, σ=0) => (q'=25, σ'=0, D=L)
 0  0  0  0 [0] 0  0  0  0  0  0  0  0  1  0  0  | Head: 4 | (q=25, σ=0) => (q'=26, σ'=0, D=L)
 0  0  0 [0] 0  0  0  0  0  0  0  0  0  1  0  0  | Head: 3 | (q=26, σ=0) => (q'=27, σ'=0, D=L)
 0  0 [0] 0  0  0  0  0  0  0  0  0  0  1  0  0  | Head: 2 | (q=27, σ=0) => (q'=28, σ'=0, D=L)
 0 [0] 0  0  0  0  0  0  0  0  0  0  0  1  0  0  | Head: 1 | (q=28, σ=0) => (q'=3, σ'=0, D=L)
[0] 0  0  0  0  0  0  0  0  0  0  0  0  1  0  0  | Head: 0 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
[0] 0  0  0  0  0  0  0  0  0  0  0  0  1  0  0  | Head: 0 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0 [0] 0  0  0  0  0  0  0  0  0  0  0  1  0  0  | Head: 1 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
 0 [0] 0  0  0  0  0  0  0  0  0  0  0  1  0  0  | Head: 1 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0  0 [0] 0  0  0  0  0  0  0  0  0  0  1  0  0  | Head: 2 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
 0  0 [0] 0  0  0  0  0  0  0  0  0  0  1  0  0  | Head: 2 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0  0  0 [0] 0  0  0  0  0  0  0  0  0  1  0  0  | Head: 3 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
 0  0  0 [0] 0  0  0  0  0  0  0  0  0  1  0  0  | Head: 3 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0  0  0  0 [0] 0  0  0  0  0  0  0  0  1  0  0  | Head: 4 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
 0  0  0  0 [0] 0  0  0  0  0  0  0  0  1  0  0  | Head: 4 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0  0  0  0  0 [0] 0  0  0  0  0  0  0  1  0  0  | Head: 5 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
 0  0  0  0  0 [0] 0  0  0  0  0  0  0  1  0  0  | Head: 5 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0  0  0  0  0  0 [0] 0  0  0  0  0  0  1  0  0  | Head: 6 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
 0  0  0  0  0  0 [0] 0  0  0  0  0  0  1  0  0  | Head: 6 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0  0  0  0  0  0  0 [0] 0  0  0  0  0  1  0  0  | Head: 7 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
 0  0  0  0  0  0  0 [0] 0  0  0  0  0  1  0  0  | Head: 7 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0  0  0  0  0  0  0  0 [0] 0  0  0  0  1  0  0  | Head: 8 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
 0  0  0  0  0  0  0  0 [0] 0  0  0  0  1  0  0  | Head: 8 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0  0  0  0  0  0  0  0  0 [0] 0  0  0  1  0  0  | Head: 9 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
 0  0  0  0  0  0  0  0  0 [0] 0  0  0  1  0  0  | Head: 9 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0  0  0  0  0  0  0  0  0  0 [0] 0  0  1  0  0  | Head: 10 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
 0  0  0  0  0  0  0  0  0  0 [0] 0  0  1  0  0  | Head: 10 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0  0  0  0  0  0  0  0  0  0  0 [0] 0  1  0  0  | Head: 11 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
 0  0  0  0  0  0  0  0  0  0  0 [0] 0  1  0  0  | Head: 11 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0  0  0  0  0  0  0  0  0  0  0  0 [0] 1  0  0  | Head: 12 | (q=3, σ=0) => (q'=29, σ'=0, D=S)
 0  0  0  0  0  0  0  0  0  0  0  0 [0] 1  0  0  | Head: 12 | (q=29, σ=0) => (q'=3, σ'=0, D=R)
 0  0  0  0  0  0  0  0  0  0  0  0  0 [1] 0  0  | Head: 13 | (q=3, σ=1) => (q'=4, σ'=1, D=S)
 0  0  0  0  0  0  0  0  0  0  0  0  0 [1] 0  0  | Head: 13 | (q=4, σ=1) => (q'=18446744073709551615, σ'=1, D=S)
```