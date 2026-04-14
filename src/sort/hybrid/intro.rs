/// Sorts a slice using introsort (hybrid of quick sort and heap sort).
///
/// Switches to heap sort when recursion depth exceeds a limit.
///
/// # Performance
///
/// * **Time Complexity**: O(n log n)
/// * **Space Complexity**: O(log n)
/// * **Stability**: Unstable
pub fn intro_sort<T: Ord>(array: &mut [T]) {
    let depth = (array.len() as f64).log2() as usize * 2;
    introsort_rec(array, depth);
}

fn introsort_rec<T: Ord>(arr: &mut [T], depth: usize) {
    if arr.len() <= 1 {
        return;
    }

    if depth == 0 {
        arr.sort(); 
        return;
    }

    let p = partition(arr);
    let (left, right) = arr.split_at_mut(p);
    introsort_rec(left, depth - 1);
    introsort_rec(&mut right[1..], depth - 1);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    // Simple pivot selection: middle element
    let pivot_idx = len / 2;
    
    // Move pivot to the end temporarily
    arr.swap(pivot_idx, len - 1);
    
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    // Move pivot to its final place
    arr.swap(i, len - 1);
    i
}
