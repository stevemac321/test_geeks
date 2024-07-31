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
#[test]
fn test_binary_tree_mindepth1()
{
  let mut tree1 = BinTree::new();
  tree1.insert(1);
  tree1.insert(3);
  tree1.insert(2);
  tree1.insert(4);
  let result = tree1.min_depth();
  assert_eq!(result, 3);
}
#[test]
fn test_binary_tree_mindepth2()
{
  let mut tree2 = BinTree::new();
  tree2.insert(10);
  tree2.insert(20); 
  tree2.insert(30);
  tree2.insert(40);
  tree2.insert(60);
  tree2.insert(2);
  let result = tree2.min_depth();
  assert_eq!(result, 2);
}

/*
NOTE: I am changing this to count the root

Minimum Depth of a Binary Tree
Difficulty: EasyAccuracy: 42.09%Submissions: 71K+Points: 2
Given a binary tree, find its minimum depth.

Example 1:

Input:
            1
          /   \
         3     2
        /
       4           

Output: 2  (for me its 3)

Explanation:
Minimum depth is between nodes 1 and 2 since
minimum depth is defined as  the number of 
nodes along the shortest path from the root 
node down to the nearest leaf node.

Example 2:

Input:
             10
          /     \
        20       30
          \        \   
          40        60 
                   /
                  2 

Output: 3

Explanation:
Minimum depth is between nodes 10,20 and 40.

Your Task:  
You don't need to read input or print anything. Complete the function minDepth() which takes the root node as an input
parameter and returns the minimum depth.
 

Expected Time Complexity: O(N)
*/
