extern crate test_geeks;

#[test]
fn test_remove_duplicates() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(test_geeks::remove_dups::remove_duplicates(&mut nums), 2);
    assert_eq!(&nums[..2], &[1, 2]);

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(test_geeks::remove_dups::remove_duplicates(&mut nums), 5);
    assert_eq!(&nums[..5], &[0, 1, 2, 3, 4]);
    assert_eq!(&nums[..5], &[0, 1, 2, 3, 4]);
}
