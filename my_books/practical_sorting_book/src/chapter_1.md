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

Keep in mind that the `sort_by` mandates that you return a Ordering Enum, but that does not means that the body of the function closure needs to call a method that returns an Ordering Enum.

Here is an example where the closure body explicitly returns an Ordering Enum based on the logic in the body. We first define whether `a` or `b` is odd and then we do a series of checks. If `a` and `b` or odd, we want to place them in ascending order, but if `a` or `b` is not odd, than we want to place the odd number in front of the even number. We do this by stating that the odd number is less than the even number.

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

    //output: [1, 3, 5, 7, 9, 0, 2, 4, 6, 8]    
}
```

Another sorting method you can use is called `sort_by_key`. This method takes in function that that accepts a single paramter `a`. The goal of the function is to do a computation on `a` that returns a value of type `K` that implements the Ord trait. The reason that the return value has to implement the Ord trait is beause `sort_by_key` is going to use your function `f` to get the Ordering Enum from checking whether or not `f(a)` is less than `f(b)`. That computation looks like this: `f(a).lt(&f(b))`.

For example, if you wanted to sort a vector of [-10, -7, -3, 0, 3, 7, 10] by their return values `x mod 7`, you can do the following:

```rust
fn main(){
	let mut list: Vec<i32> = vec![-10, -7, -3, 0, 3, 7, 10];
	list.sort_by_key(|x| x.rem_euclid(7));
	
	println!("{:?}", list)
	
	//output: [-7, 0, 7, 3, 10, -10, -3]
}
```

If your function `f` is computaionally expensive and you believe that cahing previously computed results may speed up the process, then you should consider using `sort_by_cached_key` instead.


One question we have not considered it, what happens when you compare two or more items and, based on the comparison method you chose, they are equal? How should they be sorted? In theory, any ordering of these items would be correct. However, in practice, we have two options; unstable sort, and stable sort.

Unstable sort means that these equal items can appear in any order. Stable sort means that the order they were in originally will be preserved once they are sorted.

The benefit of using unstable sorting is that it is generally faster than stable sorting. The benefit of using stable sorting is that the same imput will always give your the same output. Up until now, we have only looked at stable sorts (`sort`, `sort_by`, `sort_by_key` and `sort_by_cached_key`), but most of the sorting methods we looked at have an unstable counterpart (`sort_unstable`, `sort_unstable_by`, `sort_unstable_by_key`).


## Common uses for sorting
One common use case for using a sorting is comparing two list. When checking to see if two Vectors are equal, the ordering of the elements is important. For example, [1,2,3] is not equal to [2,3,1] even though they contain the same elements.

```rust
let a = vec![1,2,3,4];
let b = vec![2,3,1,4];

println("{:?} == {:?} => {}", a, b, a == b);
```

However, if we sorted both lists prior to checking for equality, the equality check will say that they are equal.

```rust

let mut a = vec![1,2,3,4];
let mut b = vec![2,3,1,4];

a.sort()
b.sort()

println("{:?} == {:?} => {}", a, b, a == b);
```

Another common use case for sorting is to make searching for an element easier.
For example, if you have a list of numbers and you want to find the smallest element, you can sort the list in ascending order the look at the first element of the list. If you want the largest element of the list then sorting in ascending order will place that element at the end of the list. If you want the last element in the beginning of the list, you can sort the list in descending order and place the largest element at the beginning of the list.

Ultimately, all we are doing is using the knowledge of the sort to help us find the element that we want. For example, binary search is a searching technique where we look for element X in a list that is sorted in ascending order. The properties of a list sorted in ascending order are that for any element in the list, any element on the left of it will be equal to or less than it and any element on the right is going to be equal to or greater than it. Knowing this, binary search says, let me start my search at the middle of the llist. If the element i am looking for is less than the element in the middle of the list, then I know that, if my element is in this list, it can only be in the left of of this list. This means that we do not have to search the right half at all.

It should be noted that the first half of the list is also a list sorted in acending order. As such, we can apply the same logic there.

I used binary search here as an example because it is the cleariest example of a searching technique that is built upon a specific sorted order, but this applies to most searching techniques. This is because shortucts for searching for an element can only be discovered after you understand that your elements are sorted in a certain order. 


TODO: Add exercises to find elements in a list
TODO: Add exercises to compare objects and sort them

# Chatper X + 1

Up until now, we have been talking about sorting on a single dimention. What I means by that is, we've taken a vector of items of type X and type X only has one value to sort by and we have sorted the vector. This is single dimentional because Type X only has one value to sort by.

For our first step into the multidimentional space, I want us to talk about Tuples.

A Tuple is a finte heterogeneous sequence. Finite sequence meaning that there are a finite number of elements in a tuple, and heterogenous meaning that each element can be of a differnt type.

For example, if I wanted to collect a person's first name and age, I could represent that in a tuple where the first element of the tuple is a &str and the second element of the tuple is a u32.

```rust
	let person: (&str, u32) = ("Marcus", 36);
	
	println!("{:?}", person);
```

Our Person is two demential because we have two properties we can sort by; name and age.

In Rust, Tuples implement the Ord Trait for Tuples where each element of the tuple also implements the Ord Trait (Note: This is true for Tuples of 12 or less elements. 12 is not special and may change in the future). This means that if we have a vector of Tuples, we can sort them. By default, tuples are sorted one item at a time in order from left to right comparing the left element first and, if they are equal, comparing the next element in the tuple.

```rust
	let mut test = vec![
		("a", 1),
		("ab", 0),
		("b", 2),
		("a", 0),
		("ba", 0),
	];
    test.sort();

    println!("{:?}", test);
	
	//output: [("a", 0), ("a", 1), ("ab", 0), ("b", 2), ("ba", 0)]
