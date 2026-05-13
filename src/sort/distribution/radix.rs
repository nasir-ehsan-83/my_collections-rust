/// # Radix Sort (LSD)
///
/// A non-comparative sorting algorithm that sorts non-negative integers by processing
/// individual digits. This implementation uses the Least Significant Digit (LSD) approach,
/// sorting from the rightmost digit to the leftmost digit.
///
/// ## Complexity Analysis
///
/// | Metric | Complexity | Note |
/// | :--- | :--- | :--- |
/// | **Best Time** | $O(d \cdot (n + k))$ | Performance is consistent across all data distributions. |
/// | **Average Time** | $O(d \cdot (n + k))$ | Depends heavily on the number of digits $d$ and the radix base $k$. |
/// | **Worst Time** | $O(d \cdot (n + k))$ | No worst-case degradation; execution path remains identical. |
/// | **Space Complexity** | $O(n + k)$ | Requires extra memory for bucket allocation and temporary storage. |
///
/// ## Properties
///
/// * **Stability:** ✅ **Stable**. Preserves the relative order of equal elements across digit passes.
/// * **Adaptive:** No. The algorithm performs a fixed number of passes based on the maximum key size.
/// * **In-place:** ❌ **No**. Requires helper arrays to distribute elements during each pass.
///
/// ## Example
///
/// ```rust
/// use my_collections::sort::radix_sort;
///
/// let mut numbers = [170, 45, 75, 90, 802, 24, 2, 66];
/// radix_sort(&mut numbers);
///
/// assert_eq!(numbers, [2, 24, 45, 66, 75, 90, 170, 802]);
/// ```
pub fn radix_sort(array: &mut [usize]) {
    if array.is_empty() {
        return;
    }

    let max = *array.iter().max().unwrap();
    let mut exp = 1;

    while max / exp > 0 {
        counting_sort_by_digit(array, exp);
        exp *= 10;
    }
}

fn counting_sort_by_digit(array: &mut [usize], exp: usize) {
    let mut output = vec![0; array.len()];
    let mut count = vec![0; 10];

    for &num in array.iter() {
        count[(num / exp) % 10] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for &num in array.iter().rev() {
        let digit = (num / exp) % 10;
        output[count[digit] - 1] = num;
        count[digit] -= 1;
    }

    array.copy_from_slice(&output);
}