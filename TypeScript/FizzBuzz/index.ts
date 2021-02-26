let MAX_NUMBER = 100;

for (let i = 0; i < MAX_NUMBER; i++){
    let outputStr = "";
    if (i%3 ==0){
        outputStr += "Fizz"
    }
    if (i%5 ==0){
        outputStr += "Buzz"
    }
    if (outputStr == ""){
        outputStr += i;
    }
    console.log(outputStr);
}