```

You can replicate the default sort yourself by using the `sort_by` method and the appropriate closure function.

```rust
    let mut test = vec![
		("a", 1),
		("ab", 0),
		("b", 2),
		("a", 0),
		("ba", 0),
	];

    test.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    
	println!("{:?}", test);

	//output: [("a", 0), ("a", 1), ("ab", 0), ("b", 2), ("ba", 0)]
```

In the above example, we use the `cmp` method to compare the first element of Tuples `a` and `b`. The `cmp` method returns an Ordering Enum. The Ordering Enum has a `then` method that allows us to chain orderings together when the initial `cmp` result is not Ordering::Equal.

Though there are multiple dimentions, you can still sort by a single dimention by picking a property to sort on and ignoring the rest. In the following example, we will take a list of employees and sort them by how many years they have been with the company in descending order.

```rust
let mut list = vec![
	("Marcus", 2),
	("Jovanna", 5),
	("Carmen", 2),
	("Christy", 2),
	("Dillon", 0),
	("Jerry", 1)
];

list.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    
println!("{:?}", list);

//output: [("Jovanna", 5), ("Marcus", 2), ("Carmen", 2), ("Christy", 2), ("Jerry", 1), ("Dillon", 0)]
```
When looking at the output of the example, you will notice that employees that have worked the same number of years are not presented in alphabetical order. This is intentional because we ignored any sorting of the employees name.


# TODO: Add exercises

# Chapter X + 2: Structs -- Sorting Your structs
The great thing about sorting structs, is that the rules for sorting structs is the same as the rules for sorting eveything numbers and tuples.

For example, lets create an employee struct with two fields; name, years_of_service.

```rust
struct Employee {
    name: String,
    years_of_service: u32
}
```

This struct holds the same information that was containted in the tuples of our previous example, but there are some differences. One difference is that the struct has field names. For example, when using the tuple we accessed the employee name via its index in the tuple (tuple.0) whereas, for the struct, we would access the name of the employee by calling the name property on the struct instance.

Another difference between the struct version of Employee and the tuple version is that tuple version came with a number of default implementation for Traits and struct version has no default Trait implementations, which means we cannot do things like print instances of employees (Debug Trait) or compare two instances of employees (Ord Trait).

When implementing a Trait for a struct, you have to create an `impl` block that states the Trait you are implementing, the struct you are implementing it for, and, within the block, you must implement all of the mandatory methods for that Trait.

For example, the Debug Trait has one required method that must be implemented. To implment Debug for our Employee Struct, we would do the following:

```rust
use std::fmt;

struct Employee {
    name: String,
    years_of_service: u32
}

impl fmt::Debug for Employee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Employee")
         .field("name", &self.name)
         .field("years_of_service", &self.years_of_service)
         .finish()
    }
}

fn main(){
	let marcus = Employee{name: "Marcus".to_string(), years_of_service: 2};
	println!("{:?}", marcus);
}
```

Implementing the Debug trait once is okay, but I would hate to have to do this for every Struct I create. Luckily, Rust provides us a way to automatically generate commonly used traits. We do this with the derive attribute, which allows us to pass in a list of Traits we would like to generate a default implementation for (Note: the [Rust Book](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) lists the traits that we can use Dervie with).

```rust
#[derive(Debug)]
struct Employee {
    name: String,
    years_of_service: u32
}

fn main(){
	let marcus = Employee{name: "Marcus".to_string(), years_of_service: 2};
	println!("{:?}", marcus);
	
	//output: Employee { name: "Marcus", years_of_service: 2 }
}
```

The Ord trait, which is required for sorting, is a super trait. This mean this trait has its own implementation block for itself and it also requires that you implment other traits as well. The Ord Trait requires that the Eq Trait and the PartialOrd trait be implemented as well (Note: The Eq Trait is also a super trait and it requires the PartialEq triat). Lucky for us, we are allowed to derived the default implementation for all of these traits.

```rust
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Employee {
    name: String,
    years_of_service: u32
}

