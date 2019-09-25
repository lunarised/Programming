class Game{
    private static final int PLAYERS = 2;
    private static final int HANDSIZE = 5;

    private static Hand emperorsHand;
    private static Hand[] slaveHands;

    public static void main(String[] args){
        initState();
        System.out.println(slaveHands[0].getCardAt(4));
    }
    public static void initState(){
        emperorsHand = new Hand('E', HANDSIZE);
        slaveHands = new Hand[PLAYERS-1];
        for (int i=0; i<PLAYERS -1; i++){
            slaveHands[i] = new Hand('S', HANDSIZE);
        }
    }
}