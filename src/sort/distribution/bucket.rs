
/// Sorts a slice of floats in the range [0, 1) using the bucket sort algorithm.
///
/// Bucket sort works by partitioning an array into a number of buckets. Each bucket 
/// is then sorted individually, either using a different sorting algorithm, 
/// or by recursively applying the bucket sorting algorithm.
///
/// # Performance
///
/// * **Time Complexity**: $O(n + k)$ (average case), $O(n^2)$ (worst case).
/// * **Space Complexity**: $O(n + k)$.
/// * **Stability**: Stable if the underlying sort for buckets is stable.
///
/// # Examples
///
/// ```
/// use my_collections::sort::distribution::bucket_sort;
///
/// let mut numbers = [0.42, 0.32, 0.33, 0.52, 0.37, 0.47, 0.51];
/// bucket_sort(&mut numbers);
/// assert_eq!(numbers, [0.32, 0.33, 0.37, 0.42, 0.47, 0.51, 0.52]);
/// ```
pub fn bucket_sort(array: &mut [f64]) {
    let n = array.len();
    if n < 2 {
        return;
    }

    let mut buckets: Vec<Vec<f64>> = vec![vec![]; n];

    // Put array elements in different buckets
    for &num in array.iter() {
        let bucket_idx = (num * n as f64) as usize;
        if bucket_idx < n {
            buckets[bucket_idx].push(num);
        } else {
            buckets[n - 1].push(num);
        }
    }

    // Sort individual buckets and concatenate
    let mut idx = 0;
    for bucket in buckets.iter_mut() {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap()); // Using standard library sort
        for &num in bucket.iter() {
            array[idx] = num;
            idx += 1;
        }
    }
}


