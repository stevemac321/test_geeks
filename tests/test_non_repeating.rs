extern crate test_geeks;
use test_geeks::non_repeating::non_repeating;

#[test]
fn test_non_repeating() {
    let nums = vec![1, 2, 3, 2, 1, 4];
    let result = non_repeating(nums);
    assert_eq!(&result, &[3, 4]);
}
#[test]
fn test_non_repeating2() {
    let nums = vec![1, 2, 3, 2, 1, 4, 6, 7, 8, 4];
    let result = non_repeating(nums);
    assert_eq!(&result, &[3, 6, 7, 8]);
}
