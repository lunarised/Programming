#include "../sortingUtils/fisherYates.c"
#include "../sortingUtils/sortingStructs.c"
#include <time.h>

struct isSortedBogoStruct isSortedBogo(int *arr, int _length) {
  struct isSortedBogoStruct b;
  int last = arr[0];
  for (int i = 1; i < _length; i++) {
    if (arr[i] < last) {
      b.comparasons = i;
      b.isSorted = 0;
      return b;
    }
    last = arr[i];
  }
  b.comparasons = _length;
  b.isSorted = 1;

  return b;
}

struct sortStats bogoSort(int *array, int length) {
  clock_t start, end;
  double cpu_time_used;
  struct isSortedBogoStruct b;
  struct sortStats s;
  b.isSorted = b.comparasons = 0;
  long long permutations = 0;
  long long comparasons = 0;
  start = clock();
  while (!b.isSorted) {
    b = isSortedBogo(array, length);
    comparasons += b.comparasons;
    fisherYates(array, length);
    permutations += 1;
  }

  end = clock();
  s.comparasons = comparasons;
  s.permutations = permutations;
  s.timeUsed = ((double)(end - start)) / CLOCKS_PER_SEC;
  return s;
}
