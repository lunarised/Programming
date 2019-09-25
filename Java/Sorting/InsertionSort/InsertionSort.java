import java.util.Scanner;
public class InsertionSort{

	public static void main(String[] args){
	System.out.println("Hello World");
	int[] unsorted = new int[]{22,6969,420,2,1};
	int hold;
	for(int i = 1; i<unsorted.length; i++){
		hold = unsorted[i];
		for(int j = i; j>0; j--) {
			if (hold<unsorted[j-1]){
				hold = unsorted[j-1];
				unsorted[j-1] = unsorted[j];
				unsorted[j] = hold;
				hold = unsorted[j-1];
			}
		}
	}
	for (int val: unsorted){
		System.out.print(val + " ");
	}
	System.out.print("\n");
	}

	}

