/// A node in a Binary Search Tree (BST).
///
/// Each node holds a value and optional pointers to its left and right children.
struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> Node<T> {
    /// Creates a new BST node.
    fn new(val: T) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }

    /// Recursively inserts a value into the BST.
    ///
    /// Values smaller than the current node go to the left, while larger or 
    /// equal values go to the right.
    fn insert(&mut self, val: T) {
        if val < self.val {
            match self.left {
                Some(ref mut node) => node.insert(val),
                None => self.left = Some(Box::new(Node::new(val))),
            }
        } else {
            match self.right {
                Some(ref mut node) => node.insert(val),
                None => self.right = Some(Box::new(Node::new(val))),
            }
        }
    }

    /// Performs an in-order traversal of the tree, appending values to the result vector.
    fn in_order(&self, result: &mut Vec<T>)
    where
        T: Clone,
    {
        if let Some(ref left) = self.left {
            left.in_order(result);
        }
        result.push(self.val.clone());
        if let Some(ref right) = self.right {
            right.in_order(result);
        }
    }
}

/// # Tree Sort
///
/// A sorting algorithm that builds a Binary Search Tree (BST) from the input elements 
/// and retrieves them in sorted order via in-order traversal. Performance is 
/// highly dependent on the balance of the underlying tree.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n log n) | Achieved when the tree is perfectly balanced. |
/// | **Average Time** | O(n log n) | Typical for random data distributions. |
/// | **Worst Time** | O(n²) | Occurs when the tree is skewed (sorted input). |
/// | **Space Complexity** | O(n) | Requires extra memory for the tree nodes. |
///
/// ## Properties
/// * **Stability:** ✅ **Stable**. Preserves the relative order of equal elements.
/// * **Adaptive:** No. Building the tree follows a fixed insertion process.
/// * **In-place:** ❌ **No**. Requires additional heap allocations for nodes.
///
/// ## Example
/// ```rust
/// use mycollections::sort::tree_sort;
///
/// let mut numbers =;
/// tree_sort(&mut numbers);
///
/// assert_eq!(numbers,);
/// ```
pub fn tree_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    // Build the tree using the first element as the root
    let mut root = Node::new(array[0].clone());
    for val in array.iter().skip(1) {
        root.insert(val.clone());
    }

    // Traverse the tree in-order to get sorted elements
    let mut sorted_vec = Vec::with_capacity(len);
    root.in_order(&mut sorted_vec);
    
    // Copy the sorted elements back into the original slice
    array.clone_from_slice(&sorted_vec);
}
