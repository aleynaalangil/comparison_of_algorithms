pub fn bubble_sort(arr: &mut [usize]) -> &mut [usize] {
    let len = arr.len();
    for _ in 0..len {
        for i in 1..len {
            if arr[i] < arr[i - 1] {
                arr.swap(i, i - 1);
            }
        }
    }
    arr
}
