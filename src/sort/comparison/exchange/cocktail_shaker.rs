/// # Cocktail Shaker Sort
///
/// A bidirectional bubble sort that moves the largest element to the end and the smallest 
/// to the beginning in each pass. It effectively reduces the "turtle" problem in standard bubble sort.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n) | Occurs if the slice is already sorted. |
/// | **Average Time** | O(n²) | Typical performance for random shuffles. |
/// | **Worst Time** | O(n²) | Occurs with reversed or highly unsorted data. |
/// | **Space Complexity** | O(1) | Constant space, performs swaps in-place. |
///
/// ## Properties
/// * **Stable:** Maintains the relative order of identical elements.
/// * **Adaptive:** Terminates early if a pass completes without any swaps.
/// * **In-place:** Operates directly on the input slice.
///
/// ## Example
/// ```rust
/// use mycollections::sort::cocktail_shaker_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// cocktail_shaker_sort(&mut numbers);
///
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
/// ```
pub fn cocktail_shaker_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    let mut start = 0;
    let mut end = len - 1;
    let mut swapped = true;

    while swapped {
        swapped = false;

        // Forward pass
        for i in start..end {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        swapped = false;
        end -= 1;

        // Backward pass
        for i in (start..end).rev() {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
            }
        }

        start += 1;
    }
}
