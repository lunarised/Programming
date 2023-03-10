#include "../sortingUtils/sortingStructs.c"
#include <time.h>
struct sortStats bubbleSort(int *array, int length) {
  clock_t start, end;
  double cpu_time_used;
  struct sortStats s;
  long long permutations = 0;
  long long comparasons = 0;
  int hold;
  start = clock();
  for (int i = 0; i < length - 1; i++) {
    permutations += 1;
    for (int j = 0; j < length - 1 - i; j++) {
      comparasons += 1;
      if (array[j] > array[j + 1]) {
        hold = array[j + 1];
        array[j + 1] = array[j];
        array[j] = hold;
      }
    }
  }
  end = clock();
  s.comparasons = comparasons;
  s.permutations = permutations;
  s.timeUsed = ((double)(end - start)) / CLOCKS_PER_SEC;
  return s;
}