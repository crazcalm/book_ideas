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


## How is sorting done in Rust
