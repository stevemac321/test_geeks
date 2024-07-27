/*
Interleaved Strings
Difficulty: MediumAccuracy: 24.33%Submissions: 94K+Points: 4
Given strings A, B, and C, find whether C is formed by an interleaving of A and B.

An interleaving of two strings S and T is a configuration such that it creates a new string Y from the concatenation 
substrings of A and B and |Y| = |A + B| = |C|

For example:

A = "XYZ"

B = "ABC"

so we can make multiple interleaving string Y like, XYZABC, XAYBCZ, AXBYZC, XYAZBC and many more so here your task is 
to check whether you can create a string Y which can be equal to C.

Specifically, you just need to create substrings of string A and create substrings B and concatenate them and check 
whether it is equal to C or not.

Note: a + b is the concatenation of strings a and b.

Return true if C is formed by an interleaving of A and B, else return false.

Example 1:

Input:
A = YX, B = X, C = XXY
Output: 0
Explanation: XXY is not interleaving
of YX and X
Example 2:

Input:
A = XY, B = X, C = XXY
Output: 1
Explanation: XXY is interleaving of
XY and X.
Your Task:
Complete the function isInterleave() which takes three strings A, B and C as input and returns true if C is an 
interleaving of A and B else returns false. (1 is printed by the driver code if the returned value is true, 
otherwise 0.)

Expected Time Complexity: O(N * M)
Expected Auxiliary Space: O(N * M)
here, N = length of A, and M = length of B

Constraints:
1 ≤ length of A, B ≤ 100
1 ≤ length of C ≤ 200
*/
pub fn interleaving(a: &[char], b: &[char], c: &[char]) -> bool {
   let len_a = a.len();
   let len_b = b.len();
   let len_c = c.len();

   // Initial Check: Lengths must match
   if len_c != len_a + len_b {
       return false;
   }

   // Special case: If all arrays are empty
   if len_a == 0 && len_b == 0 && len_c == 0 {
       return true;
   }

   // Check if starting and ending characters match
   let start_matches = a.first() == c.first() || b.first() == c.first();
   let end_matches = a.last() == c.last() || b.last() == c.last();

   if !start_matches || !end_matches {
       return false;
   }

   // Function to check if a substring from C matches a given array
   fn matches_substring(c: &[char], arr: &[char], start_idx: usize) -> bool {
       arr.iter().enumerate().all(|(i, &item)| {
           c.get(start_idx + i) == Some(&item)
       })
   }

   // Check if C can be formed starting from A
   if a.first() == c.first() && b.first() == c.get(len_a) {
       if matches_substring(c, a, 0) && matches_substring(c, b, len_a) {
           return true;
       }
   }

   // Check if C can be formed starting from B
   if b.first() == c.first() && a.first() == c.get(len_b) {
       if matches_substring(c, b, 0) && matches_substring(c, a, len_b) {
           return true;
       }
   }

   false
}
