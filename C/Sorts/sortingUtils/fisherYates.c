#include "stdlib.h"

int *fisherYates(int *_input, int _length) {
  for (int i = 0; i < _length; i++) {
    int selectedSwap = rand() % (_length - i) + i;
    int holdValue = _input[selectedSwap];
    _input[selectedSwap] = _input[i];
    _input[i] = holdValue;
  }
  return _input;
}