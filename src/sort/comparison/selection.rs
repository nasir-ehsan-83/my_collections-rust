/// # Selection Sort
///
/// A simple comparison-based algorithm that divides the slice into a sorted and 
/// an unsorted part. It repeatedly selects the smallest element from the unsorted 
/// section and swaps it into its correct position.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n²) | Always scans the remaining unsorted part. |
/// | **Average Time** | O(n²) | Consistent performance regardless of order. |
/// | **Worst Time** | O(n²) | Same number of comparisons as the average case. |
/// | **Space Complexity** | O(1) | Zero extra allocations (in-place). |
///
/// ## Properties
/// * **Stability:** ❌ **Unstable**. Long-distance swaps may reorder equal keys.
/// * **Adaptive:** No. It performs the same number of comparisons for any input.
/// * **In-place:** Yes. Only requires a constant amount of extra memory.
///
/// ## Example
/// ```rust
/// use my_collections::sort::selection_sort;
///
/// let mut data = [4, 2, 5, 1, 3];
/// selection_sort(&mut data);
///
/// assert_eq!(data, [1, 2, 3, 4, 5]);
/// ```
pub fn selection_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    for i in 0..len {
        let mut min_index = i;

        // Find the index of the smallest element in the remaining unsorted portion
        for j in (i + 1)..len {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }

        // Swap the found minimum element with the first element of the unsorted portion
        if min_index != i {
            array.swap(i, min_index);
        }
    }
}
