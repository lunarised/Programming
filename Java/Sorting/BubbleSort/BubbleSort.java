class BubbleSort{
    public static void main(String[] args){
        int[] unsorted = new int[]{30,20,40,80,220,6969};
	    int hold;
        for (int i = 0; i<unsorted.length-1; i++){
            for (int j = i; j<unsorted.length-1; j++){
                if (unsorted[j]>unsorted[j+1]){
                    hold = unsorted[j];
                    unsorted[j]= unsorted[j+1];
                    unsorted[j+1] = hold;
                }
                
            }
         
        }
        for (int val: unsorted){
		System.out.print(val + " ");
	}
	System.out.print("\n");
    }
}