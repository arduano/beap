#![allow(dead_code)]

pub mod animation_util;
mod coordinate;

use std::cmp::Ordering;

pub use coordinate::*;

use self::animation_util::{AnimatedSearch, AnimatedSwap, IgnoreSteps, TrackSteps};

#[derive(Debug, Clone)]
pub struct Beap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> Beap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    // The function to run the bubble up algorithm, while tracking the action it took at each step
    fn bubble_up<S: TrackSteps<AnimatedSwap>>(
        &mut self,
        steps: &mut S,
        mut coord: BeapCoordinate,
    ) {
        loop {
            if coord == BeapCoordinate::zero() {
                // We've reached the top. There is no where else to swap.
                break;
            }

            let left_parent = coord.left_parent();
            let right_parent = coord.right_parent();

            // Find the greater parent. If one parent doesn't exist, then the other is chosen.
            // If neither parent is present, then we break.
            let greater = if let Some(left_parent) = left_parent {
                if let Some(right_parent) = right_parent {
                    // If both indexes are present, return the larger one
                    if self.data[left_parent.array_index()] > self.data[right_parent.array_index()]
                    {
                        left_parent
                    } else {
                        right_parent
                    }
                } else {
                    left_parent
                }
            } else {
                if let Some(right_parent) = right_parent {
                    right_parent
                } else {
                    // This can only be reached if the coordinate is zero
                    break;
                }
            };

            let smaller_index = greater.array_index();
            let index = coord.array_index();

            // Swap if the parent is greater. Otherwise, break.
            if self.data[index] < self.data[smaller_index] {
                steps.add_step(AnimatedSwap {
                    first: coord,
                    second: greater,
                    overwrite: false,
                });
                self.data.swap(index, smaller_index);
                coord = greater;
            } else {
                break;
            }
        }
    }

    // The function to run the sink algorithm, while tracking the action it took at each step
    fn sink<S: TrackSteps<AnimatedSwap>>(
        &mut self,
        steps: &mut S,
        mut coord: BeapCoordinate,
    ) {
        loop {
            let left_child = coord.left_child();
            let right_child = coord.right_child();

            // If the left child is out of bounds then both are out of bounds, therefore break
            if left_child.array_index() >= self.data.len() {
                break;
            }

            // If right is out of bounds, use left child. Otherwise pick the smaller child.
            let smaller = if right_child.array_index() >= self.data.len() {
                left_child
            } else if self.data[left_child.array_index()] < self.data[right_child.array_index()] {
                left_child
            } else {
                right_child
            };

            let smaller_index = smaller.array_index();
            let index = coord.array_index();

            // If the child is smaller, swap to it.
            if self.data[index] > self.data[smaller_index] {
                steps.add_step(AnimatedSwap {
                    first: coord,
                    second: smaller,
                    overwrite: false,
                });
                self.data.swap(index, smaller_index);
                coord = smaller;
            } else {
                break;
            }
        }
    }

    // Step through the beap, starting in the bottom left corner, based on the compare function.
    // This can be used in all sorts of search related functions.
    fn step_through<'a, S: TrackSteps<AnimatedSearch>>(
        &'a self,
        steps: &mut S,
        mut compare: impl FnMut(BeapCoordinate, &'a T) -> Ordering,
    ) -> Option<BeapCoordinate> {
        let mut coord = BeapCoordinate::new(self.depth() - 1, 0);

        loop {
            let value = match self.data.get(coord.array_index()) {
                Some(value) => value,
                None => return None,
            };

            steps.add_step(AnimatedSearch { coord });

            let mut compared = compare(coord, value);

            if compared == Ordering::Less {
                if coord.right_child().array_index() >= self.data.len() {
                    compared = Ordering::Greater;
                }
            }

            match compared {
                Ordering::Equal => return Some(coord),
                Ordering::Greater => {
                    if let Some(parent) = coord.right_parent() {
                        coord = parent;
                    } else {
                        return None;
                    }
                }
                Ordering::Less => {
                    coord = coord.right_child();
                }
            }
        }
    }

    // Insert a new item into the heap
    pub fn insert(&mut self, value: T) {
        self.insert_steps::<IgnoreSteps>(value);
    }
    pub fn insert_steps<S: TrackSteps<AnimatedSwap>>(&mut self, value: T) -> S::WrapOutput<()> {
        let mut steps = S::new();

        // Add the item to the end
        self.data.push(value);

        // Bubble it upwards through the heap
        let coord = BeapCoordinate::from_index(self.data.len() - 1);
        self.bubble_up(&mut steps, coord);

        steps.wrap_output(())
    }

    // Pop the top item off the heap
    pub fn pop_smallest(&mut self) -> Option<T> {
        self.pop_smallest_steps::<IgnoreSteps>()
    }
    pub fn pop_smallest_steps<S: TrackSteps<AnimatedSwap>>(&mut self) -> S::WrapOutput<Option<T>> {
        let mut steps = S::new();

        // Can't pop from an empty heap
        if self.data.is_empty() {
            return steps.wrap_output(None);
        }

        // swap_remove swaps the last element with the one at the given index and returns it
        let first = self.data.swap_remove(0);
        // Add the animation step for the line above
        steps.add_step(AnimatedSwap {
            first: BeapCoordinate::from_index(self.data.len()),
            second: BeapCoordinate::zero(),
            overwrite: true,
        });

        // Sink the now-first element back down thorugh the heap
        self.sink(&mut steps, BeapCoordinate::zero());

        steps.wrap_output(Some(first))
    }

    // Modify a single value at a coordinate, then move it to preserve heap property
    pub fn set_value(&mut self, coord: BeapCoordinate, value: T) -> Option<T> {
        self.set_value_steps::<IgnoreSteps>(coord, value)
    }
    pub fn set_value_steps<S: TrackSteps<AnimatedSwap>>(
        &mut self,
        coord: BeapCoordinate,
        value: T,
    ) -> S::WrapOutput<Option<T>> {
        let mut steps = S::new();

        if coord.array_index() >= self.data.len() {
            return steps.wrap_output(None);
        }

        // Compare the value with the new value
        let diff = self.data[coord.array_index()].cmp(&value);

        let old = std::mem::replace(&mut self.data[coord.array_index()], value);

        // Based on the comparison, run the appropriate algorithm
        match diff {
            Ordering::Greater => self.bubble_up(&mut steps, coord),
            Ordering::Less => self.sink(&mut steps, coord),
            Ordering::Equal => {}
        }

        steps.wrap_output(Some(old))
    }

    // Remove an item at the specified index
    pub fn remove(&mut self, coord: BeapCoordinate) -> Option<T> {
        self.remove_steps::<IgnoreSteps>(coord)
    }
    pub fn remove_steps<S: TrackSteps<AnimatedSwap>>(
        &mut self,
        coord: BeapCoordinate,
    ) -> S::WrapOutput<Option<T>> {
        let mut steps = S::new();

        if coord.array_index() >= self.data.len() {
            return steps.wrap_output(None);
        }

        // Swap the item with the last item
        let last = self.data.swap_remove(coord.array_index());
        // Add the animation step for the line above
        steps.add_step(AnimatedSwap {
            first: BeapCoordinate::from_index(self.data.len()),
            second: coord,
            overwrite: true,
        });

        // Sink the element back down thorugh the heap
        self.sink(&mut steps, coord);

        steps.wrap_output(Some(last))
    }

    // Function for finding an item coordinate by value
    pub fn find_item(&self, value: &T) -> Option<BeapCoordinate> {
        self.find_item_steps::<IgnoreSteps>(value)
    }
    pub fn find_item_steps<S: TrackSteps<AnimatedSearch>>(
        &self,
        item: &T,
    ) -> S::WrapOutput<Option<BeapCoordinate>> {
        let mut steps = S::new();
        let coord = self.step_through(&mut steps, |_, value| value.cmp(item));
        steps.wrap_output(coord)
    }

    // Function for finding an item coordinate by value
    pub fn find_smallest_item_greater_than(&self, value: &T) -> Option<BeapCoordinate> {
        self.find_smallest_item_greater_than_steps::<IgnoreSteps>(value)
    }
    pub fn find_smallest_item_greater_than_steps<S: TrackSteps<AnimatedSearch>>(
        &self,
        greater_than: &T,
    ) -> S::WrapOutput<Option<BeapCoordinate>> {
        let mut steps = S::new();
        let mut found_coord = None;
        let mut item: Option<&T> = None;
        self.step_through(&mut steps, |coord, value| {
            if value > greater_than {
                // If the value is smaller or the current item is none, set the found values
                if item.map(|i| i > value).unwrap_or(true) {
                    found_coord = Some(coord);
                    item = Some(value);
                }

                // Move up if below cutoff
                Ordering::Greater
            } else {
                // Move down if above cutoff
                Ordering::Less
            }
        });

        steps.wrap_output(found_coord)
    }

    pub fn depth(&self) -> usize {
        if self.data.len() == 0 {
            return 0;
        } else {
            BeapCoordinate::from_index(self.data.len() - 1).row() + 1
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn get_index(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_coord(&self, coord: BeapCoordinate) -> Option<&T> {
        self.data.get(coord.array_index())
    }
}
