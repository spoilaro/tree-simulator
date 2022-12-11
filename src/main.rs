use std::fmt;

struct TreeNode {
    value: i32,
}

const MAX_DEPTH: usize = 10;

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value {}", self.value)
    }
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode { value }
    }
}

fn main() {
    // Tree with depth of 'n' needs '2n+1' space in array form
    let mut bin_tree: Vec<TreeNode> = Vec::with_capacity(2 * MAX_DEPTH + 1);

    bin_tree.push(TreeNode::new(2));

    for node in bin_tree.iter() {
        println!("{}", node);
    }
}
