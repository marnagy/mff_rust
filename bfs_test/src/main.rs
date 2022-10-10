use std::collections::VecDeque;

use rand::prelude::*;

fn main() {
    let mut tree = Tree::empty();
    let mut rng = thread_rng();
    for _ in 0..10_000 {
        tree.add(rng.gen::<i8>())
    }

    println!("{:?}", tree);

    // let mut bfs_q = VecDeque::new();
    // bfs(&tree, &mut bfs_q);
}

// fn bfs<T: Ord>(tree: &Tree<T>, q: &mut VecDeque<T>) {
//     if tree.root == None {
//         return;
//     }


// }

#[derive(Debug)]
struct Tree<T: Ord> {
    root: Option<TreeNode<T>>,
}

impl<T: Ord> Tree<T> {
    fn from(value: T) -> Tree<T> {
        let mut tree = Tree::empty();
        tree.add(value);
        tree
    }

    fn empty() -> Tree<T> {
        Tree { root: None }
    }

    fn add(&mut self, value: T) {
        match &mut self.root {
            None => self.root = Some(TreeNode::new(value)),
            Some(node) => node.add(value),
        }
    }

    fn contains(&self, value: T) -> bool {
        match &self.root {
            None => false,
            Some(node) => node.contains(value),
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

#[derive(Debug)]
struct TreeNode<T: Ord> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> TreeNode<T> {
    fn new(value: T) -> TreeNode<T> {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn add(&mut self, value: T) {
        match self.value.cmp(&value) {
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Less => match &mut self.left {
                None => self.left = Some(Box::new(TreeNode::new(value))),
                Some(node) => node.add(value),
            },
            std::cmp::Ordering::Greater => match &mut self.right {
                None => self.right = Some(Box::new(TreeNode::new(value))),
                Some(node) => node.add(value),
            },
        }
    }

    fn contains(&self, value: T) -> bool {
        if value == self.value {
            return true;
        }

        if value < self.value {
            match &self.left {
                None => false,
                Some(node) => node.contains(value),
            }
        } else {
            match &self.right {
                None => false,
                Some(node) => node.contains(value),
            }
        }
    }
}
