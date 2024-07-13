pub fn sum_array(arr: &[i32]) ->i32 {
    let mut sum: i32 = 0;

    for i in 0..arr.len() {
        sum += arr[i];
    }
    sum
}
pub fn product_array(arr: &[i32]) ->i32 {
    let mut product: i32 = 1;

    for i in 0..arr.len() {
        product *= arr[i];
    }
    product
}