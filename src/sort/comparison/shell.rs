/// # Shell Sort
///
/// An optimization of insertion sort that allows the exchange of elements far apart. 
/// It uses a decreasing gap sequence to sort sub-lists, effectively moving 
/// "out-of-place" elements to their destinations faster than standard quadratic sorts.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | $O(n \log n)$ | Typical for already partially sorted data. |
/// | **Average Time** | $O(n \log^2 n)$ | Highly dependent on the gap sequence used. |
/// | **Worst Time** | $O(n^2)$ | The upper bound for most standard gap sequences. |
/// | **Space Complexity** | $O(1)$ | Performs swaps in-place with zero extra memory. |
///
/// ## Properties
/// * **Stability:** ❌ **Unstable**. Long-range swaps can reorder equal elements.
/// * **Adaptive:** Yes. Performance scales with the initial order of the data.
/// * **In-place:** Yes. Operates directly on the input slice.
///
/// ## Example
/// ```rust
/// use my_collections::sort::shell_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// shell_sort(&mut numbers);
///
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
/// ```
pub fn shell_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    // Initialize the gap sequence (Shell's original: n/2, n/4, ...)
    let mut gap = len / 2;

    while gap > 0 {
        // Perform a gapped insertion sort for this gap size
        for i in gap..len {
            let mut j = i;

            // Shift elements until the correct location for array[i] is found
            while j >= gap && array[j - gap] > array[j] {
                array.swap(j - gap, j);
                j -= gap;
            }
        }

        // Reduce the gap for the next pass
        gap /= 2;
    }
}
