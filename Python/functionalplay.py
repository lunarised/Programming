from functools import reduce

score = lambda o1,o2,o3: o1*15-(o2/2) if o3 else o1*10-o2 

items = ["ab", "cd", "ef", "gh"]
r1 = reduce(lambda r1, o1: o1+r1, items, "")
print(r1)

examScores = [65,72,41,99,32,84]
didNotPass = filter(lambda o1: o1<50, examScores)
print(list(didNotPass))

starWars = "A long time ago in a galaxy far far away"
longWords = map(lambda o1: o1.upper(), filter(lambda o1: len(o1)>=4, starWars.split()))
print(list(longWords))

print (score(8,2,True))
print (score(8,2,False))

