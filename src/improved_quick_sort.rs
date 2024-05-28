use crate::selection_sort::selection_sort;

pub fn improved_quick_sort(arr: &mut [usize], low: usize, high: usize) -> &mut [usize] {
    let size = arr.len();
    if size < 20 {
        selection_sort(arr, size);
    };

    if low < high {
        let pi = partition(arr, low, high);

        improved_quick_sort(arr, low, pi - 1);

        improved_quick_sort(arr, pi + 1, high);
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