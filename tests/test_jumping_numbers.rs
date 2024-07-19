
extern crate test_geeks;
use test_geeks::jumping_numbers;

#[test]
fn test_jumping_numbers() {
   
  let ret1: bool = jumping_numbers::jumping_numbers(1847);
  assert_eq!(ret1, false);
  
  let ret2: bool = jumping_numbers::jumping_numbers(1212);
  assert_eq!(ret2, true);
    
}
  
