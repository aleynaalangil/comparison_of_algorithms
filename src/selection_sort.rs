use std::mem::swap;

pub fn selection_sort(arr: &mut [usize], size:usize) -> &mut [usize]{
    for i in 0..size{
        let mut min_idx = i;
        for j in i+1..size{
            if arr[j]<arr[min_idx]{
                min_idx = j;
            }
        }
        arr.swap(i, min_idx)
    }
    arr
}
