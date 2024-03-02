# Structs
## Implementating our Traits
Lets walk through the work needed to implement the Ord Trait for our Employee Struct so that we can gain a better understanding of what is happening. We will approach this systematically by writing unit tests for the desired order and then implementing each trait one at a time while removing it from the derived list.

For this example, we want our list of Employees to be sorted by years_of_service and then by name. With that in mind, here is the unit test that we will use to verify that our changes have not affected our sorting.

```rust
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Employee {
    years_of_service: u32,
    name: String,
}

fn main() {}

#[cfg(test)]
mod test {
    use super::Employee;

    #[test]
    fn test_sort() {
        let expected = vec![
            Employee {
                years_of_service: 0,
                name: "Dillon".to_string(),
            },
            Employee {
                years_of_service: 1,
                name: "Jerry".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Carmen".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Christy".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Marcus".to_string(),
            },
            Employee {
                years_of_service: 5,
                name: "Jovanna".to_string(),
            },
        ];

        let mut list_of_employees = vec![
            Employee {
                name: "Marcus".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Jovanna".to_string(),
                years_of_service: 5,
            },
            Employee {
                name: "Carmen".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Christy".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Dillon".to_string(),
                years_of_service: 0,
            },
            Employee {
                name: "Jerry".to_string(),
                years_of_service: 1,
            },
        ];

        list_of_employees.sort();

        assert_eq!(list_of_employees, expected);
    }
}
```

The Ord Trait is a super Trait that requires the Eq Trait and the PartialOrder Trait to be implemented. The Eq Trait is also a super Trait and it requires PartialEq to be implemented. We'll start by implementing PartialEq.

PartialEq is a Trait that has one required method called `eq` that allows us to compare one instance of the Struct to another instance of the Struct. This method will return a boolean to denote whether or not these Struct instance are equal. With regards to implementation, we will compare the year_of_service field and the name field of the two struct instances and return the resulting boolean.

```rust
#[derive(Debug, Eq, PartialOrd, Ord)]
struct Employee {
    years_of_service: u32,
    name: String,
}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.years_of_service == other.years_of_service && self.name == other.name
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::Employee;

    #[test]
    fn test_sort() {
        let expected = vec![
            Employee {
                years_of_service: 0,
                name: "Dillon".to_string(),
            },
            Employee {
                years_of_service: 1,
                name: "Jerry".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Carmen".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Christy".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Marcus".to_string(),
            },
            Employee {
                years_of_service: 5,
                name: "Jovanna".to_string(),
            },
        ];

        let mut list_of_employees = vec![
            Employee {
                name: "Marcus".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Jovanna".to_string(),
                years_of_service: 5,
            },
            Employee {
                name: "Carmen".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Christy".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Dillon".to_string(),
                years_of_service: 0,
            },
            Employee {
                name: "Jerry".to_string(),
                years_of_service: 1,
            },
        ];

        list_of_employees.sort();

        assert_eq!(list_of_employees, expected);
    }
}
```

In the example, we implement the PrartialEq Trait for Employee and removed that Trait from the derived list. Feel free to run the unit test to make sure everything still works as expected.

Next, we will Implement the Eq Trait. The Eq trait has no required methods. This means that as along a you implement PartialEq, you can get Eq for free by writing an implementation for an empty block of code.

```rust
#[derive(Debug, PartialOrd, Ord)]
struct Employee {
    years_of_service: u32,
    name: String,
}

impl Eq for Employee {}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.years_of_service == other.years_of_service && self.name == other.name
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::Employee;

    #[test]
    fn test_sort() {
        let expected = vec![
            Employee {
                years_of_service: 0,
                name: "Dillon".to_string(),
            },
            Employee {
                years_of_service: 1,
                name: "Jerry".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Carmen".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Christy".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Marcus".to_string(),
            },
            Employee {
                years_of_service: 5,
                name: "Jovanna".to_string(),
            },
        ];

        let mut list_of_employees = vec![
            Employee {
                name: "Marcus".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Jovanna".to_string(),
                years_of_service: 5,
            },
            Employee {
                name: "Carmen".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Christy".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Dillon".to_string(),
                years_of_service: 0,
            },
            Employee {
                name: "Jerry".to_string(),
                years_of_service: 1,
            },
        ];

        list_of_employees.sort();

        assert_eq!(list_of_employees, expected);
    }
}
```

The next Trait we will implement is PartialOrd. This Trait has one required method called `partial_cmp` that compares two instance of the same type and returns an `Option<Ordering>`. Please note that this method is very similar to the `cmp` method that comes from the Ord Trait. The difference between these two methods is that `cmp` returns `Ordering` and `partial_cmp` returns `Option<Ordering>`.

```rust
use std::cmp::Ordering;

#[derive(Debug, Ord)]
struct Employee {
    years_of_service: u32,
    name: String,
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.years_of_service
                .cmp(&other.years_of_service)
                .then(self.name.cmp(&other.name)),
        )
    }
}

impl Eq for Employee {}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.years_of_service == other.years_of_service && self.name == other.name
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::Employee;

    #[test]
    fn test_sort() {
        let expected = vec![
            Employee {
                years_of_service: 0,
                name: "Dillon".to_string(),
            },
            Employee {
                years_of_service: 1,
                name: "Jerry".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Carmen".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Christy".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Marcus".to_string(),
            },
            Employee {
                years_of_service: 5,
                name: "Jovanna".to_string(),
            },
        ];

        let mut list_of_employees = vec![
            Employee {
                name: "Marcus".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Jovanna".to_string(),
                years_of_service: 5,
            },
            Employee {
                name: "Carmen".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Christy".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Dillon".to_string(),
                years_of_service: 0,
            },
            Employee {
                name: "Jerry".to_string(),
                years_of_service: 1,
            },
        ];

        list_of_employees.sort();

        assert_eq!(list_of_employees, expected);
    }
}
```

