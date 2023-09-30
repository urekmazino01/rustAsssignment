// Define a binary tree node structure
#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0, // Empty tree has depth 0
        Some(node) => {
            // Recursively calculate the depth of the left and right subtrees
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);

            // Return the maximum depth among the left and right subtrees, plus 1 for the current node
            1 + std::cmp::max(left_depth, right_depth)
        }
    }
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode::new(4))),
            right: Some(Box::new(TreeNode::new(5))),
        })),
        right: Some(Box::new(TreeNode::new(3))),
    }));

    let depth = max_depth(root);
    println!("Maximum depth of the tree is: {}", depth);
}
