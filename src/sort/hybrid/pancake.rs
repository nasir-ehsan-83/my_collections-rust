/// # Pancake Sort
///
/// A **comparison-based** sorting algorithm that sorts a sequence by 
/// repeatedly flipping prefixes of the slice. It is a variation of 
/// selection sort that operates under the constraint of prefix reversals.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n) | Elements are already in the correct order. |
/// | **Average Time** | O(n²) | Requires scanning and flipping for each position. |
/// | **Worst Time** | O(n²) | Each element needs two flips to reach its position. |
/// | **Space Complexity** | O(1) | Only a few variables needed for indices. |
///
/// ## Properties
///
/// * **Not-Stable:** Prefix reversals do not preserve relative order.
/// * **Not-adaptive:** The algorithm always performs a full scan for the maximum.
/// * **In-place:** Modifies the original slice through reversals.
///
/// ## Example
///
/// ```rust
/// use mycollections::sort::pancake_sort;
///
/// let mut numbers = [3, 1, 2, 4, 0];
/// pancake_sort(&mut numbers);
///
/// assert_eq!(numbers, [0, 1, 2, 3, 4]);
/// ```
pub fn pancake_sort<T: Ord>(array: &mut [T]) {
    let len = array.len();

    for size in (1..=len).rev() {
        let mut max_idx = 0;
        for i in 1..size {
            if array[i] > array[max_idx] {
                max_idx = i;
            }
        }

        array[..=max_idx].reverse();
        array[..size].reverse();
    }
}
