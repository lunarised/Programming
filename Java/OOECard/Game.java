import java.util.Scanner;
class Game{
    private static final int PLAYERS = 2;
    private static final int HANDSIZE = 5;

    private static Hand emperorsHand;
    private static Hand[] slaveHands;
    private static char playerSide;
    
    private static Scanner sc;

    public static void main(String[] args){
        initState();
        System.out.println(playerSide);        
        System.out.println(slaveHands[0].getCardAt(4));
    }
    public static void initState(){
        emperorsHand = new Hand('E', HANDSIZE);
        slaveHands = new Hand[PLAYERS-1];
        for (int i=0; i<PLAYERS -1; i++){
            slaveHands[i] = new Hand('S', HANDSIZE);  
        }
        sc = new Scanner(System.in);
        boolean validInput = false;
        while(!validInput){
        System.out.print("What Side do you wish to start on? [S/E]: ");
        String inp = sc.nextLine();
        char pToProcess = Character.toUpperCase(inp.charAt(0));
        switch(pToProcess){
            case 'E':
                validInput = true;
                break;
            case 'S':
                validInput = true;
                break;
            default :
                validInput = false;
                System.out.println("Invalid input!");
        }
        playerSide = pToProcess;
        }

    }
}