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

/// Sorts a slice by building a Binary Search Tree (BST) and performing an in-order traversal.
///
/// Tree sort inserts all elements into a BST and then retrieves them in-order. 
/// Note: Since a standard BST is used here, performance depends on the input order.
///
/// # Performance
///
/// * **Time Complexity**: $O(n^2)$ (worst case - skewed tree), $O(n \log n)$ (average case).
/// * **Space Complexity**: $O(n)$ (requires additional nodes).
/// * **Stability**: This implementation is stable.
///
/// # Examples
///
/// ```
/// use my_collections::sort::tree_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// tree_sort(&mut numbers);
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
///
/// let mut strings = ["banana", "apple", "cherry"];
/// tree_sort(&mut strings);
/// assert_eq!(strings, ["apple", "banana", "cherry"]);
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
