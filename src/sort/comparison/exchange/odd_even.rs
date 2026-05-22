/// # Odd-Even Sort (Brick Sort)
///
/// A parallel-friendly sorting algorithm that alternates between comparing odd-indexed 
/// and even-indexed adjacent pairs. It is functionally similar to bubble sort but 
/// structured in two distinct phases.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n) | Achieved if the slice is already sorted. |
/// | **Average Time** | O(n²) | Standard performance for random data. |
/// | **Worst Time** | O(n²) | Occurs when the data is in reverse order. |
/// | **Space Complexity** | O(1) | Constant space, performs swaps in-place. |
///
/// ## Properties
/// * **Stable:** Preserves the relative order of equal elements.
/// * **Adaptive:** Terminates early if no swaps occur in a full cycle.
/// * **In-place:** Operates directly on the input slice.
///
/// ## Example
/// ```rust
/// use mycollections::sort::odd_even_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// odd_even_sort(&mut numbers);
///
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
/// ```
pub fn odd_even_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    let mut sorted = false;

    while !sorted {
        sorted = true;

        // Odd phase
        for i in (1..len - 1).step_by(2) {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                sorted = false;
            }
        }

        // Even phase
        for i in (0..len - 1).step_by(2) {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}