/// Sorts a slice in-place using the odd-even sort algorithm.
///
/// Odd-even sort (brick sort) alternates between comparing odd-indexed
/// and even-indexed pairs until the array is sorted.
///
/// # Performance
///
/// * **Time Complexity**: $O(n^2)$ (worst and average), $O(n)$ (best).
/// * **Space Complexity**: $O(1)$.
/// * **Stability**: This implementation is **stable**.
///
/// # Examples
///
/// ```
/// use my_collections::sort::odd_even_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// odd_even_sort(&mut numbers);
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