fn main() {
    let list = vec![
		("Marcus", 2),
		("Jovanna", 5),
		("Carmen", 2),
		("Christy", 2),
		("Dillon", 0),
		("Jerry", 1)
    ];

    let mut list_of_employees: Vec<Employee> = list.iter()
		.map(|tuple| Employee{name: tuple.0.to_string(), years_of_service: tuple.1})
		.collect();

    list_of_employees.sort();
    
	println!("{:#?}", list_of_employees);
	
	//output: [
	//	Employee {
	//		name: "Carmen",
	//		years_of_service: 2,
	//	},
	//	Employee {
	//		name: "Christy",
	//		years_of_service: 2,
	//	},
	//	Employee {
	//		name: "Dillon",
	//		years_of_service: 0,
	//	},
	//	Employee {
	//		name: "Jerry",
	//		years_of_service: 1,
	//	},
	//	Employee {
	//		name: "Jovanna",
	//		years_of_service: 5,
	//	},
	//	Employee {
	//		name: "Marcus",
	//		years_of_service: 2,
	//	},
	//]
}
```

In the above example, we derive all the Traits we need to implement the Ord Trait (including the Ord Trait itself) and, as a result, we are now able to sort a list of Employee instances.

To create the list of employee instance, we actually took the Vector of tuples that had the employee infomation, iterated of it, called a `map` and passed a closure that will take each tuple and use it to create an instance of Employee. Iterators are lazy in Rust meaning that they will not be evaluated until something consumes the iterator (ie calls `next` until there are no more items to iterator over). We do that by calling the `collect` method, can transform any iterator into a collection. 

One question you should ask is, "How does collect know what collection to turn the iterator into?". The trick to the `collect` method is that you have to specify the type of collection you want either by setting the type of the result. Two common ways of doing this is by setting the type for the variable like we did in this example (`let mut list_of_employess: Vec<Employee> = ...`) or by using the turbofish (`.collect::<Vec<Employee>>`).

Once we have the list of employee instances, we call `sort` on the vector. Then we used the pretty print debug syntax (`println(:#?)`) to print the list.

The list is sorted by `name` and then `years_of_service`. The reason as to why it is sorted this way is because the automatically derived implementation of our trait typically start from the first defined field of the struct, apply their logic, and then go on to the next field if needed. In our case, we sorted by `name` first and then `years_of_service` because that was the order of how the fields where defined in the struct.

If we switch the ordering of the fields, you will see that the ordering also changes.

```rust
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Employee {
    years_of_service: u32,
    name: String,
}

fn main() {
    let list = vec![
	("Marcus", 2),
	("Jovanna", 5),
	("Carmen", 2),
	("Christy", 2),
	("Dillon", 0),
	("Jerry", 1)
    ];

    let mut list_of_employees = list.iter()
	.map(|tuple| Employee{name: tuple.0.to_string(), years_of_service: tuple.1})
	.collect::<Vec<Employee>>();

    list_of_employees.sort();
    println!("{:#?}", list_of_employees);    
}

	//output: [
	//	Employee {
	//		years_of_service: 0,
	//		name: "Dillon",
	//	},
	//	Employee {
	//		years_of_service: 1,
	//		name: "Jerry",
	//	},
	//	Employee {
	//		years_of_service: 2,
	//		name: "Carmen",
	//	},
	//	Employee {
	//		years_of_service: 2,
	//		name: "Christy",
	//	},
	//	Employee {
	//		years_of_service: 2,
	//		name: "Marcus",
	//	},
	//	Employee {
	//		years_of_service: 5,
	//		name: "Jovanna",
	//	},
	//]
```

#TODO: highlight the pros and cons of using the default sort. ie, adding a new field will change all the results...

The benifits to using derive to autogenerate trait implementations is that its fast. With one line of code you will be able to compare different instances of your struct and sort a collection of them. 

The downside to this approach is that it's brittle. If you change the ordering of your fields or add a new field to your struct, you will change the way your struct instances are compared and you may alter the sorted order of your struct instance in unintended ways.

Lets walk through implementing the Ord trait for Employee so that we can get a better understanding of the pros and cons of implementing this code ourselves.

To make this process easier to understand, we will implement one trait at a time and, once the implementation is finished, we will remove that trait from the derived list.

If you recall, The Ord Trait is a super requires the Eq Trait and the PartialOrd (Note: also a super trait the requires PartialEq...) trait to be implement for our struct. The Eq Trait is also a super trait that requires the PartialEq trait to be implemented. We will start our implementation from the PartialEq trait.

The PartialEq trait has one required method and that is the `eq` method where we compare one instance of our struct to another instance of our struct and return a boolean to denote whether or not they are eqaul.

```rust
#[derive(Debug, Eq, PartialOrd, Ord)]
struct Employee {
    name: String,
    years_of_service: u32,
}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
	self.name == other.name && self.years_of_service == other.years_of_service
    }
}

fn main() {
    let list = vec![
	("Marcus", 2),
	("Jovanna", 5),
	("Carmen", 2),
	("Christy", 2),
	("Dillon", 0),
	("Jerry", 1)
    ];

    let mut list_of_employees = list.iter()
	.map(|tuple| Employee{name: tuple.0.to_string(), years_of_service: tuple.1})
	.collect::<Vec<Employee>>();

    list_of_employees.sort();
    println!("{:#?}", list_of_employees);
	
	//output: [
	//	Employee {
	//		name: "Carmen",
	//		years_of_service: 2,
	//	},
	//	Employee {
	//		name: "Christy",
	//		years_of_service: 2,
	//	},
	//	Employee {
	//		name: "Dillon",
	//		years_of_service: 0,
	//	},
	//	Employee {
	//		name: "Jerry",
	//		years_of_service: 1,
	//	},
	//	Employee {
	//		name: "Jovanna",
	//		years_of_service: 5,
	//	},
	//	Employee {
	//		name: "Marcus",
	//		years_of_service: 2,
	//	},
	//]
}
```

When implementing the PartialEq Trait, we compared the `name` and the `years_of_service` properties to see if they matched. Once implemented, we removed the PartialEq Trait from the derived Traits list.

The next trait we will implement is the Eq Trait. The Eq trait has no required methods. This means that as along a you implement PartialEq, you can get Eq for free by writing an implementation for an empty block of code.

```rust
#[derive(Debug, PartialOrd, Ord)]
struct Employee {
    name: String,
    years_of_service: u32,
}

impl Eq for Employee {}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
	self.name == other.name && self.years_of_service == other.years_of_service
    }
}

fn main() {
    let list = vec![
	("Marcus", 2),
	("Jovanna", 5),
	("Carmen", 2),
	("Christy", 2),
	("Dillon", 0),
	("Jerry", 1)
    ];

    let mut list_of_employees = list.iter()
	.map(|tuple| Employee{name: tuple.0.to_string(), years_of_service: tuple.1})
	.collect::<Vec<Employee>>();

    list_of_employees.sort();
    println!("{:#?}", list_of_employees);

	//output: [
	//	Employee {
	//		name: "Carmen",
	//		years_of_service: 2,
	//	},
	//	Employee {
	//		name: "Christy",
	//		years_of_service: 2,
	//	},
	//	Employee {
	//		name: "Dillon",
	//		years_of_service: 0,
	//	},
	//	Employee {
	//		name: "Jerry",
	//		years_of_service: 1,
	//	},
	//	Employee {
	//		name: "Jovanna",
	//		years_of_service: 5,
	//	},
	//	Employee {
	//		name: "Marcus",
	//		years_of_service: 2,
	//	},
	//]
}
```

The next Trait we should implement is PartialOrd. This trait has one required method called `partial_cmp` that compares two instances of the type and returns an `Option<Ordering>`. Note that this method is very similar to the `cmp` method that comes from the Ord trait. The difference between the two methods is that `partial_cmp` returns `Option<Ordering>` while `cmp` returns `Ordering`.

In Rust, `Option` is an enum with two variants; `Some` and `None`. You can think of these Variants as the existence of an answer. `Some<T>` means that some answer of type T exists and `None` means that no answer exists.

```rust
use std::cmp::Ordering;

#[derive(Debug, Ord)]
struct Employee {
    name: String,
    years_of_service: u32,
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	Some(self.name.cmp(&other.name).then(self.years_of_service.cmp(&other.years_of_service)))
    }
}

impl Eq for Employee {}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
	self.name == other.name && self.years_of_service == other.years_of_service
    }
}

fn main() {
    let list = vec![
	("Marcus", 2),
	("Jovanna", 5),
	("Carmen", 2),
	("Christy", 2),
	("Dillon", 0),
	("Jerry", 1)
    ];

    let mut list_of_employees = list.iter()
	.map(|tuple| Employee{name: tuple.0.to_string(), years_of_service: tuple.1})
	.collect::<Vec<Employee>>();

    list_of_employees.sort();
    println!("{:#?}", list_of_employees);

	//output: [
	//	Employee {
	//		name: "Carmen",
	//		years_of_service: 2,
	//	},
	//	Employee {
	//		name: "Christy",
	//		years_of_service: 2,
	//	},
	//	Employee {
	//		name: "Dillon",
	//		years_of_service: 0,
	//	},
	//	Employee {
	//		name: "Jerry",
	//		years_of_service: 1,
	//	},
	//	Employee {
	//		name: "Jovanna",
	//		years_of_service: 5,
	//	},
	//	Employee {
	//		name: "Marcus",
	//		years_of_service: 2,
	//	},
	//]
}
```

In our implementation of the PartialOrd Trait, we leveraged the fact that the fields of the struct already implement the Ord Trait and used their `cmp` methods to implement our `partial_cmp` method. Taking advantage of the Ordering Enum methods, we compared the `name` field and then the `years_of_service` field.

Our end goal for implementing these traits was to reach a start where changing the order of the fields in our struct does not change the sorted order of our results. Thought we have not finished implementing all of the traits (Ord trait still needs to be implemented), we have seemingly already reached that goal. When you change the order of the fields of the Employee struct, you will notices that the sorted order remains the same.

```rust
use std::cmp::Ordering;

#[derive(Debug, Ord)]
struct Employee {
    years_of_service: u32,
    name: String,
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	Some(self.name.cmp(&other.name).then(self.years_of_service.cmp(&other.years_of_service)))
    }
}

impl Eq for Employee {}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
	self.name == other.name && self.years_of_service == other.years_of_service
    }
}

fn main() {
    let list = vec![
	("Marcus", 2),
	("Jovanna", 5),
	("Carmen", 2),
	("Christy", 2),
	("Dillon", 0),
	("Jerry", 1)
    ];

    let mut list_of_employees = list.iter()
	.map(|tuple| Employee{name: tuple.0.to_string(), years_of_service: tuple.1})
	.collect::<Vec<Employee>>();

    list_of_employees.sort();
    println!("{:#?}", list_of_employees);
}

//output: [
//	Employee {
//        years_of_service: 2,
//        name: "Carmen",
//    },
//    Employee {
//        years_of_service: 2,
//        name: "Christy",
//    },
//    Employee {
//        years_of_service: 0,
//        name: "Dillon",
//    },
//    Employee {
//        years_of_service: 1,
//        name: "Jerry",
//    },
//    Employee {
//        years_of_service: 5,
//        name: "Jovanna",
//    },
//    Employee {
//        years_of_service: 2,
//        name: "Marcus",
//    },
//]
```

Though we have reached our goal without implementing the Ord Trait, I still feel a sense of uneasiness. At this point, it is probably safe to assume that the autogenerated Ord Trait code relies on the PartialOrd Trait implementation of `partial_cmp` to implement `cmp`. However, why assume this to be true, when we can ensure that it is true by implementing the Ord Trait?

```rust
use std::cmp::Ordering;

#[derive(Debug)]
struct Employee {
    years_of_service: u32,
    name: String,
}

impl Ord for Employee {
    fn cmp(&self, other: &Self) -> Ordering {
	self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	Some(self.name.cmp(&other.name).then(self.years_of_service.cmp(&other.years_of_service)))
    }
}

impl Eq for Employee {}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
	self.name == other.name && self.years_of_service == other.years_of_service
    }
}

fn main() {
    let list = vec![
	("Marcus", 2),
	("Jovanna", 5),
	("Carmen", 2),
	("Christy", 2),
	("Dillon", 0),
	("Jerry", 1)
    ];

    let mut list_of_employees = list.iter()
	.map(|tuple| Employee{name: tuple.0.to_string(), years_of_service: tuple.1})
	.collect::<Vec<Employee>>();

    list_of_employees.sort();
    println!("{:#?}", list_of_employees);


//output: [
//	Employee {
//        years_of_service: 2,
//        name: "Carmen",
//    },
//    Employee {
//        years_of_service: 2,
//        name: "Christy",
//    },
//    Employee {
//        years_of_service: 0,
//        name: "Dillon",
//    },
//    Employee {
//        years_of_service: 1,
//        name: "Jerry",
//    },
//    Employee {
//        years_of_service: 5,
//        name: "Jovanna",
//    },
//    Employee {
//        years_of_service: 2,
//        name: "Marcus",
//    },
//]
}
```

Congratulations! We have implemented the Ord Trait for our Employee struct.

Before we move on, there is something I have to mention about super traits. Given the relationships of the dependencies, it is easy to mistakingly assume that the methods of the sub Traits must be used to implement the super traits. This directional ordering, does next exist. For example, even though the Ord Trait is a super trait that requires the PartialOrd trait, we can use the Order trait to implement PartialOrd.

```rust
use std::cmp::Ordering;

#[derive(Debug)]
struct Employee {
    years_of_service: u32,
    name: String,
}

impl Ord for Employee {
    fn cmp(&self, other: &Self) -> Ordering {
	self.name.cmp(&other.name).then(self.years_of_service.cmp(&other.years_of_service))
    }
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
	Some(self.cmp(&other))
    }
}

impl Eq for Employee {}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
	self.name == other.name && self.years_of_service == other.years_of_service
    }
}

fn main() {
    let list = vec![
	("Marcus", 2),
	("Jovanna", 5),
	("Carmen", 2),
	("Christy", 2),
	("Dillon", 0),
	("Jerry", 1)
    ];

    let mut list_of_employees = list.iter()
	.map(|tuple| Employee{name: tuple.0.to_string(), years_of_service: tuple.1})
	.collect::<Vec<Employee>>();

    list_of_employees.sort();
    println!("{:#?}", list_of_employees);

//output: [
//	Employee {
//        years_of_service: 2,
//        name: "Carmen",
//    },
//    Employee {
//        years_of_service: 2,
//        name: "Christy",
//    },
//    Employee {
//        years_of_service: 0,
//        name: "Dillon",
//    },
//    Employee {
//        years_of_service: 1,
//        name: "Jerry",
//    },
//    Employee {
//        years_of_service: 5,
//        name: "Jovanna",
//    },
//    Employee {
//        years_of_service: 2,
//        name: "Marcus",
//    },
//]
}
```

# TODO: Figure out how to close the section

# Chapter X + ?: Sorting Poker hands

Up until this point, we have been taking a list of items and sorting them by one logical arrangement at a time. Admittedly, this covers most practical cases of sorting, but how do we handle situations where there are multiple logical arrangments that we can use to sort?

Take poker for example. Poker is a game where we want to sort our hand based on the type of poker hand we have. The main reason we do this is because, when we compare hands, we compare the type of poker hand we have and, if the type of hand is the same, we compare the cards in the hand to see which hand is of greater value. This value is denoted by the denoted by the rules of the game.

(Add list of poker hands)

For example, If we are looking at a Tne Pair hand, we can break the hand down into two groups; Two Pairs and The Rest of the Cards. When sorting, we would arrange the cards so that each pair is grouped together, we would make sure that the highest pair is in the front, and we would order the rest of the cards in descending order by card number.

(show an example of sorting a Two Pair hand)


If we had a High Card poker hand, then we would want to sort your hand solely by card number in descreasing order placing our High Card in the front.

(Show an example of sorting High card)

Once we sort the poker hands so that the highest value cards are in the front, we can then sort poker hands of the same type by comparing the cards in the hands from front to back.

(Show an example of comparing two poker hands of the same type)


Once we are able to compare hands of the same type, we can then sort a list of poker hands with the first sort being the poker hand type and, if two hands are of the same type, then by the hands themselves.


## Poker hands examples

Before we make bring our Poker example to life, we need to have cards. A card can be thought of as a struct with a name and a suite. The name will e numbers ranging from 1 to 13 where jack through ace are 10 - 13. The suites can be represented as enums.

```rust
enum Suite {
    Heart,
    Club,
    Spade,
    Diamond,
}

struct Card {
    name: u8,
    suite: Suite,
}

fn main(){}
```

To make the cards visually less confusing, we are going to implement the Debug trait for the Card so that 10 - 13 are shown as jack - ace. The steps to implement the default debug code can be found in the Rust Standard library documentation. In their example, they take a Point struct and manually implement the Debug trait.

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}

