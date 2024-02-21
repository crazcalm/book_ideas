# Line them up

Once you know that you can compare your items, you can them put them into a Vector and use the Vector's sort method to order them. Given that the sort methem can re-arrange the items within the Vector, the Vector must be mutable.

```rust
let mut list = vec![3,2,5,1,6,7,8,0,9,4];
list.sort();
println!("{:?}", list);
```
In the above example, we use `{:?}` to print out the list instead of `{}`. This is because `{}` is used for objects that implement the Display Trait. Vectors do not implement the Display Trait, but they do implement the Debug Trait, which means that they have a debug representation that can be printed to the screen. The `:?` tells the formatter to print out the debug representation of the object.


Another way to sort a Vector is to use the `sort_by` method. This method has the following function signature:

```rust,noplayground
pub fn sort_by<F>(&mut self, mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
```
`compare` is a function that takes references to two items that are of the same type and returns a variant of the Ordering Enum. The puporse of this function is to compare two items. The returned value will let us know if item A is less then (Ordering::Less), greater than (Ordering::Greater) or equal to (Ordering::Equal) item B.

