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

If you recall, The Ord Trait is a super requires the Eq Trait and the PartialOrd trait to be implement for our struct. The Eq Trait is also a super trait that requires the PartialEq trait to be implemented. We will start our implementation from the PartialEq trait.

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
