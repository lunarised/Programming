#!/bin/python3
sI = input("What max number do you want to go to? :")
maxN = int(sI)
i = 1
for i in range(1, maxN):
    if (i%3 == 0):
        if (i%5 == 0):
            print ("FizzBuzz")
        else:
            print("Fizz")
    else:
        if (i%5 == 0):
            print ("Buzz")
        else:
            print (i)

