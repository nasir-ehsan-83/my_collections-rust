/// # Gnome Sort
///
/// A simple sorting algorithm similar to insertion sort, but instead of shifting 
/// elements, it moves them to their correct position through a series of swaps, 
/// stepping backward whenever a swap is performed.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n) | Occurs when the data is already sorted. |
/// | **Average Time** | O(n²) | Standard quadratic time for random data. |
/// | **Worst Time** | O(n²) | Occurs when the slice is sorted in reverse. |
/// | **Space Complexity** | O(1) | Performs swaps in-place without extra memory. |
///
/// ## Properties
/// * **Stability:** ✅ **Stable**. It preserves the relative order of equal keys.
/// * **Adaptive:** Yes. It finishes in linear time for sorted or nearly-sorted input.
/// * **In-place:** Yes. Only requires a single index variable for navigation.
///
/// ## Example
/// ```rust
/// use mycollections::sort::gnome_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// gnome_sort(&mut numbers);
///
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
/// ```
pub fn gnome_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    let mut i = 0;

    while i < len {
        if i == 0 || array[i] >= array[i - 1] {
            i += 1;
        } else {
            array.swap(i, i - 1);
            i -= 1;
        }
    }
}
