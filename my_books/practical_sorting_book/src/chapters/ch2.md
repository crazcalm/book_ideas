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

```rust
let mut list = vec![3,2,5,1,6,7,8,0,9,4];
list.sort_by(|a, b| a.cmp(b));
println!("{:?}", list);
```

In the above example, we pass the closure `|a, b| a.cmp(b)` to the `sort_by` method. This closure takes two parameters; `a` and `b`. Then, in the function body, used the `cmp` method to compare `a` to `b`. The `cmp` method returns an variant of the Ordering Enum.

## Where did the cmp method come from?

In order to sort a list, you must be able to compare the items of that list, but how do you know if the items are comparable? I mentioned that the items have to be the same type, but that is only part of the answer. The full answer is that the items have to be of the same type and that type must implement the Ord Trait, which is a trait that guarentees that two items of the same type are comparable. The existence of the `cmp` method is a result of implementing the Ord Trait.

The requirement of the Ord Trait when sorting can be seen in the function signature for Vec's sort method:

```rust,noplayground

impl<T> [T] {
...
pub fn sort(&mut self)
    where
        T: Ord,
...
```
In the above snippet, we see that, in order to use the sort method, `T`, which represents the type of the elements of the list, has to implement the Ord Trait. This means that in all of our examples up till now where we have successfully sorted a list, the type of the elements in those lists have all implemented the Ord Trait.
