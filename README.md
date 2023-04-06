# minus-one
[![Crates.io](https://img.shields.io/crates/v/minus-one.svg?style=plastic)](http://crates.io/crates/minus-one)

```rust
let max_byte: u8 = minus_one!(256);
assert_eq!(max_byte, 255);

let min_byte: u8 = minus_one!(1);
assert_eq!(min_byte, 0);
```

Often in firmware-land, you are using a register where a register value of 0 corresponds to a value of 1. This is a
common pattern in hardware.
However, this can be a headache when you are trying to write a value to the register, and you have to remember to
subtract 1 from the value you want to write.
In addition, sometimes it is not even _possible_ to subtract 1 because the value you want to write is the maximum
value.

Suppose you have a `u8` and you want to write something which represents scalar 256. You cannot subtract 1 from 256
because 255 is the maximum value of an `u8`.
The `minus_one!` macro allows you to write `minus_one!(256)` and get `255` back, and likewise `minus_one!(1)` will
return `0`.