let origin = Point { x: 0, y: 0 };

assert_eq!(format!("The origin is: {origin:?}"), "The origin is: Point { x: 0, y: 0 }");
```

The Debug trait has one method called `fmt` that takes in a formatter and returns an Result. In `fmt`, we configure the formatter by passing it the information it needs to represent our object. The object being formatted is a struct, so we use `fmt.debug_struct`, which returns `fmt::DebugStruct` (a helper use to gather information about the struct). We then use the `.fields` methods to populate the field information for our struct. Lastely, we call `finish` to see if our configuation has any issues. It should also be noted that `finish` returns the Result, which is expected by the `fmt` function signature.

When taking this example and modifying it to our needs, we know that, outside of the names and types used, the only difference between what we want and they have is that we want on one of our fields to be represented by a string we derive from its value. For example, if `card.name` is 11, then we want the debug output value to be "J" and not 11. We can accomplish this with a match statement.

TODO: Make sure cards are 2-14, where 11-13 are the face cards and Ace is 14

```rust
enum Suite {
    Heart,
    Club,
    Spade,
    Diamond,
}

struct Card {
    name: u8,
    suite: Suite,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	let name = match self.name {
	    x @ 2 ..=10 => x.to_string(),
	    11 => "J".to_string(),
	    12 => "Q".to_string(),
	    13 => "K".to_string(),
	    14 => "A".to_string(),
	    _ => panic!("number {:?} is not a valid card number", &self.name),
	};

