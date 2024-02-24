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
One aspect of sorting that we have not questioned is what should happen when multiple elements in the list are considered equal. For example, the list [one, two, also_one, also_two] where `one` and `also_one` equal 1 and `two` and `also_two` equal 2. if we were to sort this list, then, technically speaking, there are 4 valid combinations:

1. [one, also_one, two, also_two]
2. [one, also_one, also_two, two]
3. [also_one, one, two, also_two]
4. [also_one, one, also_two, two]

```rust
let one = 1;
let also_one = 1;
let two = 2;
let also_two = 2;

let list_1 = vec![one, also_one, two, also_two];
let list_2 = vec![also_one, one, also_two, two];

println!("{:?} == {:?} = {}", list_1, list_2, list_1 == list_2);
```
When comparing two variations of a correctly sorted list, we see that they are equal. That said, having multiple possible arrangements is not always ideal. For that reason, we have unstable sort and stable sort.

Unstable sort is sorting where there is no definitive order given to elements that are considered equal. This gives rise to the situation where the list [one, two, also_one, also_two] has 4 valid arrangements that can result from being sorted.

Stable sort is sorting with the added requirement that all elements that are equal will stay true to the relative ordering in the original list. This means that for the list [one, two, also_one, also_two], the only valid sorted arrangement is [one, also_one, two, also_two] because `one` was originally in front of `also_one` and `two` was originally in front of `also_two`.

The sorting methods that we have looked at so far have all been stable sorts, but most of them do have unstable counterparts.

sort        ----    sort_unstable
sort_by     ----    sort_unstable_by
sort_by_key ----    sort_unstable_by_key


The benefits to using unstable sorting are that it tends to be faster than stable sorting. However, If you need to the gaurentee that there will only ever be one valid sorted arrangement, then you should use a stable sort.

