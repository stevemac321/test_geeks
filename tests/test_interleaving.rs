extern crate test_geeks;
use test_geeks::interleaving::interleaving;

#[test]
fn test_interleaving() {
    // Test case where `c` is not an interleaving of `a` and `b`
    {
        let a = ['Y', 'X'];
        let b = ['X'];
        let c = ['X', 'X', 'Y'];
        let result = interleaving(&a, &b, &c);
        assert_eq!(result, false);
    }

    // Test case where `c` is an interleaving of `a` and `b`
    {
        let a = ['X', 'Y'];
        let b = ['X'];
        let c = ['X', 'X', 'Y'];
        let result = interleaving(&a, &b, &c);
        assert_eq!(result, true);
    }

    // Additional test cases

    // Test case where `a` is empty and `b` is the same as `c`
    {
        let a: [char; 0] = [];
        let b = ['X', 'Y', 'Z'];
        let c = ['X', 'Y', 'Z'];
        let result = interleaving(&a, &b, &c);
        assert_eq!(result, true);
    }

    // Test case where `b` is empty and `a` is the same as `c`
    {
        let a = ['X', 'Y', 'Z'];
        let b: [char; 0] = [];
        let c = ['X', 'Y', 'Z'];
        let result = interleaving(&a, &b, &c);
        assert_eq!(result, true);
    }

    // Test case where `c` is longer than the combined length of `a` and `b`
    {
        let a = ['X'];
        let b = ['Y'];
        let c = ['X', 'Y', 'Z'];
        let result = interleaving(&a, &b, &c);
        assert_eq!(result, false);
    }

    // Test case where `c` is empty and `a` and `b` are also empty
    {
        let a: [char; 0] = [];
        let b: [char; 0] = [];
        let c: [char; 0] = [];
        let result = interleaving(&a, &b, &c);
        assert_eq!(result, true);
    }
       
}


/*
A = YX, B = X, C = XXY
Output: 0
Explanation: XXY is not interleaving
of YX and X
Example 2:

Input:
A = XY, B = X, C = XXY
Output: 1
*/