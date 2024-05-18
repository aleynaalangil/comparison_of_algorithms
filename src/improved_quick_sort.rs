pub fn improved_quick_sort(arr: &mut [usize], low: usize, high: usize) -> &mut [usize] {
    if low < high {
        let pi = hoare_partition(arr, low, high);

        improved_quick_sort(arr, low, pi - 1);

        improved_quick_sort(arr, pi + 1, high);
    }
    arr
}

fn hoare_partition(arr: &mut [usize], low: usize, high: usize) -> usize {
    let pivot = arr[low];
    let mut i = low;
    let mut j = high + 1;

    loop {
        i += 1;
        while i <= high && arr[i] < pivot {
            i += 1;
        }

        j -= 1;
        while arr[j] > pivot {
            j -= 1;
        }

        if i >= j {
            break;
        }

        arr.swap(i, j);
    }

    arr.swap(low, j);
    j
}
