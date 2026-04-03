/// Sorts a slice in-place using the comb sort algorithm.
///
/// Comb sort improves on bubble sort by using a gap larger than 1. The gap 
/// starts large and shrinks by a shrink factor (usually 1.3) until it reaches 1, 
/// effectively eliminating "turtles" (small values at the end of the list).
///
/// # Performance
///
/// * **Time Complexity**: $O(n^2)$ (worst case), $O(n \log n)$ (average case).
/// * **Space Complexity**: $O(1)$ (in-place).
/// * **Stability**: This implementation is unstable.
///
/// # Examples
///
/// ```
/// let mut numbers = [4, 2, 5, 1, 3];
/// comb_sort(&mut numbers);
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
