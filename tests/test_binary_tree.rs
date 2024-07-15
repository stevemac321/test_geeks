extern crate test_geeks;
use test_geeks::binary_tree::{BinTree};

#[test]
fn test_binary_tree() {
    let mut tree = BinTree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(3);
        tree.insert(7);
        tree.insert(12);
        tree.insert(18);

        let result = tree.in_order();
        assert_eq!(result, vec![3, 5, 7, 10, 12, 15, 18]);
}
