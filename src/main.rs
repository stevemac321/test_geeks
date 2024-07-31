extern crate test_geeks;
use test_geeks::binary_tree::{BinTree};

fn main() {
    println!("{}", "test 1");
    let mut tree1 = BinTree::new();
    tree1.insert(1);
    tree1.insert(3);
    tree1.insert(2);
    tree1.insert(4);
    let result1 = tree1.min_depth();
    
    println!("{}", "test 2");
  let mut tree2 = BinTree::new();
  tree2.insert(10);
  tree2.insert(20); 
  tree2.insert(30);
  tree2.insert(40);
  tree2.insert(60);
  tree2.insert(2);
  let result2 = tree2.min_depth();
  
}
