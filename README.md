# Empirical Analysis of Sorting Algorithm's Performances 

This report analyzes the performance of various sorting algorithms on arrays of different sizes, measuring the execution time in milliseconds (ms). The algorithms evaluated are Bubble Sort, Improved Bubble Sort, Quick Sort, Improved Quick Sort, Merge Sort, and Selection Sort. The sizes of the arrays tested are 10, 100, 1000, 10000, and 100000 elements.

## Execution Times (in milliseconds)

| Sort Algorithm / Size  | 10       | 100      | 1000     | 10000      | 100000      |
|------------------------|----------|----------|----------|------------|-------------|
| Bubble Sort            | 0.001583 | 0.048241 | 4.54735  | 456.891091 | 45876.59328 |
| Improved Bubble Sort   | 0.002184 | 0.001042 | 0.005124 | 0.049266   | 0.456641    |
| Quick Sort             | 0.002583 | 0.142599 | 16.71835 | 1336.210667| N/A         |
| Improved Quick Sort    | 0.0008   | 0.009383 | 0.241192 | 4.116691   | 71.877866   |
| Merge Sort             | 0.003633 | 0.050916 | 0.625849 | 7.415425   | 85.859133   |
| Selection Sort         | 0.001274 | 0.027049 | 2.220891 | 217.878183 | 22031.81403 |

## Analysis

### Bubble Sort
- **Performance**: Bubble Sort is the least efficient algorithm among those tested, especially noticeable with larger array sizes. It took approximately 45.88 seconds to sort an array of 100,000 elements.
- **Practical Usage**: Due to its inefficiency, Bubble Sort is not suitable for large datasets. It may be used for educational purposes or small datasets where simplicity is preferred over performance.

### Improved Bubble Sort
- **Performance**: This version of Bubble Sort is significantly faster than the standard Bubble Sort, managing to sort 100,000 elements in about 0.46 milliseconds.
- **Practical Usage**: Improved Bubble Sort is better than the standard version but still not ideal for very large datasets due to its inherent inefficiencies.

### Quick Sort
- **Performance**: Quick Sort is generally efficient, sorting 10,000 elements in about 1.34 seconds. However, it did not perform well on the largest dataset, potentially due to the lack of a median-of-three or similar pivot selection strategy.
- **Practical Usage**: Quick Sort is highly efficient for most datasets but may degrade in performance on certain inputs without improvements like randomized or median-of-three pivot selection.

### Improved Quick Sort
- **Performance**: Improved Quick Sort uses Hoare's partitioning scheme and is extremely efficient, sorting 100,000 elements in about 71.88 milliseconds.
- **Practical Usage**: Improved Quick Sort is highly recommended for large datasets due to its efficiency and speed.

### Merge Sort
- **Performance**: Merge Sort performs consistently well, sorting 100,000 elements in about 85.86 milliseconds.
- **Practical Usage**: Merge Sort is stable and efficient for large datasets. It is particularly useful when stability (preserving the order of equal elements) is required.

### Selection Sort
- **Performance**: Selection Sort is more efficient than Bubble Sort but still not suitable for very large datasets, taking approximately 22.03 seconds to sort 100,000 elements.
- **Practical Usage**: Selection Sort is not recommended for large datasets due to its inefficiency but can be used for small datasets or educational purposes.

## Practical Limits

### What is the biggest problem size you can run in two seconds?

Based on the data, the following conclusions can be made about the largest problem sizes that can be sorted within two seconds:

- **Bubble Sort**: Sorting an array of approximately 10,000 elements takes about 456.89 milliseconds, so it can handle around this size within two seconds.
- **Improved Bubble Sort**: This algorithm can sort far beyond 100,000 elements within two seconds.
- **Quick Sort**: Sorting 10,000 elements takes about 1.34 seconds, so it can handle slightly more than 10,000 elements within two seconds.
- **Improved Quick Sort**: Sorting 100,000 elements takes about 71.88 milliseconds, so it can handle well beyond this size within two seconds.
- **Merge Sort**: Sorting 100,000 elements takes about 85.86 milliseconds, so it can handle well beyond this size within two seconds.
- **Selection Sort**: Sorting an array of approximately 10,000 elements takes about 217.88 milliseconds, so it can handle around this size within two seconds.

## Verdict on Practical Usage

- **Bubble Sort and Selection Sort** are impractical for large datasets due to their inefficiency.
- **Improved Bubble Sort** offers better performance but still lags behind more advanced algorithms for large datasets.
- **Quick Sort** is generally efficient but requires optimization to handle all cases effectively.
- **Improved Quick Sort** (using Hoare's partitioning scheme) and **Merge Sort** are the best choices for large datasets, offering excellent performance and scalability.

In conclusion, for practical usage, **Improved Quick Sort** and **Merge Sort** are highly recommended for sorting large datasets efficiently. Other algorithms can be used for smaller datasets or specific scenarios where their properties are beneficial.