When implementing the `partial_cmp` method, we can leverage the fact that each of the field types implement Ord. As such, can compare the `years_of_service` field and then the `name` field using their types `cmp` and chain the comparisions with the `then` method on the `Ordering` enum that is returned from the `cmp` method. We wrap all of this in a `Some` enum because we need to return `Some<Ordering>`. Also, because the `Ordering` Enum is explicitly being mentioned in our function signature, we need to make sure we import it at the top of the file.

At this point in time, we have implemented enough Traits to prevent the changing of Struct field order from altering our sorting algorithm. To demostraite that, we will switch the ordering of our Employee Struct fields so that `name` comes before `years_of_service`.

```rust
use std::cmp::Ordering;

#[derive(Debug, Ord)]
struct Employee {
    name: String,
    years_of_service: u32,
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.years_of_service
                .cmp(&other.years_of_service)
                .then(self.name.cmp(&other.name)),
        )
    }
}

impl Eq for Employee {}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.years_of_service == other.years_of_service && self.name == other.name
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::Employee;

    #[test]
    fn test_sort() {
        let expected = vec![
            Employee {
                years_of_service: 0,
                name: "Dillon".to_string(),
            },
            Employee {
                years_of_service: 1,
                name: "Jerry".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Carmen".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Christy".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Marcus".to_string(),
            },
            Employee {
                years_of_service: 5,
                name: "Jovanna".to_string(),
            },
        ];

        let mut list_of_employees = vec![
            Employee {
                name: "Marcus".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Jovanna".to_string(),
                years_of_service: 5,
            },
            Employee {
                name: "Carmen".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Christy".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Dillon".to_string(),
                years_of_service: 0,
            },
            Employee {
                name: "Jerry".to_string(),
                years_of_service: 1,
            },
        ];

        list_of_employees.sort();

        assert_eq!(list_of_employees, expected);
    }
}
```

The last Trait we have to implement is the Ord Trait. This Trait has one required method called `cmp`. Given the similarities of the `cmp` method and the `partial_eq` method, we can implement `cmp` using `partial_eq`.

```rust
use std::cmp::Ordering;

#[derive(Debug)]
struct Employee {
    name: String,
    years_of_service: u32,
}

impl Ord for Employee {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.years_of_service
                .cmp(&other.years_of_service)
                .then(self.name.cmp(&other.name)),
        )
    }
}

impl Eq for Employee {}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.years_of_service == other.years_of_service && self.name == other.name
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::Employee;

    #[test]
    fn test_sort() {
        let expected = vec![
            Employee {
                years_of_service: 0,
                name: "Dillon".to_string(),
            },
            Employee {
                years_of_service: 1,
                name: "Jerry".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Carmen".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Christy".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Marcus".to_string(),
            },
            Employee {
                years_of_service: 5,
                name: "Jovanna".to_string(),
            },
        ];

        let mut list_of_employees = vec![
            Employee {
                name: "Marcus".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Jovanna".to_string(),
                years_of_service: 5,
            },
            Employee {
                name: "Carmen".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Christy".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Dillon".to_string(),
                years_of_service: 0,
            },
            Employee {
                name: "Jerry".to_string(),
                years_of_service: 1,
            },
        ];

        list_of_employees.sort();

        assert_eq!(list_of_employees, expected);
    }
}
```
Congratulations! We have implemented the Ord Trait for our Employee struct.

Before we move on, there is something I have to mention about super traits. Given the relationships of the dependencies, it is easy to mistakingly assume that the methods of the sub Traits must be used to implement the super traits. This directional ordering, does next exist. For example, even though the Ord Trait is a super trait that requires the PartialOrd trait, we can use the Order trait to implement PartialOrd.

```rust
use std::cmp::Ordering;

#[derive(Debug)]
struct Employee {
    name: String,
    years_of_service: u32,
}

impl Ord for Employee {
    fn cmp(&self, other: &Self) -> Ordering {
        self.years_of_service
            .cmp(&other.years_of_service)
            .then(self.name.cmp(&other.name))
    }
}

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Employee {}

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.years_of_service == other.years_of_service && self.name == other.name
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::Employee;

    #[test]
    fn test_sort() {
        let expected = vec![
            Employee {
                years_of_service: 0,
                name: "Dillon".to_string(),
            },
            Employee {
                years_of_service: 1,
                name: "Jerry".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Carmen".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Christy".to_string(),
            },
            Employee {
                years_of_service: 2,
                name: "Marcus".to_string(),
            },
            Employee {
                years_of_service: 5,
                name: "Jovanna".to_string(),
            },
        ];

        let mut list_of_employees = vec![
            Employee {
                name: "Marcus".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Jovanna".to_string(),
                years_of_service: 5,
            },
            Employee {
                name: "Carmen".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Christy".to_string(),
                years_of_service: 2,
            },
            Employee {
                name: "Dillon".to_string(),
                years_of_service: 0,
            },
            Employee {
                name: "Jerry".to_string(),
                years_of_service: 1,
            },
        ];

        list_of_employees.sort();

        assert_eq!(list_of_employees, expected);
    }
}
```

In the above example, we took the comparison logic out of `partial_eq` method and put it into the `cmp` method. Then we called the `cmp` method within the `partial_eq` method. Running our unit tests shows us that this change did not affect our sorting order.
