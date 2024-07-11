extern crate test_geeks;

#[test]
fn test_max_index_span() {
    let mut nums0 = vec![1, 10];
    assert_eq!(test_geeks::max_index_span::max_index_span(&mut nums0), 1);
    
    let mut nums1 = vec! [34, 8, 10, 3, 2, 80, 30, 33, 1]
;    assert_eq!(test_geeks::max_index_span::max_index_span(&mut nums1), 6);
}
