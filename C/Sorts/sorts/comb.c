#include "../sortingUtils/sortingStructs.c"
#include <time.h>
struct sortStats combSort(int *array, int length) {
  struct sortStats s;
  clock_t start, end;
  double cpu_time_used;
  double shrink = 1.4;
  long long permutations = 0;
  long long comparasons = 0;
  int hold;
  int gap = length;
  int swapped = 1;
  start = clock();
  while (gap > 1 || swapped) {
    permutations += 1;
    gap = (int)gap / shrink;
    if (gap < 1) {
      gap = 1;
    }
    swapped = 0;
    for (int i = 0; i < length - gap; i++) {
      comparasons += 1;
      if (array[i] > array[i + gap]) {
        hold = array[i + gap];
        array[i + gap] = array[i];
        array[i] = hold;
        swapped = 1;
      }
    }
  }
  end = clock();
  s.comparasons = comparasons;
  s.permutations = permutations;
  s.timeUsed = ((double)(end - start)) / CLOCKS_PER_SEC;
  return s;
}