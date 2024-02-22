# My Sorting Options

So far, we have introduced the `sort` and `sort_by` methods, but there is another method called `sort_by_key` that we can use as well. Here's the function signature:
```
impl<T> [T] {
...
pub fn sort_by_key<K, F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
...
```
`sort_by_key` takes in a function f that takes an element from the list and returns a value of type K where type K implements the Ord Trait. Comparing this method to `sort` and `sort_by`, we see that:

1. The elements of the list do not have to implement (This is a requirement for `sort`)
2. The function passed in only takes a single parameter (The function passed into `sort_by` takes 2 paramters)
3. The return value of the function must implement the Ord Trait (The return value of the `sort_by` method is a variant of the Ordering Enum)

2 tells us that `sort_by_key` is not used to compare elements in a list. Knowing that being able to compare elements in a list is a requirement for sorting, 1 and 3 tells us that the function will return the value will be used to sort this list.

For example, if you wanted to sort a vector of [-10, -7, -3, 0, 3, 7, 10] by their return values `x mod 7`, you can do the following:
```rust
fn main(){
	let mut list: Vec<i32> = vec![-10, -7, -3, 0, 3, 7, 10];
	list.sort_by_key(|x| x.rem_euclid(7));
	
	println!("{:?}", list)
}
```
In this example, we use the `rem_euclid` to compute the value of `x mod 7` where `x` is an element of the list. `x mod 7` gives us a list of [4, 0, 4, 0, 3, 0, 3] which is then sorted and placed with its original values.

If the function that we use to compute these keys is computationally expensive, then you may use the `sort_by_cached_key` which will keep a local store of your results so that you can avoid repeate computations.

## Unstable Sort