	f.debug_struct("Card")
	    .field("name", &name)
	    .field("suite", &self.suite)
	    .finish()
    }
}

fn main(){}
``` 

In a our match statement, we used 2 special features; the `@` for binding and the `_` to match all other possibilities. So `x @ 2 ..= 10` means that this arm is true if the value we are matching against is in the range of `2..=10`. If so, we will bind the matched value to `x` and then give you access to `x` in your expression code block.

The `_` feature allows us to catch all the values that are not matched by the arms above it. This is needed because `match` expressions are exhaustive in the sense that your arms much cover every possible value for the type that you are trying to match against. For our type of `u8`, all values that are not 2-14 will match against this arm. Because these are not valid values for our cards, I decided to raise an error (panic) if this arm is ever matched.

Given that panics are errors and we know that the earlier you find an error, the easier it is to fix it. We should created a functions to create our Cards and yell at us when try to create an invalid card.

```rust,ignore
impl Card {
    fn new(name: u8, suite: Suite) -> Result<Self, &'static str> {
	match name {
	    x @ 2..=14 => Ok(Card{name, suite}),
	    _ => Err("name must be in the range of 1 - 13"),
	}
    }
}
```
With this associated function called `new`, we can ensure that only valid Cards are created. We do this by returning a Result, which is an enum with two variants; Ok and Err. If we can create a valid card, we return that Card wrapped in an Result::Ok. If we cannot create a valid card, we return an error messaged wrapped in an Result::Err.

Now that we have a way to create our cards, we should create a deck of crads and print them out to make sure everything works as expected.

```rust
use std::fmt;

