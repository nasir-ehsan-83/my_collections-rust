/// # Bucket Sort
///
/// A distribution-based sorting algorithm that partitions a slice of floats
/// in the range `[0, 1]` into a finite number of buckets. Each bucket is
/// then sorted individually before merging.
///
/// ## Complexity Analysis
///O(n²)
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | O(n + k) | Elements are uniformly distributed across buckets. |
/// | **Average Time** | O(n + k) | Linear time when input is uniformly distributed. |
/// | **Worst Time** | O(n²) | All elements cluster into a single bucket. |
/// | **Space Complexity** | O(n + k) | Extra memory needed for k buckets holding n items. |
///
/// ## Properties
///
/// * **Stable:** Depends on the underlying sorting algorithm used for buckets.
/// * **Not-adaptive:** The bucket allocation process remains identical for all distributions.
/// * **Not-In-place:** Requires additional heap allocations for the buckets.
///
/// ## Example
///
/// ```rust
/// use mycollections::sort::bucket_sort;
///
/// let mut numbers = [0.42, 0.32, 0.33, 0.52, 0.37, 0.47, 0.51];
/// bucket_sort(&mut numbers);
///
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


