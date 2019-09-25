class SelectionSort{
    public static void main(String[] args){
        int[] unsorted = new int[]{30,20,40,80,220,6969};
	    int hold;
        for (int i = 0; i<unsorted.length; i++){
            int min = unsorted[i];
            int minLoc = i;

            for (int j = i; j<unsorted.length; j++){
                if (min > unsorted[j]){
                    min = unsorted[j];
                    minLoc = j;
                }
            }
            hold = unsorted[i];
            unsorted[i] = min;
            unsorted[minLoc] = hold;            
        }
        for (int val: unsorted){
		System.out.print(val + " ");
	}
	System.out.print("\n");
    }
}