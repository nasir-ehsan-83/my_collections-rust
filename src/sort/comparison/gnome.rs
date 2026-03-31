/// Sorts a slice in-place using the gnome sort algorithm.
///
/// Gnome sort is similar to insertion sort but uses swapping to move elements
/// into the correct position by stepping backward when necessary.
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
/// use my_collections::sort::gnome_sort;
///
/// let mut numbers = [4, 2, 5, 1, 3];
/// gnome_sort(&mut numbers);
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
