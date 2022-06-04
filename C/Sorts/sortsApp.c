#include "./sorts/betterBubble.c"
#include "./sorts/bogo.c"
#include "./sorts/bubble.c"
#include "./sorts/comb.c"
#include "./sorts/exchange.c"
#include "./sorts/gnome.c"
#include "./sorts/iCantBelieveItCanSort.c"
#include "sortingUtils/sortingChecking.c"
#include <stdio.h>
#include <string.h>
#include <time.h>
#define LENGTH 100000
#define SLOWLENGTH 10
#define ALLOWSLOW 0
int main() {
  srand(time(NULL));

  int array[LENGTH];
  for (int i = 0; i < LENGTH; i++) {
    array[i] = rand() % 100000;
  }

  /* Block of overly slow sorts */

  /* To do, Stooge Sort */
  if (ALLOWSLOW) {
    int bogoArray[SLOWLENGTH];
    memcpy(bogoArray, array, SLOWLENGTH * sizeof(int));
    struct sortStats bogoSortStats = bogoSort(bogoArray, SLOWLENGTH);
    printf(
        "Bogosort: Number of comparasons: %lld, Number of permutations: %lld, "
        "Time taken: %f\n",
        bogoSortStats.comparasons, bogoSortStats.permutations,
        bogoSortStats.timeUsed);
  }

  int bubbleArray[LENGTH];
  memcpy(bubbleArray, array, LENGTH * sizeof(int));
  struct sortStats bubbleSortStats = bubbleSort(bubbleArray, LENGTH);
  printf(
      "BubbleSort: Number of comparasons: %lld, Number of permutations: %lld, "
      "Time taken: %f, Correctly Sorted: %d\n",
      bubbleSortStats.comparasons, bubbleSortStats.permutations,
      bubbleSortStats.timeUsed, isSorted(bubbleArray, LENGTH));

  int betterBubbleArray[LENGTH];
  memcpy(betterBubbleArray, array, LENGTH * sizeof(int));
  struct sortStats betterBubbleSortStats =
      betterBubbleSort(betterBubbleArray, LENGTH);
  printf(
      "BetterBubbleSort: Number of comparasons: %lld, Number of permutations: "
      "%lld, "
      "Time taken: %f, Correctly Sorted: %d\n",
      betterBubbleSortStats.comparasons, betterBubbleSortStats.permutations,
      betterBubbleSortStats.timeUsed, isSorted(betterBubbleArray, LENGTH));

  int combArray[LENGTH];
  memcpy(combArray, array, LENGTH * sizeof(int));
  struct sortStats combSortStats = combSort(combArray, LENGTH);
  printf("CombSort: Number of comparasons: %lld, Number of permutations: %lld, "
         "Time taken: %f, Correctly Sorted: %d\n",
         combSortStats.comparasons, combSortStats.permutations,
         combSortStats.timeUsed, isSorted(combArray, LENGTH));

  int exchangeArray[LENGTH];
  memcpy(exchangeArray, array, LENGTH * sizeof(int));
  struct sortStats exchangeSortStats = exchangeSort(exchangeArray, LENGTH);
  printf("ExchangeSort: Number of comparasons: %lld, Number of permutations: "
         "%lld, "
         "Time taken: %f, Correctly Sorted: %d\n",
         exchangeSortStats.comparasons, exchangeSortStats.permutations,
         exchangeSortStats.timeUsed, isSorted(exchangeArray, LENGTH));

  int iCBICSArray[LENGTH];
  memcpy(iCBICSArray, array, LENGTH * sizeof(int));
  struct sortStats iCBICSSortStats = iCantBelieveItCanSort(iCBICSArray, LENGTH);
  printf("iCBICSSort: Number of comparasons: %lld, Number of permutations: "
         "%lld, "
         "Time taken: %f, Correctly Sorted: %d\n",
         iCBICSSortStats.comparasons, iCBICSSortStats.permutations,
         iCBICSSortStats.timeUsed, isSorted(iCBICSArray, LENGTH));

  int gnomeArray[LENGTH];
  memcpy(gnomeArray, array, LENGTH * sizeof(int));
  struct sortStats gnomeSortStats = gnomeSort(gnomeArray, LENGTH);
  printf("GnomeSort: Number of comparasons: %lld, Number of permutations: "
         "%lld, "
         "Time taken: %f, Correctly Sorted: %d\n",
         gnomeSortStats.comparasons, gnomeSortStats.permutations,
         gnomeSortStats.timeUsed, isSorted(gnomeArray, LENGTH));

  return EXIT_SUCCESS;
}
