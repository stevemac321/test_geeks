extern crate test_geeks;
use test_geeks::sum_product::sum_array;
use test_geeks::sum_product::product_array;


#[test]
fn test_sum_array() {
{
    let arr =  [1,2,3,4,5];
    assert_eq!(sum_array(&arr), 15);
}
}

#[test]
fn test_product_array() {
    let arr =  [1,2,3,4,5];
    assert_eq!(product_array(&arr), 120);
}