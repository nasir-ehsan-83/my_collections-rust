///# Bubble Sort
///
///An in-place sorting algorithm with an early-exit optimization. It’s designed to be simple and memory-efficient, specifically for small collections or nearly-sorted data.
///
///## Complexity Analysis
///
///
///| Metric | Complexity | Note |
///| :--- | :--- | :--- |
///| **Best Time** | O(n) | Triggered when the slice is already sorted. |
///| **Average Time** | O(n²)$ | Expected for random data. |
///| **Worst Time** | O(n²) | Occurs when the data is in reverse order. |
///| **Space Complexity** | O(1) | Zero extra allocations (in-place). |
///
///## Properties
///*   **Stability:** ✅ **Stable**. Does not swap equal elements.
///*   **Adaptive:** Yes. Returns early if no swaps occur in a pass.
///*   **In-place:** Yes. Only requires a few temporary variables for swapping.
///
///## Example
///```
///use mycollections::sort::bubble_sort;
///
///let mut data = [5, 2, 8, 1, 9];
///bubble_sort(&mut data);
///
///assert_eq!(data, [1, 2, 5, 8, 9]);
///```
pub fn bubble_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    for i in (1..len).rev() {
        let mut swapped = false;

        for j in 0..i {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }

        // Optimization: If no elements were swapped, the array is sorted.
        if !swapped {
            break;
        }
    }
}
