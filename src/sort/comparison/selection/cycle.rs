/// # Cycle Sort
///
/// An **in-place**, non-stable comparison sort that is theoretically optimal 
/// in terms of the total number of writes to the original array.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n²) | Always performs a complete set of cycles. |
/// | **Average Time** | O(n²) | Inefficient for large datasets. |
/// | **Worst Time** | O(n²) | Quadratic complexity in all cases. |
/// | **Space Complexity** | O(1) | Does not require any additional storage. |
///
/// ## Properties
///
/// * **Not-Stable:** Does not preserve the relative order of equal elements.
/// * **Not-adaptive:** Does not benefit from pre-sorted data.
/// * **In-place:** Minimizes memory writes by moving each element at most once.
///
/// ## Example
///
/// ```rust
/// use mycollections::sort::cycle_sort;
///
/// let mut numbers = [10, 1, 3, 9, 7];
/// cycle_sort(&mut numbers);
///
/// assert_eq!(numbers, [1, 3, 7, 9, 10]);
/// ```
pub fn cycle_sort<T: PartialOrd + Copy>(array: &mut [T]) {
    let n = array.len();

    for cycle_start in 0..n - 1 {
        let mut item = array[cycle_start];
        let mut pos = cycle_start;

        // Find where the item belongs
        for i in cycle_start + 1..n {
            if array[i] < item {
                pos += 1;
            }
        }

        // If the item is already in the correct position
        if pos == cycle_start {
            continue;
        }

        // Skip duplicates
        while item == array[pos] {
            pos += 1;
        }

        // Put the item into its new position
        let mut temp = array[pos];
        array[pos] = item;
        item = temp;

        // Rotate the rest of the cycle
        while pos != cycle_start {
            pos = cycle_start;
            for i in cycle_start + 1..n {
                if array[i] < item {
                    pos += 1;
                }
            }

            while item == array[pos] {
                pos += 1;
            }

            let temp = array[pos];
            array[pos] = item;
            item = temp;
        }
    }
}
