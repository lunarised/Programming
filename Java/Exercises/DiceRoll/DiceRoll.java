/**
 * Dice Roll Simulator
 * @author James McKenzie
 *
 *23/3/2018
 *
 * Generates a random number from 1 to 6
 */



import java.util.Random;
public class DiceRoll{

	public static void main(String[] args){
		int roll;
		Random rand = new Random();
		roll = rand.nextInt(6);
		roll++;
		System.out.println(roll);



	}


}
