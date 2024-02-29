# Working with Structs
## Structs and Traits

The great thing about structs is that everything we learned about numbers and tuples applies to structs.

For example, here is an employee struct with two fields; name and years_of_service.

```rust
struct Employee {
    name: String,
    years_of_service: u32
}
```

This struct has two fields. One field is of type `String` and the other is of type `u32`, which is similar to a Tuple in one of our previous examples. The difference between these two data types is that the Struct has named fields. For example, when using the tuple we accessed the employee name via its index in the tuple (tuple.0) whereas, for the struct, we would access the name of the employee by calling the name property on the struct instance.

Another difference between Structs and Tuples, is that Tuple come with a number of default implementations of Traits. For example, we were able to print out debug information for our Tuples because Tuple implements the Debug Trait. We were also able to sort a list of Tuples because Tuples have a default implementation of the Ord Trait. Our Stucts do not have default implementations of Traits. So, if you want the functionality of a Trait, you have to implement it.

When implementing a Trait for a Struct, you have to create an `impl` block that states the Trait you are implementing, the struct you are implementing it for, and, within the vlock, you mugt implement all of the mandatory methods for that Trait.

For example, The Debug Trait has one required method. To implement Debug for our Employee Struct, we would do the following:

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
`fmt` is the required method that must be implemented for the Debug Trait. Its function signature is defined by the trait (You can find it in the documention of the Trait). When implementing a required method, we are responsible for filling out the function body. For the `fmt` method, our function body is filled with a lot of boilerplate code that can be pulled from the Debug Trait documentation. That said, from a high level, we are telling the formatter (`fmt::Formatter`) how to read our object. `f.debug_struct` tells the formatter that our object is a Struct. We pass in the name of the Struct, so that the formatter knows what to call our Struct. We then use the `fields` method to inform the formatter what our field names are called and where it can find the values for those fields. Lastly, we call `finish` to tell the formatter that we will not give you and more information and that you can now do whatever processing on your end to create our Debug string.

Implementing the Debug trait for every Struct you create can become tetious. Luckily, Rust provides us a way to automatically generate commonly used traits. We do this with the derived attribute, which allows us to pass in a list of Trait we would like to generate default implementations for (Note: the [Rust Book](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) lists the traits that we can use Dervie with).

Here is how we would use `derive` to generate a default implementation for the Debug Trait:
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

Using derive to generate the implementation of Debug creates the same code that we wrote in our previous example. This means that, for the cases where cases where you do not want to do anything custom in your debug representation, using derive to generate your implementation is a valid choice that will save you time. 
