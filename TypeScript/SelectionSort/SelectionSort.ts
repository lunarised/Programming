export default function SelectionSort(nums :number[]) :number[]{
    for (let i = 0 ; i< nums.length; i++){ //finish with a sorted element
        let min = nums[i];
        let minindex = i;
        for (let j  = i; j<nums.length; j++ ){
            if (nums[j] < min){
                min = nums[j];
                minindex = j;
            }
        }
            nums[minindex] = nums[i]; //nums[1] = 3
            nums[i] = min;             //nums[0] = 1
    }
    return nums;
}
console.log(SelectionSort([3,4,12,-21,222,1,2]));