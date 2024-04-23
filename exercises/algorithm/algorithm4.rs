/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::{Debug, Display};


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Display + Debug,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        if let Some(ref mut c) = self.root {
            Self::insert_node(c, value)
        }else{
            self.root = Some(Box::new(TreeNode::new(value)))
        }
    }

    fn insert_node(node:&mut Box<TreeNode<T>>, value: T){
        if value < node.value{
            match node.left {
                Some(ref mut l) => Self::insert_node(l, value),
                None => {
                    let new_node = Box::new(TreeNode{
                        value,
                        left:None,
                        right:None
                    });
                    node.left = Some(new_node)
                }
            }
        }else if value > node.value{
            match node.right {
                Some(ref mut r) => Self::insert_node(r, value),
                None => {
                    let new_node = Box::new(TreeNode{
                        value,
                        left:None,
                        right:None
                    });
                    node.right = Some(new_node)
                }
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match self.root {
            Some(ref r) => Self::search_node(r, value),
            _ => false
        }
    }

    fn search_node(node: &Box<TreeNode<T>>, value: T) -> bool{
        if node.value == value{
            true
        }else if node.value < value{
            if let Some(ref right) = node.right{
                Self::search_node(right, value)
            }else{
                false
            }
        }else if node.value > value{
            if let Some(ref left) = node.left{
                Self::search_node(left, value)
            }else{
                false
            }
        }else{
            true
        }
    }

}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_insert(){
        let mut bst =BinarySearchTree::new();
        bst.insert(1);
        bst.insert(3);
    }

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


