import java.util.Scanner;
import java.util.Random;
public class Gamble{
    public static void main(String[] args){
        Random rand = new Random();
        Scanner sc = new Scanner(System.in);
        System.out.print("Input Player 1 Bet: ");
        int p1start = sc.nextInt();
        System.out.print("Input Player 2 Bet: ");
        int p2start = sc.nextInt();
        int p1 = p1start;
        int p2 = p2start;
        sc.close();
        while (true){
            p1 = rand.nextInt(p1);
            if (p1 == 0){
                System.out.println("Player 1 has been bankrupted!");
                System.out.println("Player 2 has won!");
                break;
            }
            System.out.println("Player 1 rolls a " + p1);
            p2 = rand.nextInt(p2);
            if (p2 == 0){
                System.out.println("Player 2 has been bankrupted!");
                System.out.println("Player 1 has won!");
                break;
            }
            System.out.println("Player 2 rolls a " + p2);
        }
    }
}