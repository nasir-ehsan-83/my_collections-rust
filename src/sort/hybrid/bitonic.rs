
/// Sorts a slice using bitonic sort (parallel-friendly).
///
/// Works best when length is a power of two.
///
/// # Performance
///
/// * **Time Complexity**: O(n log² n)
/// * **Space Complexity**: O(log n)
/// * **Stability**: Unstable
pub fn bitonic_sort<T: Ord>(array: &mut [T]) {
    //let len = array.len();
    bitonic_sort_rec(array, true);
}

fn bitonic_sort_rec<T: Ord>(arr: &mut [T], ascending: bool) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    bitonic_sort_rec(&mut arr[..mid], true);
    bitonic_sort_rec(&mut arr[mid..], false);
    bitonic_merge(arr, ascending);
}

fn bitonic_merge<T: Ord>(arr: &mut [T], ascending: bool) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    for i in 0..mid {
        if (arr[i] > arr[i + mid]) == ascending {
            arr.swap(i, i + mid);
        }
    }

    bitonic_merge(&mut arr[..mid], ascending);
    bitonic_merge(&mut arr[mid..], ascending);
}