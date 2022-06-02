#include "../sortingUtils/sortingStructs.c"
#include <time.h>
struct sortStats exchangeSort(int *array, int length) {
  clock_t start, end;
  double cpu_time_used;
  struct sortStats s;
  long long permutations = 0;
  long long comparasons = 0;
  int hold;
  start = clock();
  for (int i = 0; i < length - 1; i++) {
    permutations += 1;
    for (int j = i + 1; j < length; j++) {
      comparasons += 1;
      if (array[i] > array[j]) {
        hold = array[j];
        array[j] = array[i];
        array[i] = hold;
      }
    }
  }
  end = clock();
  s.comparasons = comparasons;
  s.permutations = permutations;
  s.timeUsed = ((double)(end - start)) / CLOCKS_PER_SEC;
  return s;
}