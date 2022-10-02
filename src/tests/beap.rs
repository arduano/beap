use crate::beap::{Beap, BeapCoordinate};

fn validate_heap_property(beap: &Beap<i32>) {
    for (i, node) in beap.iter().enumerate() {
        let coord = BeapCoordinate::from_index(i);
        if let Some(left_child_val) = beap.get_coord(coord.left_child()) {
            assert!(node <= left_child_val);
        }
        if let Some(right_child_val) = beap.get_coord(coord.right_child()) {
            assert!(node <= right_child_val);
        }
    }
}

fn make_test_beap() -> Beap<i32> {
    let mut beap = Beap::new();

    validate_heap_property(&beap);
    beap.insert(1);
    validate_heap_property(&beap);
    beap.insert(10);
    validate_heap_property(&beap);
    beap.insert(5);
    validate_heap_property(&beap);
    beap.insert(3);
    validate_heap_property(&beap);
    beap.insert(15);
    validate_heap_property(&beap);
    beap.insert(20);
    validate_heap_property(&beap);
    beap.insert(2);
    validate_heap_property(&beap);
    beap.insert(4);
    validate_heap_property(&beap);
    beap.insert(6);
    validate_heap_property(&beap);
    beap.insert(1);
    validate_heap_property(&beap);
    beap.insert(10);

    beap
}

#[test]
fn test_insert_remove_order() {
    let mut beap = make_test_beap();

    validate_heap_property(&beap);
    assert_eq!(beap.pop_smallest(), Some(1));
    validate_heap_property(&beap);
    assert_eq!(beap.pop_smallest(), Some(1));
    validate_heap_property(&beap);
    assert_eq!(beap.pop_smallest(), Some(2));
    validate_heap_property(&beap);
    assert_eq!(beap.pop_smallest(), Some(3));
    validate_heap_property(&beap);
    assert_eq!(beap.pop_smallest(), Some(4));
    validate_heap_property(&beap);
    assert_eq!(beap.pop_smallest(), Some(5));
    validate_heap_property(&beap);
    assert_eq!(beap.pop_smallest(), Some(6));
    validate_heap_property(&beap);
    assert_eq!(beap.pop_smallest(), Some(10));
    validate_heap_property(&beap);
    assert_eq!(beap.pop_smallest(), Some(10));
    validate_heap_property(&beap);
    assert_eq!(beap.pop_smallest(), Some(15));
    validate_heap_property(&beap);
    assert_eq!(beap.pop_smallest(), Some(20));
    validate_heap_property(&beap);
    assert_eq!(beap.pop_smallest(), None);
    validate_heap_property(&beap);
}

#[test]
fn test_random_remove() {
    let mut beap = make_test_beap();

    // Gets the value at the index, makes sure the removed value is equal,
    // and then makes sure the heap property is still valid.
    let mut remove_assert = |index: usize| {
        let coord = BeapCoordinate::from_index(index);
        let value = beap.get_coord(coord).cloned();
        assert_eq!(beap.remove(coord), value);
        validate_heap_property(&beap);
    };

    // test out of bounds remove
    remove_assert(12);

    remove_assert(10);
    remove_assert(5);
    remove_assert(6);
    remove_assert(2);
    remove_assert(5);
    remove_assert(1);
    remove_assert(0);
    remove_assert(1);
    remove_assert(2);
    remove_assert(1);
    remove_assert(0);

    // test zero capacity remove
    remove_assert(0);
}

#[test]
fn test_random_increment_decrement() {
    let mut beap = make_test_beap();

    // Increments the value at the index, makes sure the returned value is the previous value,
    // and then makes sure the heap property is still valid.
    let mut set_value = |index: usize, change: i32| {
        let coord = BeapCoordinate::from_index(index);
        let value = *beap.get_coord(coord).unwrap();
        assert_eq!(beap.set_value(coord, value + change).unwrap(), value);
        validate_heap_property(&beap);
    };

    set_value(10, -20);
    set_value(5, -10);
    set_value(6, -5);
    set_value(2, -2);
    set_value(5, -1);

    set_value(1, 1);
    set_value(0, 10);
    set_value(1, 20);
    set_value(2, 5);
    set_value(1, 2);
    set_value(0, 1);
}

#[test]
fn test_item_find_index() {
    let beap = make_test_beap();

    // Finds the index of the item and verify that it is correct.
    let find = |value: i32| {
        let coord = beap.find_item(&value).unwrap();
        assert_eq!(beap.get_coord(coord), Some(&value));
    };

    find(1);
    find(2);
    find(3);
    find(4);
    find(5);
    find(6);
    find(10);
    find(15);
    find(20);

    // test nonexistent items
    assert!(beap.find_item(&21).is_none());
    assert!(beap.find_item(&0).is_none());
}

#[test]
fn test_find_next_item_greater_index() {
    let beap = make_test_beap();

    // Finds the index of the item and verify that it is correct.
    let find_next = |value: i32| {
        // Subtract 1 so the exact value is found
        dbg!(value);
        let coord = beap.find_smallest_item_greater_than(&(value - 1)).unwrap();
        assert_eq!(beap.get_coord(coord), Some(&value));
    };

    find_next(1);
    find_next(2);
    find_next(3);
    find_next(4);
    find_next(5);
    find_next(6);
    find_next(10);
    find_next(15);
    find_next(20);

    // test nonexistent items
    assert!(beap.find_item(&21).is_none());
    assert!(beap.find_item(&0).is_none());
}
