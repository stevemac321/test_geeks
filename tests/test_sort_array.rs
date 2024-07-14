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
    bubble_sort(&mut arr);
    assert_eq!(arr, ["apple", "banana", "cherry", "date"]);
}

use test_geeks::sort_array::bubble_sort;
#[test]
fn test_insertion_bubble_integers() {
    let mut arr = [6, 8, 10, 1, 9];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1, 6, 8, 9, 10]);
}

#[test]
fn test_insertion_bubble_floats() {
    let mut arr = [3.3, 1.1, 4.4, 2.2];
    bubble_sort(&mut arr);
    assert_eq!(arr, [1.1, 2.2, 3.3, 4.4]);
}

#[test]
fn test_insertion_bubble_strings() {
    let mut arr = ["banana", "apple", "cherry", "date"];
    bubble_sort(&mut arr);
    assert_eq!(arr, ["apple", "banana", "cherry", "date"]);
}

use test_geeks::sort_array::quick_sort;
#[test]
fn test_quick_sort_integers() {
    let mut arr = [6, 8, 10, 1, 9];
    quick_sort(&mut arr);
    assert_eq!(arr, [1, 6, 8, 9, 10]);
}
