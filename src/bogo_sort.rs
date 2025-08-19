use rand::seq::SliceRandom;
use rand::thread_rng;

// Bogo sort - aka. gambling sort / random
pub fn bogo_sort(arr: &mut [i32]) {
    fn is_sorted(arr: &[i32]) -> bool {
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                return false;
            }
        }
        true
    }

    while !is_sorted(arr) {
        println!("Shuffling: {:?}", arr);
        arr.shuffle(&mut thread_rng());
    }
}
