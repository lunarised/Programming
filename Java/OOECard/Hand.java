class Hand{
    private char side;
    private Card[] cardsInHand;
    private int handSize;
    public Hand(char _side, int _handSize){
        side = _side;
        handSize = _handSize;
        cardsInHand = new Card [_handSize];  
        for (int i=0; i < cardsInHand.length - 2; i++){
            cardsInHand[i] = new Card('C');
        }
        if (Character.toUpperCase(_side) == 'E'){
            cardsInHand[cardsInHand.length - 1] = new Card('E');
        }
        else if (Character.toUpperCase(_side) == 'S'){
            cardsInHand[cardsInHand.length - 1] = new Card('S');
        }
    }
    public char getCardAt(int _index){
        if (_index >= 0 && _index < handSize){
         return cardsInHand[_index].getType();
        }
        else{
            return 'X';
        }
    }

}
