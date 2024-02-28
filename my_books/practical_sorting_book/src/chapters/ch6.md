# Multi-dimensional sorting

Multi-dimensional sorting is the idea of sorting based on more than one property. A data structure that allows us to easily create items with more than one property are tuples.

A Tuple is a finite heterogeneous sequence. Finite meaning that there are a fix number of elements within in a tuple. Heterogenous meaning that each element of a tuple can be of a different type.

The tuple `(String, u32)` states that the first element is of type `String` and the second element is of type `u32`. You can think of the elements of your tuple as properties of the object. For example, `String` could represent a person's name while `u32` represents their age.

If we were to sort a list of these tuples, we could do it by the first element, the second element, or a combination of the two elements. To sort only on the first element or the second element would be a single dimensional sort because we would only be sorting on one property. An example of this is sorting a group of people by name. In this type of sort, if two people had the same name, then we can arbitrarily place one person in front of the other because we know that any ordering of these two people would be correct. However, if we were to do a multi-dimensional sort by both name and age, people with the same name would then be sorted by their age. In other words, we will first sort the group of people by name, and, if multiple people have the same name, we would then sort those people by age.

This way of sorting where we look at one property and, if two elements have the same value for that property, we then compare the next property is the default sort implementation for Tuple. This default implementation looks at the left most element first and moves right as needed.

```rust
	let mut test = vec![
		("a".to_string(), 1),
		("ab".to_string(), 0),
		("b".to_string(), 2),
		("a".to_string(), 0),
		("ba".to_string(), 0),
	];
    test.sort();

    println!("{:?}", test);
	
	//output: [("a", 0), ("a", 1), ("ab", 0), ("b", 2), ("ba", 0)]
```

In the above example, we use the default sort implementation to sort a list of tuples. We can replicate sort by using the `sort_by` method with the appropriate closure function.

```rust
    let mut test = vec![
		("a".to_string(), 1),
		("ab".to_string(), 0),
		("b".to_string(), 2),
		("a".to_string(), 0),
		("ba".to_string(), 0),
	];

    test.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    
	println!("{:?}", test);

	//output: [("a", 0), ("a", 1), ("ab", 0), ("b", 2), ("ba", 0)]
```

In our closure we use the `cmp` method to compare the first element of Tuples `a` and `b`. The `cmp` method returns an Ordering Enum. The Ordering Enum has a `then` method that allows us to chain Ordering Enums together when the initial `cmp` result is not Ordering::Equal.

Though our tuple has multiple properties, we can still sort by a single property. For example, we could take a list of employees represented by the tuple `(&str, u32)` where `&str` is represents their name and `u32` represents their years with the company and sort them, in descending order, by how many years they have been with the company.

```rust
let mut list = vec![
	("Marcus".to_string(), 2),
	("Jovanna".to_string(), 5),
	("Carmen".to_string(), 2),
	("Christy".to_string(), 2),
	("Dillon".to_string(), 0),
	("Jerry".to_string(), 1)
];

list.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    
println!("{:?}", list);

//output: [("Jovanna", 5), ("Marcus", 2), ("Carmen", 2), ("Christy", 2), ("Jerry", 1), ("Dillon", 0)]
```
When looking at the output of our sort, you will notice that employees that have worked the same number of years are not presented in alphabetical order. This is intentional because the function we are using to sort them ignores their name.
