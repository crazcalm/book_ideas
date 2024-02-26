# Common uses for sorting
Sorting is typically done for two reasons; comparing and searching. When comparing list of elements, order matters because equality means that every element at each index is equal. For example, [1,2,3] is not equal to [2,3,1] even though they contain the same elements.

```rust
let a = vec![1,2,3,4];
let b = vec![2,3,1,4];

println("{:?} == {:?} => {}", a, b, a == b);
```
However, if we sort both lists prior to checking for equality, we ensure that the elements of both lists have been ordered in the same manner. This gives us the expectation that, as long as both lists have the same elements, the sorted order of the list should be equal as well.

```rust

let mut a = vec![1,2,3,4];
let mut b = vec![2,3,1,4];

a.sort()
b.sort()

println("{:?} == {:?} => {}", a, b, a == b);
```

## Search
When we talk about search we usually are talking about searching for an element or a group of elements from a collection of elements via a specific criteria. For example, given the list [2,9,3,1,8,7,6,9,0], asking if the number 5 is in the list is a single element search and asking for all the values less than 5 is an example of searching for a group of elements via a specific criteria.

Sorting makes searching easier because, if your collection of elements are sorted, then you know that you can leverage the rules of the logical arrangement to reach your answer faster. For example, sorting [2,9,3,1,8,7,6,9,0] in ascending order gives us [0,1,2,3,4,6,7,8,9]. If we want to check if the number 5 is in the list, we now have a number of ways we can go about it.

A method that would work on both the sorted and unsorted version of the list is the brute force method where we look at every element of the list and check if they are equal to 5. For an unsorted list, this makes sense because there is no guarentee that the next element in the list is not 5. For a sorted list, this seems silly because you end up doing a number of unnecessary checks. For example, if you start from the left most element, which is 0, and move right, you will evenutually reach the number 6. Sorting in acending order means that every number to the right of 6 are equal to or greater than 6, which means that 5 cannot be to the right of 6. Stopping our search here would save us 3 comparisons (7,8,9).

If we started from the right most element, which is the number 9 and moved left we would eventually reach the number 4. Ascending order also means that all the elemement left of 4 are equal to or less than 4, which means that 5 cannot be to the left of 4. Stopping our search here will save us 4 comparisons (0,1,2,3).

Given how much the rules of ascending order tell us about the list, there is no harm in starting our search from an arbitrary location. For example, is we started our search in the middle of the list, which is at number 4, we know that if the number 5 is in the list, that is would be to the right of the number 4. This means that our search algorithm now knows what direction to continue our search in. If your search algorithm decides to check the elemement to the right of the number 4, it will see the number 6 and know that 5 cannot be in this list. In this scenario, our algorithm just saved us 7 comparisons all because we understood how the list was ordered.
