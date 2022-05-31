int isSorted(int *array, int length) {
  for (int i = 0; i < length - 1; i++) {
    if (array[i] > array[i + 1]) {
      return 0;
    }
  }
  return 1;
}