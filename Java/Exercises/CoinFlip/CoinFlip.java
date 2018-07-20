import java.util.Random;
public class CoinFlip{
	public static void main(String[] args){
		int flip;
		Random coin = new Random();
		flip = coin.nextInt(2);
		if (flip == 0){
			System.out.println("Tails");
		}else{
			System.out.println("Heads");
		}

	}
}
