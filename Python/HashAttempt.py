import random
def hashF(inp):
    n = 0
    i = 0
    for char in inp:
        ab = ord(char)
        if ab == 32:
            ab = 96
        n += (ab-96)*((i+1)*27)
        i += 1
    n %= 716559
    random.seed(n)
    print (random.randint(1, 1000000)) 
s1 = str(input("Input String 1: "))
s2 = str(input("Input String 2: "))
sC = s1+s2
hashF(sC)
