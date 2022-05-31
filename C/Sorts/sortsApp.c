#include "bogo.c"
#include "bubble.c"
#include "comb.c"
#include "sortingUtils/sortingChecking.c"
#include <stdio.h>
#include <string.h>
#include <time.h>
#define LENGTH 100000
#define SLOWLENGTH 10
#define ALLOWSLOW 0
int main() {
  int array[LENGTH];
  for (int i = 0; i < LENGTH; i++) {
    array[i] = rand() % 100000;
  }

  srand(time(NULL));

  /* Block of overly slow sorts */

  /* To do, Stooge Sort */
  if (ALLOWSLOW) {
    int bogoArray[SLOWLENGTH];
    memcpy(bogoArray, array, SLOWLENGTH);
    struct sortStats bogoSortStats = bogoSort(bogoArray, SLOWLENGTH);
    printf("Bogosort: Number of comparasons: %d, Number of permutations: %d, "
           "Time taken: %f\n",
           bogoSortStats.comparasons, bogoSortStats.permutations,
           bogoSortStats.timeUsed);
  }

  int bubbleArray[LENGTH];
  memcpy(bubbleArray, array, LENGTH);
  struct sortStats bubbleSortStats = bubbleSort(bubbleArray, LENGTH);
  printf("BubbleSort: Number of comparasons: %d, Number of permutations: %d, "
         "Time taken: %f, Correctly Sorted: %d\n",
         bubbleSortStats.comparasons, bubbleSortStats.permutations,
         bubbleSortStats.timeUsed, isSorted(bubbleArray, LENGTH));

  int combArray[LENGTH];
  memcpy(combArray, array, LENGTH);
  struct sortStats combSortStats = combSort(combArray, LENGTH);
  printf("BubbleSort: Number of comparasons: %d, Number of permutations: %d, "
         "Time taken: %f, Correctly Sorted: %d\n",
         combSortStats.comparasons, combSortStats.permutations,
         combSortStats.timeUsed, isSorted(combArray, LENGTH));

  return EXIT_SUCCESS;
}
