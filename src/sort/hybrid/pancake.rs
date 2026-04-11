/// Sorts a slice using pancake sort.
///
/// Pancake sort repeatedly flips the largest element to its correct position.
///
/// # Performance
///
/// * **Time Complexity**: O(n²)
/// * **Space Complexity**: O(1)
/// * **Stability**: Unstable
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
