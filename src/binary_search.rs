pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        let guess = arr[mid];

        if guess == target {
            return Some(mid);
        } else if guess < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}
