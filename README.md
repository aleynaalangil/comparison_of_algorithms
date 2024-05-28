# Empirical Study

This report analyzes the performance of various sorting algorithms on arrays of different sizes, measuring the execution time in milliseconds (ms). The algorithms evaluated are Bubble Sort, Improved Bubble Sort, Quick Sort, Improved Quick Sort, Merge Sort, and Selection Sort. The sizes of the arrays tested are 10, 100, 1000, 10000, and 100000 elements.

## Execution Times (in milliseconds)

| Sort Algorithm/Size | 10       | 100      | 1000      | 10000       | 100000      |
|---------------------|----------|----------|-----------|-------------|-------------|
| Bubble Sort         | 0.001583 | 0.048241 | 4.54735   | 456.891091  | 45876.59328 |
| Improved Bubble Sort| 0.002184 | 0.001042 | 0.005124  | 0.049266    | 0.456641    |
| Quick Sort          | 0.002583 | 0.142599 | 16.71835  | 1336.210667 | N/A         |
| Improved Quick Sort | 0.001358 | 0.144258 | 16.780766 | 1390.252866 | N/A         |
| Merge Sort          | 0.003633 | 0.050916 | 0.625849  | 7.415425    | 85.859133   |
| Selection Sort      | 0.001274 | 0.027049 | 2.220891  | 217.878183  | 22031.81403 |

## Analysis of Data

### Largest Problem Size Within Two Seconds

#### Observations:

- **Bubble Sort**: Takes 4.54735 seconds for 1000 elements, 456.891091 seconds for 10,000 elements, and 45876.59328 seconds for 100,000 elements. It can't handle more than 1000 elements within 2 seconds.
- **Improved Bubble Sort**: Takes 0.049266 seconds for 10,000 elements and 0.456641 seconds for 100,000 elements. It can handle 100,000 elements within 2 seconds.
- **Quick Sort**: Takes 16.71835 seconds for 1000 elements, 1336.210667 seconds for 10,000 elements. It can't handle more than 1000 elements within 2 seconds.
- **Improved Quick Sort**: Takes 16.780766 seconds for 1000 elements and 1390.252866 seconds for 10,000 elements. It can't handle more than 1000 elements within 2 seconds.
- **Merge Sort**: Takes 7.415425 seconds for 10,000 elements and 85.859133 seconds for 100,000 elements. It can handle 10,000 elements within 2 seconds.
- **Selection Sort**: Takes 2.220891 seconds for 1000 elements and 217.878183 seconds for 10,000 elements. It can handle 1000 elements within 2 seconds.

## Practical Usage Report

### Bubble Sort
- **Time Complexity**: O(n²)
- **Performance**: Highly inefficient for large datasets. From the data, it cannot sort more than 1000 elements in under 2 seconds.
- **Practical Use**: Limited to very small datasets.

### Improved Bubble Sort
- **Time Complexity**: O(n²) in the worst case, but can be better on nearly sorted data.
- **Performance**: More efficient than the basic Bubble Sort. It can handle up to 100,000 elements within 2 seconds based on the data.
- **Practical Use**: Suitable for small to moderately sized datasets, especially when the data is nearly sorted.

### Quick Sort
- **Time Complexity**: O(n log n) on average, O(n²) in the worst case.
- **Performance**: Efficient for large datasets in practice but fails to handle more than 1000 elements within 2 seconds in this specific instance due to potential worst-case behavior.
- **Practical Use**: Generally good for large datasets but not ideal for highly repetitive or sorted data without modifications.

### Improved Quick Sort
- **Time Complexity**: O(n log n) on average, with improvements for small datasets.
- **Performance**: Similar to Quick Sort, limited by worst-case scenarios. Unable to handle more than 1000 elements in under 2 seconds.
- **Practical Use**: Suitable for large datasets with a safeguard for small arrays, making it more versatile than the standard Quick Sort.

### Merge Sort
- **Time Complexity**: O(n log n)
- **Performance**: Consistent and efficient for large datasets. Can handle up to 10,000 elements within 2 seconds.
- **Practical Use**: Reliable for large datasets due to its stable performance and predictable time complexity.

### Selection Sort
- **Time Complexity**: O(n²)
- **Performance**: Inefficient for large datasets. Can handle only up to 1000 elements within 2 seconds.
- **Practical Use**: Similar to Bubble Sort, limited to small datasets.

## Conclusion

For practical usage, Merge Sort and Improved Bubble Sort are the most efficient for handling larger datasets within the given 2-second threshold. Merge Sort is particularly reliable for consistently large datasets, while Improved Bubble Sort is more efficient for smaller to moderately sized or nearly sorted datasets. Quick Sort and its improved version have potential but may face limitations in worst-case scenarios. Bubble Sort and Selection Sort are not suitable for large datasets.

