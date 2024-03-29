- Day 12 is a difficult day and is solved using a dynamic programming recursive function, good one to study
- Day 5 requires a very creative take on inputs and outputs to avoid needing trillions of iterations, another good one to study
- Day 10 requires thinking outside of the box and "ray tracing" on the string input
- Day 17 was as painful as pulling teeth, it uses a modified A* pathfinding algorithm
    - A*, vitally, can use anything as a Node in its path. The modified algorithm here uses a multidimensional key for the pathfinding
      which involves a coordinate, a direction used to reach the coordinate, and the number of sequential steps made in that direction. This
      is needed to meet special criteria on the pathfinding (must walk a minimum distance x, can only walk a max distance y) and to meet a
      requirement that each tile has a _cost_ associated with its value, independent of the distance we go. The many variables here make it a
      good example to learn how A* works and how to modify it to meet different constraints
- Day 18, [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula) to find the area of a polygon of arbitrary size using its corner coordinates

This AoC has used a lot of iterators so some helpful info on Map<>
```rust
// What happens when I call map().map().sum()?
// The first map will generate an iterator which internally holds two things, the input iterator and a function signature.
// Every time next() is called on this iterator, it will internally call next() on the iter it holds and pass the Some(val) to its function signature.
// The second map call takes the first map and internally holds onto it as its own iterator and has its own mapped function signature. When
// next() is called on the second map, it will call next() on its iter which actually causes the first iter to call next() on itself, pass that Some(val)
// to its function signature, return that value to the second map iter, which itself will pass that value to its function signature.
// BUT maps are _lazy_. This means that after `Pattern::read_input().iter().map().map()` is called, nothing has been iterated on.
// When sum() is called, the second map iterator will run and internally run the first map iterator
// The first map here takes a function that returns a value, not a reference, so the second map's function signature _must_ take a value and not a reference
Pattern::read_input(input).iter().map(Pattern::get_reflection).map(LineOfReflection::to_usize).sum()
```

- Day 15 implements a HashMap which is really cool, check out solution for a reminder as to how a basic hashmap works
    - A hashmap is a `SomeAddressableStorage<Vec<T>>`
        - When a key/value is inserted
            - The key is hashed to generate an address and the hashmap checks to see if it contains a `Vec<T>` (referred to as a "bucket") at that address
                - If a bucket exists, value `T` is inserted into the `Vec`
                - If a bucket does not exist, a new `Vec<T>` is created at that address with the inserted value as its only value
            - The un-hashed key is still associated with the value so we know what value to use when getting/updating/removing it later on
        - When we access a value by key (to get, update, or remove) the hash of the key is used to determine the address of the `Vec` we search,
          and then that `Vec` is iterated through until we reach the relevant `T`.
