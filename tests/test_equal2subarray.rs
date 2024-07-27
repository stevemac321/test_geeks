extern crate test_geeks;
use test_geeks::equal2subarray::equal2subarray;
use test_geeks::equal2subarray::equal2subarray_opt;


/*Input: arr[] = [1,2,3,7,5], n = 5, s = 12
Output: 2 4*/
#[test]
fn test_equal2subarray1() {
   let arr =  [1,2,3,7,5];
   let s = 12;
   let tup = equal2subarray(&arr, s);
   assert_eq!(tup, (2,4));
}
/*Input: arr[] = [1,2,3,4,5,6,7,8,9,10], n = 10, s = 15,
Output: 1 5*/
#[test]
fn test_equal2subarray2() {
   let arr =  [1,2,3,4,5,6,7,8,9,10];
   let s = 15;
   let tup = equal2subarray(&arr, s);
   assert_eq!(tup, (1,5));
}
/* Input: arr[] = [7,2,1], n = 3, s = 2
Output: 2 2*/
#[test]
fn test_equal2subarray3() {
   let arr =  [7,2,1];
   let s = 2;
   let tup = equal2subarray(&arr, s);
   assert_eq!(tup, (2,2));
}
/* Input: arr[] = [5,3,4], n = 3, s = 2
Output: -1 for is return 0 */
#[test]
fn test_equal2subarray4() {
   let arr =  [5,3,4];
   let s = 2;
   let tup = equal2subarray(&arr, s);
   assert_eq!(tup, (0,0));
}
//////////////
/*Input: arr[] = [1,2,3,7,5], n = 5, s = 12
Output: 2 4*/
#[test]
fn test_equal2subarrayopt1() {
   let arr =  [1,2,3,7,5];
   let s = 12;
   let tup = equal2subarray_opt(&arr, s);
   assert_eq!(tup, (2,4));
}
/*Input: arr[] = [1,2,3,4,5,6,7,8,9,10], n = 10, s = 15,
Output: 1 5*/
#[test]
fn test_equal2subarrayopt2() {
   let arr =  [1,2,3,4,5,6,7,8,9,10];
   let s = 15;
   let tup = equal2subarray_opt(&arr, s);
   assert_eq!(tup, (1,5));
}
/* Input: arr[] = [7,2,1], n = 3, s = 2
Output: 2 2*/
#[test]
fn test_equal2subarrayopt3() {
   let arr =  [7,2,1];
   let s = 2;
   let tup = equal2subarray_opt(&arr, s);
   assert_eq!(tup, (2,2));
}
/* Input: arr[] = [5,3,4], n = 3, s = 2
Output: -1 for is return 0 */
#[test]
fn test_equal2subarrayopt4() {
   let arr =  [5,3,4];
   let s = 2;
   let tup = equal2subarray_opt(&arr, s);
   assert_eq!(tup, (0,0));
}

