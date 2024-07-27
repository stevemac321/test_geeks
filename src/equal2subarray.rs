pub fn equal2subarray(_arr: &[i32], _s: i32) ->(usize, usize) {
   if _arr.len() < 1 {
      return (0,0);
   }
   
   for left in 0.._arr.len() {
      let mut total = _arr[left];
      if total == _s {
         return (left + 1,  left + 1);
      }
      for right in left + 1.._arr.len() {
         if total < _s {
            total += _arr[right];
            if total == _s {
             return (left+1, right+1);
            }
         }
      }  
   }
   (0,0)
}

pub fn equal2subarray_opt(arr: &[i32], s: i32) -> (usize, usize) {
   let mut left = 0;
   let mut total = 0;

   for right in 0..arr.len() {
       total += arr[right];
       while total > s && left <= right {
           total -= arr[left];
           left += 1;
       }
       if total == s {
           return (left + 1, right + 1);
       }
   }
   (0, 0)
}




/*
Given an unsorted array arr of size n that contains only non negative integers, find a sub-array (continuous elements) 
that has sum equal to s. You mainly need to return the left and right indexes(1-based indexing) of that subarray.

In case of multiple subarrays, return the subarray indexes which come first on moving from left to right. If no such 
subarray exists return an array consisting of element -1.

Examples:

Input: arr[] = [1,2,3,7,5], n = 5, s = 12
Output: 2 4
Explanation: The sum of elements from 2nd to 4th position is 12.
Input: arr[] = [1,2,3,4,5,6,7,8,9,10], n = 10, s = 15,
Output: 1 5
Explanation: The sum of elements from 1st to 5th position is 15.
Input: arr[] = [7,2,1], n = 3, s = 2
Output: 2 2
Explanation: The sum of elements from 2nd to 2nd position is 2.
Input: arr[] = [5,3,4], n = 3, s = 2
Output: -1
Explanation: There is no subarray with sum 2
Expected Time Complexity: O(n)
Expected Auxiliary Space: O(1)

Constraints:

0 <= arr[i] <= 109
1 <= n <= 105
0 <= s <= 109
*/