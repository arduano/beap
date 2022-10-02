# Bi-Parent Heaps

A bi-parent heap (a beap) is a heap data structure that allows for a node to have two parents. This is useful for implementing a priority queue that allows searching for specific values or criteria of values.

For example, a beap can find all values smaller than X and incrememnt them, or find a specific value and remove it from the queue, etc.

The functionality of a beap is very similar to a heap, e.g. bubbling up a value is basically identical, but sinking a value down is different to a binary heap and closer to bubbling up as each node has 2 parents as well as 2 children.

For searching, the beap's heap property guarantees that values of similar size would be located around the same horizontal layer, so searching involves starting in the bottom left and then going right-up or right-down based on whether the node's value is smaller or larger than the value being searched for. If trying to go right-down from a parent that has no child, then you go right-up instead. If going right-up but the node has no right parent, then searching ends.

And just like binary heaps, beaps can also be efficiently stored within arrays.

## Application domain

Beaps are a very flexible data structure that has the following properties:

- Allows searching faster than O(N) time
- Provides the value of either the smallest or the largest value (depending on the configuration) in O(1) time.
- Performs any insertions and deletions below O(N) time
- Performs modifications to existing values in below O(N) time but above O(1) time
- Allows random access to any element with a known coordinate in O(1) time
- Space complexity is the same as a normal variable length array

The difference from binary heaps comes purely with the search time, which is O(N) in binary heaps but below O(N) in beaps. However the ability to search opens up many other possibilities such as finding and incrementing/decrementing/removing specific values.

For example, a beap can be used to implement an efficient "best-fit" allocation algorithm, where finding the best allocation happens below O(N) time where N is the number of empty blocks of different sizes. And of course, in this example, it could also choose between first-fit and worst-fit at the same time too under the same implementation.

## Theoretical complexity

The theoretical time complexity of almost all operations in the beap is O(sqrt(N)). This is because the height of the tree involves taking the square root rather than the log of the node count as each layer increases in size by 1 rather than doubling.

Although search operations happen mostly sideways rather than vertically, the width of the tree is proportional to the height, so the time complexity is still O(sqrt(N)).

The random access complexity is O(1) if the coordinate is known, as it's basically an array in that case. And due to the heap property, the coordinate of the smallest/largest value is always known.

The space complexity is the same as a normal variable length array.

## Implementation complexity

The complexity of the implementation is a bit over the top in some places due to the animation support. However, the actual code is fairly generic and the same code can be used without any animations.

Animation information is abstracted behind the `TrackSteps` trait and different implementations of `TrackSteps` either track and return the steps or ignore them.

The coordinates of the beap are tracked in `beap/coordinate.rs`, which is a nice abstraction for stepping around the tree while making sure that the coordinates are always valid.

`beap.rs` contains all of the main logic, for moving values around the beap, searching for values, and any related function that relies on either of those two parts. E.g. the `find_smallest_item_greater_than` and `find_item` functions rely on the generic search/traverse function `step_through`. Any tree modification function would rely on the `bubble_up` or the `sink` function or both.

There are no external components used in the core implementation, other than the standard library's variable length array, which I am very confident with.

## Compromises and assumptions

There were no compromises or assumptions, other than that the beap nodes have no value parameter and is only the keys.

## Effectiveness of the tests

The tests are spread across 2 files: `tests/coords.rs` and `tests/beap.rs`. The coords tests are making sure that the coordinate system functions correctly, and the beap tests are for the beap.

Coordinates test:

- Converting from a coordinate (row/pos) to array index, and converting from an array index back into the coordinate
- Stepping up and down the tree, and whether the coordinates accurately determine invalid parents.

Beap tests:

- Testing insertions and priority queue removals
- Testing random index removals
- Testing random value increments/decrements
- Finding the coordinate of a value
- Finding the coordinate of the next greatest value from a specified value
- All tests that test removals check for the zero-size edge case
- After any call to the beap's functions, the heap property is validated

The test beap isn't that large, but I avoided using a random number generator to make sure the tests are consistent.

## Insights and observations

All operations in the beap can be reduced to 4 operations:

- Accessing/modifying random values by their coordinate
- Bubbling up a value (moving it to its children to maintain heap property)
- Sinking a value (moving it to its parents to maintain heap property)
- Searching for a value (moving sideways and up/down based on a criteria to find a value)

Also, setting up a good coordinate system abstraction really helps towards implementing these operations easier.
