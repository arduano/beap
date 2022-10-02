use crate::beap::BeapCoordinate;

fn coords(row: usize, pos: usize) -> BeapCoordinate {
    BeapCoordinate::from_coords(row, pos).unwrap()
}

#[test]
fn test_coord_index() {
    assert_eq!(coords(0, 0).array_index(), 0);
    assert_eq!(coords(1, 0).array_index(), 1);
    assert_eq!(coords(1, 1).array_index(), 2);
    assert_eq!(coords(2, 0).array_index(), 3);
    assert_eq!(coords(2, 1).array_index(), 4);
    assert_eq!(coords(2, 2).array_index(), 5);
    assert_eq!(coords(3, 0).array_index(), 6);
    assert_eq!(coords(3, 1).array_index(), 7);
    assert_eq!(coords(3, 2).array_index(), 8);
    assert_eq!(coords(3, 3).array_index(), 9);
}

#[test]
fn test_coord_from_index() {
    assert_eq!(BeapCoordinate::from_index(0), coords(0, 0));
    assert_eq!(BeapCoordinate::from_index(1), coords(1, 0));
    assert_eq!(BeapCoordinate::from_index(2), coords(1, 1));
    assert_eq!(BeapCoordinate::from_index(3), coords(2, 0));
    assert_eq!(BeapCoordinate::from_index(4), coords(2, 1));
    assert_eq!(BeapCoordinate::from_index(5), coords(2, 2));
    assert_eq!(BeapCoordinate::from_index(6), coords(3, 0));
    assert_eq!(BeapCoordinate::from_index(7), coords(3, 1));
    assert_eq!(BeapCoordinate::from_index(8), coords(3, 2));
    assert_eq!(BeapCoordinate::from_index(9), coords(3, 3));
}

#[test]
fn test_invalid_step() {
    // Stepping around from root
    assert_eq!(coords(0, 0).left_parent(), None);
    assert_eq!(coords(0, 0).right_parent(), None);
    assert_eq!(coords(0, 0).left_child(), coords(1, 0));
    assert_eq!(coords(0, 0).right_child(), coords(1, 1));

    // Stepping around from second row left node
    assert_eq!(coords(1, 0).left_parent(), None);
    assert_eq!(coords(1, 0).right_parent(), Some(coords(0, 0)));
    assert_eq!(coords(1, 0).left_child(), coords(2, 0));
    assert_eq!(coords(1, 0).right_child(), coords(2, 1));

    // Stepping around from second row right node
    assert_eq!(coords(1, 1).left_parent(), Some(coords(0, 0)));
    assert_eq!(coords(1, 1).right_parent(), None);
    assert_eq!(coords(1, 1).left_child(), coords(2, 1));
    assert_eq!(coords(1, 1).right_child(), coords(2, 2));
}
