/// A trait for types that can be used in counting sort.
///
/// Types implementing this trait must be convertible to and from a non-negative index.
pub trait CountingSortable: Copy {
    /// Converts the value to an index.
    fn to_index(&self) -> usize;

    /// Creates a value from an index.
    fn from_index(index: usize) -> Self;
}

impl CountingSortable for usize {
    fn to_index(&self) -> usize {
        *self
    }

    fn from_index(index: usize) -> Self {
        index
    }
}
/// # Counting Sort
///
/// An integer sorting algorithm that works by counting the occurrences of each
/// unique value and reconstructing the slice in-place using those counts.
///
/// This implementation is generic over types that implement `CountingSortable`.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n + k) | Constant time relative to the element count and range. |
/// | **Average Time** | O(n + k) | Efficient when the value range $k$ is not significantly larger than $n$. |
/// | **Worst Time** | O(n + k) | Performance remains uniform regardless of data distribution. |
/// | **Space Complexity** | O(k) | Requires temporary memory proportional to the value range $k$. |
///
/// ## Properties
///
/// * **Unstable**. This specific in-place implementation does not preserve relative order.
/// * **Not-adaptive:** The frequency counting loop runs for all input structures.
/// * **In-place:** Modifies the input slice directly without allocating an output array.
///
/// ## Example
///
/// ```rust
/// use mycollections::sort::counting_sort;
///
/// let mut numbers = [4usize, 2, 2, 8, 3, 3, 1];
/// counting_sort(&mut numbers);
///
/// assert_eq!(numbers, [1, 2, 2, 3, 3, 4, 8]);
/// ```

pub fn counting_sort<T: CountingSortable>(array: &mut [T]) {
    if array.len() < 2 {
        return;
    }

    let max = array.iter().map(|x| x.to_index()).max().unwrap();
    let mut count = vec![0; max + 1];

    // Count occurrences
    for &item in array.iter() {
        count[item.to_index()] += 1;
    }

    // Reconstruct array
    let mut index = 0;
    for value in 0..=max {
        for _ in 0..count[value] {
            array[index] = T::from_index(value);
            index += 1;
        }
    }
}