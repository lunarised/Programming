const BubbleSort = require('./BubbleSort.js').default;

test('5,4,3,2,1', () => {
  expect(BubbleSort([5,4,3,2,1])).toEqual([1,2,3,4,5]);
});


test('-34, 1, 12, 55, 255, 255 ', () => {
  expect(BubbleSort([1,55,12,-34,255,255])).toEqual([ -34, 1, 12, 55, 255, 255 ]);
});


test('1', () => {
  expect(BubbleSort([1])).toEqual([1]);
});


test('65,4,33,21,-1', () => {
  expect(BubbleSort([65,4,33,21,-1])).toEqual([-1,4,21,33,65]);
});