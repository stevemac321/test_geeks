/*
Problem Description:
Given an array arr of positive integers, the task is to return the maximum of j - i subjected to the constraint of arr[i] < arr[j] and i < j.

Examples:
Input: arr[] = [1, 10]
Output: 1
Explanation: arr[0] < arr[1] so (j-i) is 1-0 = 1.

Input: arr[] = [34, 8, 10, 3, 2, 80, 30, 33, 1]
Output: 6
Explanation: In the given array arr[1] < arr[7] satisfying the required condition(arr[i] < arr[j]) thus giving the maximum difference of j - i which is 6(7-1).
*/

/*
Steps to the Solution:
1. Initialize a variable `max` to 0 to keep track of the maximum index difference.
2. Iterate through each element in the array using an outer loop with index `i`.
3. For each element `arr[i]`, iterate through the subsequent elements in the array using an inner loop with index `j`.
4. Check if the condition `arr[i] < arr[j]` is met.
5. If the condition is met, calculate the difference `j - i`.
6. Update `max` if `j - i` is greater than the current value of `max`.
7. Return the value of `max` after all iterations.
*/

pub fn max_index_span(arr: &[i32]) ->usize {
    let mut max: usize = 0;

    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if (arr[i] < arr[j]) && (j - i) > max {
                max = j - i;
            }
        }
    }

    max
}
