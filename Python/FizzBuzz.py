#!/bin/python3
sI = input("What max number do you want to go to? :") #Asks for what input the user wants to use as a max number
                                                      #and stores it in a variable called sI
maxN = int(sI)                                        #Makes a new variable called MaxN which converts the input given by 
                                                      #user into a integer
i = 1                                                 #defines an iterator to start at 1
for i in range(1, maxN):                              #iterates through every value between 1 and maxN (roof exclusive)
    if (i%3 == 0):                                    #If the value of the iterator modulo 3 is equal to 0 (i is a multiple of 3)
        if (i%5 == 0):                                #If the value of the iterator modulo 5 is equal to 0 (i is a multiple of 5)  
            print ("FizzBuzz")                        #Print out Fizzbuzz because its a multiple of 3 and 5
        else:                                         #If its not a multiple of 5
            print("Fizz")                             #Print Fizz because its a multiple of 3 and not 5
    else:                                             #If its not a multiple  of 3
        if (i%5 == 0):                                #If its a multiple of 5
            print ("Buzz")                            #print Buzz because its a multiple of 5 and not 3
        else:                                         #If its neither a multiple of 3 or 5
            print (i)                                 #Just print the number stored in the iterator

