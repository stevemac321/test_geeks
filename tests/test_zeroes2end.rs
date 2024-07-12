extern crate test_geeks;
use test_geeks::zeroes2end::zeroes2end;

#[test]
fn test_zeroes2end() 
{
    {
        let mut arr = [1, 0, 2, 0, 3, 0, 4, 5];
        let newlen = zeroes2end(&mut arr);
        assert_eq!(newlen, 5);
        for i in newlen..arr.len() {
            assert_eq!(arr[i], 0);
        }
    }

    {
        let mut arr = [1, 0, 2, 0, 3, 0, 4, 5, 0, 0, 0,9, 9, 0];
        let newlen = zeroes2end(&mut arr);
        assert_eq!(newlen, 7);
        for i in newlen..arr.len() {
            assert_eq!(arr[i], 0);
        }
    }
    {
        let mut arr = [0,0,0];
        let newlen = zeroes2end(&mut arr);
        assert_eq!(newlen, 0);
        for i in newlen..arr.len() {
            assert_eq!(arr[i], 0);
        }
    }
}