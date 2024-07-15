use std::fmt::Debug;

pub struct BNode<T: PartialOrd + PartialEq + Debug> {
    data: T,
    count: i32,
    left: Option<Box<BNode<T>>>,
    right: Option<Box<BNode<T>>>,
}

impl<T: PartialOrd + PartialEq + Debug> BNode<T> {
    fn new(data: T) -> Self {
        BNode {
            data,
            count: 1,
            left: None,
            right: None,
        }
    }
}

pub struct BinTree<T: PartialOrd + PartialEq + Debug + Clone> {
    root: Option<Box<BNode<T>>>,
}

impl<T: PartialOrd + PartialEq + Debug + Clone> BinTree<T> {
    pub fn new() -> Self {
        BinTree {
            root: None,
        }
    }

    pub fn insert(&mut self, data: T) {
        self.root = Self::inner_insert(self.root.take(), data);
    }

    fn inner_insert(node: Option<Box<BNode<T>>>, data: T) -> Option<Box<BNode<T>>> {
        if node.is_none() {
            return Some(Box::new(BNode::new(data)));
        }

        let mut p = node.unwrap();
        if p.data == data {
            p.count += 1;
        } else if data < p.data {
            p.left = Self::inner_insert(p.left.take(), data);
        } else {
            p.right = Self::inner_insert(p.right.take(), data);
        }
        Some(p)
    }

    pub fn print(&self) {
        self.inner_print(&self.root);
    }

    fn inner_print(&self, node: &Option<Box<BNode<T>>>) {
        if let Some(ref n) = node {
            self.inner_print(&n.left);
            println!("{:?}", n.data);
            self.inner_print(&n.right);
        }
    }
    pub fn in_order(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.inner_in_order(&self.root, &mut result);
        result
    }

    fn inner_in_order(&self, node: &Option<Box<BNode<T>>>, result: &mut Vec<T>) {
        if let Some(ref n) = node {
            self.inner_in_order(&n.left, result);
            result.push(n.data.clone());
            self.inner_in_order(&n.right, result);
        }
    }
}


/* 
    fn inner_insert(node: Option<Box<BNode<T>>>, data: T) -> Option<Box<BNode<T>>> {
        match node {
            Some(mut p) => {
                if p.data == data {
                    p.count += 1;
                } else if data < p.data {
                    p.left = Self::inner_insert(p.left.take(), data);
                } else {
                    p.right = Self::inner_insert(p.right.take(), data);
                }
                Some(p)
            }
            None => Some(Box::new(BNode::new(data))),
        }
    }
*/