#[derive(Debug, Clone)]
enum Suite {
    Heart,
    Club,
    Spade,
    Diamond,
}


struct Card {
    name: u8,
    suite: Suite,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	let name = match self.name {
	    x @ 1 ..=10 => x.to_string(),
	    11 => "J".to_string(),
	    12 => "Q".to_string(),
	    13 => "K".to_string(),
	    14 => "A".to_string(),
	    _ => panic!("number {:?} is not a valid card number", &self.name),
	};

	f.debug_struct("Card")
	    .field("name", &name)
	    .field("suite", &self.suite)
	    .finish()
    }
}

impl Card {
    fn new(name: u8, suite: Suite) -> Result<Self, &'static str> {
	match name {
	    x @ 2..=14 => Ok(Card{name, suite}),
	    _ => Err("name must be in the range of 1 - 13"),
	}
    }
}

fn main(){
    let mut cards = Vec::new();
    let suites = vec![Suite::Heart, Suite::Spade, Suite::Diamond, Suite::Club];

    for suite in suites {
        for num in 2..=14 {
            cards.push(Card {
                name: num,
                suite: suite.clone(),
            });
        }
    }

    println!("{:?}", cards);

}
```

When printing our cards, we have to remember that we implemented Debug for Card, but we did not do it for Suite. So, in order to print the suite of the Card, we used derive to create a Debug implementation for Suite. Another trait we dereived for Suite was clone. This was done more our of convenience than need. I say that because each card needs to own its one instance of Suite. We could of manually created a new Suite instance for each card, but it is much easier to create one instance of each Suite variant we need and then use clone to create new instances of that Suite variant as needed.

It is time to sort our cards. Poker is a game where the suite of the card matters for a poker hand (a collection of five cards), but not for the individual cards. Knowing this, we will implement the Ord and Eq trait solely based on the Card name.

```rust
use std::fmt;

#[derive(Debug, Clone)]
enum Suite {
    Heart,
    Club,
    Spade,
    Diamond,
}


struct Card {
    name: u8,
    suite: Suite,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	let name = match self.name {
	    x @ 1 ..=10 => x.to_string(),
	    11 => "J".to_string(),
	    12 => "Q".to_string(),
	    13 => "K".to_string(),
	    14 => "A".to_string(),
	    _ => panic!("number {:?} is not a valid card number", &self.name),
	};

	f.debug_struct("Card")
	    .field("name", &name)
	    .field("suite", &self.suite)
	    .finish()
    }
}

impl Card {
    fn new(name: u8, suite: Suite) -> Result<Self, &'static str> {
	match name {
	    x @ 1..=13 => Ok(Card{name, suite}),
	    _ => Err("name must be in the range of 1 - 13"),
	}
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name).reverse()
    }
}

