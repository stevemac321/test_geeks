extern crate test_geeks;
use test_geeks::sort_array::insertion_sort;

#[test]
fn test_insertion_sort_integers() {
    let mut arr = [6, 8, 10, 1, 9];
    insertion_sort(&mut arr);
    assert_eq!(arr, [1, 6, 8, 9, 10]);
}

#[test]
fn test_insertion_sort_floats() {
    let mut arr = [3.3, 1.1, 4.4, 2.2];
    insertion_sort(&mut arr);
    assert_eq!(arr, [1.1, 2.2, 3.3, 4.4]);
}

#[test]
fn test_insertion_sort_strings() {
    let mut arr = ["banana", "apple", "cherry", "date"];
    insertion_sort(&mut arr);
    assert_eq!(arr, ["apple", "banana", "cherry", "date"]);
}