# List Sync

A system that allows one to sync lists of things (with support for nested lists!) in an efficient manner by generating only the differences between the source and target list. These differences can then be applied to the source list in order to convert it into the target list.

The use case is syncing large lists by sending only the edits over the network.

### Running

The usual.

```bash
$ cargo run
```

### Testing

Still the usual.

```bash
$ cargo test
```

### Under the Hood

This project uses the famous Levenshtein's Algorithm to calculate the "distance" between the two lists. This algorithm is usually associated with calculating the edit distance between strings, however, we generalize it to work on any type that can satisfy the conditions for an equivalence relation (i.e. `a == b` and `a != b` should be mutually exclusive, and the equality operation `==` should be reflexive, symmetric and transitive).

In terms of code, these conditions are made known to the rust compiler by having types that implement the `std::cmp::Eq` trait.

In order to allow our custom type that stores nested lists to play well with the generic implementation of the Levenshtein's Algorithm, we'll need to have this type implement the `Eq` trait as well.

### Asymptotic Complexity

Right now, this implementation is terribly inefficient.

The naive recursive implementation of the Levenshtein Algorthm yields a near unusable time complexity of O(3^n). Thankfully, the DP based implementations do much better at O(n^2).

Right now, the most recommended implementation to use is the memoization based DP implementation, which would, in theory, not calculate distances that it doesn't need to calculate. In practice, the entire table does usually get filled up.

The greatest performance bottleneck is the implementation of the function that checks if two list items are equal. This function is O(1) if the items don't contain more lists, but O(n) when they do (where n is the total number of items contained within this item, including grandchildren and all).

This leads to a total complexity of O(n^3) for calculating the distance when using the DP based implementation of Levenshtein's Algorithm. I intend to move to a O(1) implementation for the equality check to mitigate this.
