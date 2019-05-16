def randBuilder(nRand):
        d = [0]*nRand
        i = 0;
        while i<nRand:
                d[i] = random.randint(0, 99)
                i +=1
        print("Max Element: ", max(d))
        print("Min Element: ", min(d))

