pub fn quick_sort(arr: &mut [usize], low: usize, high: usize) -> &mut [usize] {
    if low < high {
        let pi = partition(arr, low, high);

        quick_sort(arr, low, pi - 1);

        quick_sort(arr, pi + 1, high);
    }
    arr
}

fn partition(arr: &mut [usize], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}
