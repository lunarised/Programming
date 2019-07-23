#!/bin/python3
inp = input("What is the size of the matrix?")
iinp = int (inp)
if iinp<3 or iinp%2 == 0:
    print ("Oof")
    exit()
#Initial State Setup
outString = ""
currentNum = (int((iinp**2)/2)+1)
outString += str(currentNum) + " "
currentNum += 1
outString += str(currentNum) + " "
radius = 1
while radius < iinp :
   ##Go down##
    for i in range(radius):
        currentNum += iinp
        outString += str(currentNum) + " "
    radius += 1
    ##Go Left##
    for i in range(radius):
        currentNum -= 1
        outString += str(currentNum) + " "
    ##Go up##
    for i in range(radius):
        currentNum -= iinp
        outString += str(currentNum) + " "
    radius += 1
    ##Go right##
    for i in range(radius):
        if currentNum != iinp:
            currentNum += 1
            outString += str(currentNum) + " "
        else:
            break
print (outString)
