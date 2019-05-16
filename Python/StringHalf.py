def wordHalf():
        inp = input("Your input here:  ")
        i = 1
        outstr = ""
        for c in inp:
                if i%2 == 0 :        
                        outstr += c
                i+=1
        print (outstr)
