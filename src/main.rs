use std::fmt;

struct TreeNode {
    value: i32,
}

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
    let mut bin_tree: Vec<TreeNode> = Vec::new();

    bin_tree.push(TreeNode::new(2));

    for node in bin_tree.iter() {
        println!("{}", node);
    }
}
