"use strict";
exports.__esModule = true;
function BubbleSort(nums) {
    var arrLength = nums.length;
    for (var i = 0; i < arrLength - 1; i++) {
        for (var j = 0; j < arrLength - 1 - i; j++) {
            if (nums[j] > nums[j + 1]) {
                /* Fancy XOR variable swap. Not overly efficient but its fun to do */
                nums[j] = nums[j] ^ nums[j + 1];
                nums[j + 1] = nums[j] ^ nums[j + 1];
                nums[j] = nums[j] ^ nums[j + 1];
            }
        }
    }
    return nums;
}
exports["default"] = BubbleSort;
