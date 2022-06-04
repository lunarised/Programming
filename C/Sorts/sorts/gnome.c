#include "../sortingUtils/sortingStructs.c"
#include <time.h>
struct sortStats gnomeSort(int *array, int length) {
  clock_t start, end;
  double cpu_time_used;
  struct sortStats s;
  long long permutations = 0;
  long long comparasons = 0;
  int hold;
  int i = 0;
  start = clock();
  while (i < length) {
    ++comparasons;
    if (i == 0 || array[i] >= array[i - 1]) {
      ++i;

    } else {
      hold = array[i - 1];
      array[i - 1] = array[i];
      array[i] = hold;
      --i;
    }
  }

  end = clock();
  s.comparasons = comparasons;
  s.permutations = permutations;
  s.timeUsed = ((double)(end - start)) / CLOCKS_PER_SEC;
  return s;
}