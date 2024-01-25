# The Basics

Add an overview of this chapter.

## What is Sorting?

As a software engineer, I would define sorting as the act of placing a collection of items in order. For example, I can take a list of numbers and sort them so that they are in ascending order or descending order.

TODO: Add visuals

Though I agree with this definition, there are two parts of it that I would like to examine; Collection and order.

Do the items you sort have to be in a collection? It depends on how you defined a collection. If you think of a collection as a data structure like Vectors, tuples, Queues, etc, then the anser is no. Though it is straight forward to sort items contained in one of these data structures, what happens when you have to sort more items than you can fit into memory? You can still sort these items and you will still use these data structures to help you sort, but the sorted output cannot be held in any one of these data structures because there is not enough memory to do so.

However, if we think of a collection as a number of items, then they answer is yes. It is yes because we now have no implicit limitation on how many things we can sort.

The second part of the definition that I want to examine is the word "order". The American Heritage dictionary defines order as "a condition of logical or comprehensible arrangement among the separate elements of a group." [link](https://ahdictionary.com/word/search.html?q=order)

With this definition in mind, are the lists [1,2,3,4,5] and [5,4,3,2,1] ordered? Yes, they are ordered because ascending and decending numbers are both a logical arrangment of numbers. What about the list of [1,3,5,2,4], is this ordered? I would say yes because these numbers follow the logical arrangment of ascending order where all odd number are placed in front of even numbers. 

When it comes to order, the focus is on being able to create a logical arrangement of your items. So even if a list does not look ordered, it could be the case that order it following a logical arrangement that we are not used to seeing. For example, the list [3,1,2,4,5] could be unordered, or it could be a sorted list where 3s get priority and all other numbers get arranged in ascending order. 


TODO: re-write the above and add a conclusion statement.

## The first step in Sorting

Sorting involves arranging items in a logical order. However, you can only order items that can be compared. So the first step in sorting, is making sure that the items you want to sort can be compared to one other.

In Rust, we can only compare items if they are of the same type. This means that if you check if 1 is greater than 2, you will get false.

```rust
println!("{} > {} = {}", 1, 2, 1 > 2);
```

But if you check if 1 is greater than 'a' (char), you will get a mismatched type error.

```rust,should_panic
println!("{} > {} = {}", 1, 'a', 1 > 'a');
```

Once you know the items you are working with can be compared, then you can put them into a vector and use the sort method to sort them.

It should be noted that the vector has to be mutable because the act sorting will rearrange the items in the vector.

```rust
let mut list = vec![3,2,5,1,6,7,8,0,9,4];
list.sort();
println!("{:?}", list);

//output: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
```
In the above example, when we print out the list, we use `{:?}`. This is because Vectors do not implement the Display Trait, which means that we cannot use `{}` when printing out our list. We are allowed to use `{:?}` when printing because Vectors implement the Debug Trait, which allows us to print the debugging context for an object.


Another way to sort a vector of items is by using the `sort_by` method. This method takes a function that takes two items and returns an Ordering Enum, which has three Variants; Less, Equal and Greater. The returned Ordering Enum is used to denote how these two items compare to each other.

```rust
let mut list = vec![3,2,5,1,6,7,8,0,9,4];
list.sort_by(|a, b| a.cmp(b));
println!("{:?}", list);

//output: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
```
In the above example, we pass a closure function into that accepts the parameters a and b. In the body of the closure, we compare a to be with the `cmp` method. The `cmp` function, which it defined for the type itself, will return Ordering::Less, Ordering::Equal, or Ordering::Greater.

Though this makes sense, where did this `cmp` method come from?

Earlier in the chapter, I said that you can only order items that can be compared. In Rust, this means that the item implements the Ord Trait, which ensures that the type of item that you are working with has the methods needed to be compared to another instance of that type.

Coming back to the Ordering Enum, we talked about it as a return value, but it also has a useful API. For example, if you want to sort an array in decreating order, you may be tempted to sort the vector and then reverse it. 

```rust
let mut list = vec![3,2,5,1,6,7,8,0,9,4];
list.sort();
list.reverse();
println!("{:?}", list);

//output: [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
```

Though this works, another way to get the same result is to call the reverse method on the Ordering Enum:

```rust
let mut list = vec![3,2,5,1,6,7,8,0,9,4];
list.sort_by(|a, b| a.cmp(b).reverse());
println!("{:?}", list);

//output: [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
```

TODO: Add exercises to compare objects and sort them
