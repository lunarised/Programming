const SelectionSort = require('./SelectionSort.js').default;

test('5,4,3,2,1', () => {
  expect(SelectionSort([5,4,3,2,1])).toEqual([1,2,3,4,5]);
});


test('-34, 1, 12, 55, 255, 255 ', () => {
  expect(SelectionSort([1,55,12,-34,255,255])).toEqual([ -34, 1, 12, 55, 255, 255 ]);
});


test('1', () => {
  expect(SelectionSort([1])).toEqual([1]);
});


test('65,4,33,21,-1', () => {
  expect(SelectionSort([65,4,33,21,-1])).toEqual([-1,4,21,33,65]);
});