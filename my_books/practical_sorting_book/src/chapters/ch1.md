# Apples to Apples

Sorting is the act of arranging items into a logical order. That said, in order to know which item goes in front of the other, they must be compared. So the first step in sorting is making sure that the items you want to sort can be compared.

In Rust, we can only compare items of the same type. For example, consider the comparison `1 > 'a'`. 1 is a number and 'a' is a charater. It is clear that they are not the same type, so we should not be able to compare them. 

```rust,should_panic
println!("{} > {} = {}", 1, 'a', 1 > 'a');
```

1 and 2 are both numbers, but Rust has different types of numbers. If 1 of type u8 and 2 is of type i32, then we cannot compare them.

```rust,should_panic
let one: u8 = 1;
let two: i32 = 2;

println!("{} > {} = {}", one, two, one > two);
```

It is only when 1 and 2 are of the same type that we can actually compare them.

```rust
let one: u8 = 1;
let two: u8 = 2;
println!("{} > {} = {}", one, two, one > two);
```

For simple examples, we do not have to specify the type because Rust has type inference, which means that, in the absence of an explicit type, the compilier will make an educated guess at the which type to use. Coming back to our comparison of `1 > 2`, the compilier will note that these numbers are boing compared and set them to the same type.

```rust
let one = 1;
let two = 2;

println!("{} > {} = {}", one, two, one > two);
```
