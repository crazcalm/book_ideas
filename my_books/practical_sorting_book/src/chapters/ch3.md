# Logical Arrangements

The power of the Ordering Enum is in the usage of its API and variants. For example, if you want to sort a Vector of numbers in ascending order, we can can use the `sort` method. We can accomplish the same thing by using the `sort_by` method and passing the closure `|a, b| a.cmp(b)`.

```rust
let mut list_1 = [6,5,7,4,8,2,9,1,3,0];
let mut list_2 = [6,5,7,4,8,2,9,1,3,0];

list_1.sort();

list_2.sort_by(|a, b| a.cmp(b));

assert_eq!(list_1, list_2);
```

If we wanted to sort the numbers into descending order, we cannot do that with `list_1`. With `list_1`, we can reverse all the elements in the list by calling the `reverse` method on the Vector. This should give us the correct ordering, but we did not go through the sorting process to achieve the ordering.

With `list_2`, we can call `reverse` on the return result form the `cmp` method. This is because `cmp` returns a variant of the Ordering Enum, and the Ordering Enum has a `reverse` method that allows you to reverse the result of the comparision. In other words, Ordering::Less become Ordering::Greater, Ordering::Greater becomes Ordering::less and Ordering::Equal stays the same.

```rust
let mut list_1 = [6,5,7,4,8,2,9,1,3,0];
let mut list_2 = [6,5,7,4,8,2,9,1,3,0];

list_1.sort();
list_1.reverse();

list_2.sort_by(|a, b| a.cmp(b).reverse());

assert_eq!(list_1, list_2);
```
This means that, for `list_2`, we are correctly arranging the numbers into decending order while we are sorting.


What if we wanted to arrange all of the number in ascending order and have all of the odd number appear before all of the even number? We know we cannot do this with the `sort` method because we have no way of telling it how to partition groups of numbers, but we can do this with the `sort_by` method. We accomplish this by writing a function that determines if `a` or `b` is odd, and if only one of them is odd, we give it priority. Keeping the default behavior in mind where the item with the least value gets placed in front, we give the odd number priority over the even numbers by making sure we always report that it is less than the even number.

```rust
use std::cmp::Ordering;

fn main() {
    let mut list: Vec<i32> = vec![3,2,5,1,6,7,8,0,9,4];
    
	list.sort_by(|a, b| {	
		let a_is_odd = a.abs() % 2 == 1;
		let b_is_odd = b.abs() % 2 == 1;
	
		if a_is_odd && b_is_odd {
			a.cmp(b)
		} else if a_is_odd {
			Ordering::Less
		} else if b_is_odd {
			Ordering::Greater
		} else {
			a.cmp(b)
		}
    });
    
	println!("{:?}", list);
}
```
In the above example, our closure function checks to see if `a` is odd and if `b` is odd. If both `a` and `b` are odd, then we can do a regular comparison between `a` and `b` because neither of them have priority. If `a` is odd and `b` is even, we need to report that `a` is less than `b` so that `a` always gets placed in front of `b`. If `b` is odd and `a` is even, then we need to report that `a` is greater than `b` so that `b` can always be placed in front of `a`. If both `a` and `b` are even, then we can do a regular comparison because neither of them get priority.

The end result of using this closure function in our `sort_by` call is that all of the odd number are placed in front of all the even numbers and the odd number and the even number, within their respective groups, are arranged in acending order.
