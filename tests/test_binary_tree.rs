extern crate test_geeks;
use test_geeks::binary_tree::{BinTree};

#[test]
fn test_binary_tree() {
    let mut tree = BinTree::new();
    tree.insert(10);
    tree.insert(5);
    tree.insert(15);
    tree.print();
}
