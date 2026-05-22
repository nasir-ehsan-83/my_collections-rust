/// # Comb Sort
///
/// An improvement over bubble sort that uses a shrinking gap to eliminate "turtles" 
/// (small values at the end of the slice). It's significantly faster than standard 
/// bubble sort on large datasets.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n log n) | Typical for already partially sorted data. |
/// | **Average Time** | O(n log n) | Much more efficient than quadratic sorts. |
/// | **Worst Time** | O(n²) | Rare, but possible with specific data patterns. |
/// | **Space Complexity** | O(1) | Zero extra allocations (in-place). |
///
/// ## Properties
/// * **Unstable:** Does not guarantee order of equal elements.
/// * **Not-adaptive:** The gap sequence is predetermined by the shrink factor.
/// * **In-place:** Operates directly on the input slice.
///
/// ## Example
/// ```rust
/// use mycollections::sort::comb_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// comb_sort(&mut numbers);
///
/// assert_eq!(numbers, [1, 2, 3, 4, 5]);
/// ```
pub fn comb_sort<T: PartialOrd>(array: &mut [T]) {
    let len = array.len();
    if len < 2 {
        return;
    }

    let mut gap = len;
    let shrink = 1.3;
    let mut sorted = false;

    while !sorted {
        gap = (gap as f64 / shrink).floor() as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        for i in 0..len - gap {
            if array[i] > array[i + gap] {
                array.swap(i, i + gap);
                sorted = false;
            }
        }
    }
}
