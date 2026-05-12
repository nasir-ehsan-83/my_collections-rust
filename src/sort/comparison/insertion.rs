/// # Insertion Sort
///
/// A simple, comparison-based sorting algorithm that builds the final sorted slice 
/// one element at a time. It works similarly to the way you sort playing cards 
/// in your hands.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | $O(n)$ | Extremely efficient for nearly-sorted data. |
/// | **Average Time** | $O(n^2)$ | Standard quadratic time for random data. |
/// | **Worst Time** | $O(n^2)$ | Occurs when the slice is in reverse order. |
/// | **Space Complexity** | $O(1)$ | Zero extra allocations (in-place). |
///
/// ## Properties
/// * **Stability:** ✅ **Stable**. Preserves the original order of equal elements.
/// * **Adaptive:** Yes. Performance improves as the data becomes more sorted.
/// * **In-place:** Yes. Only requires a small, constant amount of memory.
///
/// ## Example
/// ```rust
/// use my_collections::sort::insertion_sort;
///
/// let mut data = [4, 2, 5, 1, 3];
/// insertion_sort(&mut data);
///
/// assert_eq!(data, [1, 2, 3, 4, 5]);
/// ```
pub fn insertion_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    for i in 1..array.len() {
        let key = array[i].clone();
        let mut j = i;

        // Shift elements that are greater than the key to the right
        while j > 0 && array[j - 1] > key {
            array[j] = array[j - 1].clone();
            j -= 1;
        }

        array[j] = key;
    }
}
