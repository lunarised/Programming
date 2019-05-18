def callBottle(inp):
	if inp > 2:
		print(inp, " bottles of beer on the wall!")
		print(inp, " bottles of beer!")
		print("You take one down, and pass it around")
		inp -= 1
		print(inp," bottles of beer on the wall")
		print("")
		callBottle(inp)
	elif inp == 2:
		print(inp, " bottles of beer on the wall!")
		print(inp, " bottles of beer!")
		print("You take one down, and pass it around")
		inp -= 1
		print("1 more bottle of beer on the wall")
		print("")
		callBottle(inp)
	elif inp == 1:
		print("1 more bottle of beer on the wall!")
		print("1 more bottle of beer!")
		print("Take it down, and guzzle it down")
		inp -= 1
		print("No more bottles of beer on the wall")
	else:
		print("Please give a positive number of bottles to drink")
sinp = input("How many bottles of beer are on the wall? : ")
inp = int(sinp)
callBottle(inp)

