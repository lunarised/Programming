public class A99Bottles{
	public static void main(String[] args){
		bottleCount(10);		
	}
	public static void bottleCount(int _bottle){
		if (_bottle > 1){
			System.out.println(_bottle + " bottles of beer on the wall!");
			System.out.println(_bottle + " bottles of beer!");
			System.out.println("You take one down, and pass it around!");
			_bottle--;
			System.out.println(_bottle + " bottles of beer on the wall!");
			System.out.println("");
			bottleCount(_bottle);
		}
		else if (_bottle == 1){
			System.out.println(_bottle + " more bottle of beer on the wall!");
			System.out.println(_bottle + " more bottle of beer!");
			System.out.println("Take it down, and pass it around!");
			_bottle--;
			System.out.println("No more bottles of beer on the wall!");
			return;
		}
		if (_bottle <= 0){
			return;
		}
	}
}
