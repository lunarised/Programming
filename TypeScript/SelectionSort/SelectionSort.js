"use strict";
exports.__esModule = true;
function SelectionSort(nums) {
    for (var i = 0; i < nums.length; i++) { //finish with a sorted element
        var min = nums[i];
        var minindex = i;
        for (var j = i; j < nums.length; j++) {
            if (nums[j] < min) {
                min = nums[j];
                minindex = j;
            }
        }
        nums[minindex] = nums[i]; //nums[1] = 3
        nums[i] = min; //nums[0] = 1
    }
    return nums;
}
exports["default"] = SelectionSort;
console.log(SelectionSort([3, 4, 12, -21, 222, 1, 2]));