fn main(){
    let mut cards = Vec::new();
    let suites = vec![Suite::Heart, Suite::Spade, Suite::Diamond, Suite::Club];

    for suite in suites {
        for num in 2..=14 {
            cards.push(Card {
                name: num,
                suite: suite.clone(),
            });
        }
    }
	
	cards.sort();
    println!("{:?}", cards);

}
```

Next, we need to address the poker hand and state with they mean with reference to sorting. These are the various poker hands (add graphic) and I would like to group them by the sorting method we can use to order them.

# Sort by Card name
Technically speaking, High card and straight are the only poker hand in this category because it is the only hand is judge based solely on the card name. That said, sorting by card name is the seconary short for most poker hands. For example, if you are comparing One pair poker hands and both hands have a pair of aces, you will have to look at the rest of the cards in both hands to determine the winner. We do this by sorting the rest of the cards by card name and comparing the hands from left to right one card at a time and whoever has the highest next card wins.


## Sort by Card name and Suite
Royal flush, Straight flush, and flush can all be sorted by card name while giving priority to a single suite. If you only have 5 cards in your hand, then the suite doesn't matter and you can use "Sort by Card name" and ignore the suite. However, if you are in a situation where you have to pick the best 5 cards out of a hand and you know you have a flush have type, then you must remember to prioritize the suite of the flush in your sort.

## Short by Card name with priority to a single card name
Two pair, three pair, and four of a kind fall into this category because we are selecting a single number to put in front of our list (giving them priority) and then sorting the rest of the cards by card name (ex. 33AKQ)

## Sort by Card name while prioritizing 2 card names
This sort is specifically for the poker hand full house where you have three of a kind and two of a kind in the same hand. In terms of comparing two hands, you first check against the three of a kind, and if that is the same, you check against the two of a kind. In theory, sorting by card name should matter here because poker hands are based on your best five cards and none of these 5 cards are sorted by card name, but, in practice, you should still have a consistent way of organizing the rest of the cards in the hand.

# Sorting poker hands

A couple things you may have notice when listing out the strategies of sorting poker hands, is that the sort depends on extra information. We need to know what type of hand we have in order to pick the correcting sort, and then, for most hand types, we need to know what card names and/or suites need to be prioritized in the sort.

To hold this information along with the list of cards, we should create a struct called PokerHand. This struct should know the PokerHand type and have a list of cards that represent the "hand".

```rust
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum PokerHandType {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

#[derive(Debug)]
struct PokerHand {
    cards: Vec<Card>,
    poker_hand_type: Option<HandType>,
}
```

For convience, I made the poker hand type an enum and I ordered variants of the enum from strongest hand type to weakest hand type. The reason I did that is because when we compare poker hands, we first compare the hand type. In order to make this comparision, PokerHandType must implement the following traits; Ord, Eq, PartialOrd, PartialEq. I do not want to write the implementation for all of these traits, when I know I can use the derived version of these traits and still get what I want.In order to make sure the derived implementation of these traits work as I expected it to, I need to make sure that the enum variants are in the order that I want them to be sorted in. (Note: I may want to talk about how derive works for enums. Its the same as with structs, but yeah...) 

## Poker hand 1 vs Poker hand 2

When compaing poker hands, if the poker hand type is the same, we then have to compare the poker hands themselves to determine the winner. When comparing hands, there are three possible results; equal, less than or greater.

We will start with the equal case, but, before we start, lets state what we know.

1. If two poker hands do not have the same type, then they cannot be equal.
2. If two poker hands have the same type, then they may be equal.
3. If two poker hands have the same type, then we have to compare the cards in the hand to determine if they are equal.
4. (assuming 5 cards hands) If two poker hands have the same type, and they have the same cards (comparing card names and not card suite), then the poker hands are equal.

When laying out these statements, we see that, according to statement 4, the cards do not have to be sorted to determine equality. For example, [2 Spades, 2 hearts, 4 diamond, ace clubs, king spades] is equal to [ace spades, king hearts, 4 spades, 2 clubs, 2 diamonds] because, when you count how many times each card name is represented, you get the same value (2:2, 4:1, 14 (ace):1, 13:1). Knowing this means that we can use a hashmap to count our cards and then use a hashmap to see of the list of cards are equal.

```rust,noplaygound
impl PartialEq for PokerHand {
    fn eq(&self, other: &Self) -> bool {
	match self.poker_hand_type == other.poker_hand_type {
	    true => {
		if self.cards.len() != other.cards.len() {
		    false
		} else {
		    let mut counter_1 = HashMap::new();
		    for card in &self.cards {
			counter_1.entry(card.name).and_modify(|count| *count +=1).or_insert(1);
		    }

		    let mut counter_2 = HashMap::new();
		    for card in &other.cards {
			counter_2.entry(card.name).and_modify(|count| *count += 1).or_insert(1);
		    }

		    counter_1 == counter_2
		}
	    }
	    false => false,
	}
    }
}

impl Eq for PokerHand {}
```

Now that we know how to check to check if two poker hands are equal. We now need to figure out how to determine when one hand is greater than the other.

Though we view the winning hand as the "greater" hand, in sorting, we tend to sort in ascending order. Then means that in order to get the "greater" hand in the first slot (Think of sorting a list of hands), we will say that the "greater" hand is less than the other hand to make sure that the "greater" hand gets prioritized and placed in the first slot.

That knowledge combined with the fact that we have to pick the type of sorting we do based on the type of poker hand we have means that we have to write a custom method to do our sorting for us. Lets write the outline of that method.


```rust,noplayground
impl PokerHand {
    // Assuming a 5 card hand
    fn sort_hand(&mut self) -> Result<(), String>{
		if let Some(poker_hand_type) = &self.poker_hand_type {
			match poker_hand_type {
				PokerHandType::Pair | PokerHandType::ThreeOfAKind | PokerHandType::FourOfAKind => {
					todo()!
				},
				PokerHandType::RoyalFlush | PokerHandType::StraightFlush | PokerHandType::Flush | PokerHandType::Straight | PokerHandType::HighCard => {
					todo()!
				},
				PokerHandType::FullHouse | PokerHandType::TwoPair => {
					todo()!
				},
			}
		} else {
			Err("Cannot sort because no poker hand type is specified".to_string())
		}
    }
}
```

This is our `sort_hand` method. our method returns a Result of an empty value and an Error. The reason we return a result is because that poker hand type field is optional. If we do not know what poker hand type we have, then we do not have enough information to sort our hand. When this happends, we should raise an error. 

We use `if let Some()` to get a reference to the poker hand type. Once obtained, we match against it to figure out what kind of sort algorithm we need to run to properly order the cards. For simplicity, we will assume that we have a 5 card hand.

The match arms of our match statment are used to group the different poker hand type into common algorithms. For example, FullHouse and TwoPair are grouped togather because they both do a priority sort on two numbers (one number being of a higher priority than the other).


We essentially have three different sorting algorthms that we need to implement; priority of two cards, priority of one card, and card name order from high to low, which is the secondary sort for the two other groups. Lucky for us, the "card name order from high to low" is the default sort for the Cards.

Starting with the simpliest algorithm first, we will do the "card name order from high to low" sort.

```rust,noplayground
PokerHandType::RoyalFlush | PokerHandType::StraightFlush | PokerHandType::Flush | PokerHandType::Straight | PokerHandType::HighCard => {
	self.cards.sort();

	if *poker_hand_type == PokerHandType::StraightFlush || *poker_hand_type == PokerHandType::Straight {
		// In the case where the straight is Ace, 5, 4 ,3, 2, we need to list
		// the left by 1 -> 5, 4, 3, 2, Ace
		if self.cards[0].name == 14 && self.cards[1].name == 5 {
			self.cards.rotate_left(1);
		}
	}

	Ok(())
},
```

As mentioned, we can rely on the default sorting order of the cards to provide use with this sort, but there are edge cases. For example, Striaght and Staightflush can have the Ace at the beginning of the run (ace, king, queen, jack, 10) or at the end of the run (5, 4, 3, 2, Ace). When sorting the cards, the Ace will be placed as the highest card in the run, so instead of getting (5,4,3,2,ace), we will have (ace, 5,4,3,2). To correct this edge case, we have to note that every time we see this ordering, we need to rotate all the elements in the list to the left by 1 resulting in (5,4,3,2, ace). Vect's in Rust do have a rotate_left that we can leverage for this.

The next two sorting algorithms will require helper methods. This is because our poker hand type tells us that we need to prioritize a card or 2, but the poker hand type does not tell us which cards to prioritize. So, we will create helper methods to obtain this information.

Next, we will tackle the sorting algorithm for the  poker hand types of Pair, Three of a kind and Four of a kind. The commonality between the poker hand types is that they all prioritize one card name in front and they rest of the cards are sorted in card name order from highest to lowest.

Using the poker hand type to our advantage, we know that Pair, Three of a kind and Four of a kind will only ever have one card name that repeats in their hand. With this in mind, we can write a helper method find the first repeat card in the list and returns that card name value.

```rust,noplayground
    fn find_first_repeat_card_name(&self) -> Option<u8> {
        let mut seen = HashSet::new();

        for card in &self.cards {
            if !seen.contains(&card.name) {
                seen.insert(card.name);
            } else {
                return Some(card.name);
            }
        }
        None
    }

```

Our `find_first_repeat_card_name` method has a immutable reference to self and returns `Option<u8>` (note: u8 is the data type of card name). The reason why we return an Option as appose to directly returning a `u8` is that this method can technically be called at any point in time for any poker hand type and not all poker hand types have repeat cards (Example: Striaght and HighCard). If this method is called when there are not repeat cards, then it should return None.

With regard to finding the first repeat card.name in then list of cards, we can accomplish this with a hashmap and a single iteration over the list. While iterating over the list, we will put every card name we see in the hashmap. If we come across a card name that we have already seen, then that card name is our first repeat card name. In our helper function, you see that once we found the card repeat card name, we return it immediately.

Now that we have our helper method, we can move on to implementing the sorting algorithm.

```rust,noplayground
PokerHandType::Pair | PokerHandType::ThreeOfAKind | PokerHandType::FourOfAKind => {
	let priority_card_name = self
        .find_first_repeat_card_name()
        .expect("Unable to find repeat card name");
    
	self.cards.sort_by(|a, b| {
		if a.name == priority_card_name && b.name == priority_card_name {Ordering::Equal}
		else if a.name == priority_card_name && b.name != priority_card_name {Ordering::Less}
		else if a.name != priority_card_name && b.name == priority_card_name {Ordering::Greater}
		else {a.cmp(&b)}
});

Ok(())
}
```

TODO -> figure out how closures work with pulling in varibles from the outer scope.

In this match arm, we use our helper method to set the `priority_card_name` variable which holds the value of the card name that needs to be prioritized. We then call `sort_by` on the vec of cards and pass in our closure function. The functions list of the cases of `a` or `b` have the same card name as the priority card and then prioritizes the card accordingly. If neither `a` or `b` are have the priorit number, then we fallback to the sort implementation on the cards themselves by call `a.cmp(&b)`, which gives us the "card name order from highest to lowest".
