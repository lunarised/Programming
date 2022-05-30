#include "bogo.c"
#include "bubble.c"
#include <stdio.h>
#include <string.h>
#include <time.h>
#define LENGTH 100000
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
    int bogoArray[LENGTH];
    memcpy(bogoArray, array, LENGTH);
    struct sortStats bogoSortStats = bogoSort(bogoArray, LENGTH);
    printf("Bogosort: Number of comparasons: %d, Number of permutations: %d, "
           "Time taken: %f\n",
           bogoSortStats.comparasons, bogoSortStats.permutations,
           bogoSortStats.timeUsed);
  }

  int bubbleArray[LENGTH];
  memcpy(bubbleArray, array, LENGTH);
  struct sortStats bubbleSortStats = bubbleSort(bubbleArray, LENGTH);
  printf("BubbleSort: Number of comparasons: %d, Number of permutations: %d, "
         "Time taken: %f\n",
         bubbleSortStats.comparasons, bubbleSortStats.permutations,
         bubbleSortStats.timeUsed);
  return EXIT_SUCCESS;
}
