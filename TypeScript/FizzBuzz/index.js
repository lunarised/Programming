var MAX_NUMBER = 100;
for (var i = 0; i < MAX_NUMBER; i++) {
    var outputStr = "";
    if (i % 3 == 0) {
        outputStr += "Fizz";
    }
    if (i % 5 == 0) {
        outputStr += "Buzz";
    }
    if (outputStr == "") {
        outputStr += i;
    }
    console.log(outputStr);
}
