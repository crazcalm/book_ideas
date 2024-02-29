# Working with Structs
## Super Traits

The Ord Trait, which is requires for sorting, is a super trait. A super trait is a regular Trait with the additional requirement of listing other traits that must implement for this Trait to work. Here is a snippet of the Ord Trait definition:

```rust,noplayground
pub trait Ord: Eq + PartialOrd {
    // Required method
    fn cmp(&self, other: &Self) -> Ordering;
	...
}
```

The `Eq + PartialOrd` part means that the `Eq` and the `PartialOrd` Traits are required for the implementation of the `Ord` trait. Please note that when a super trait lists required Traits, those traits can also be super traits. For example, `Eq` is also a super trait. The `Eq` Trait requires that PartialEq be implemented.

```rust,noplayground
pub trait Eq: PartialEq { }
```

This means that implementing the `Ord` Trait means that we also have to implement the `Eq` Trait, the `PartialEq` Trait and the `PartialOrd` Trait. Lucky for us, we are allowed generate default implements for all of these Traits using derive.

```rust
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Employee {
    name: String,
    years_of_service: u32
}

fn main() {
    let mut list_of_employees = vec![
		Employee{name: "Marcus".to_string(), years_of_service: 2},
		Employee{name: "Jovanna".to_string(), years_of_service: 5},
		Employee{name: "Carmen".to_string(), years_of_service: 2},
		Employee{name: "Christy".to_string(), years_of_service: 2},
		Employee{name: "Dillon".to_string(), years_of_service: 0},
		Employee{name: "Jerry".to_string(), years_of_service: 1},
    ];

    list_of_employees.sort();
    
	println!("{:#?}", list_of_employees);
}
```

