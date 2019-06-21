#!/bin/ruby
module Game				#I am using modules, as no objects will be created i this game
	module_function			#Allows the module to have its members called outside of the module
	DIFFICULTY = 10			#The Number of Guesses the player gets. Free to modify
	def userInput(prompt: "")	#Takes an optional prompt which will be stored in prompt. In this case
					#I will use as a prompt
		puts prompt if prompt 	#Prints out the Prompt
		gets.chomp		#Reads User Input
	end				#Returns the result of gets
	def compare(instring, _masterKey)	#The method that compares the guess with the masterKey
		result = "XXXXX"		#assume all results are wrong
		(0..4).each do |x|			#for each of the letters in the strings (5) 0 and 4 are inclusive
			result[x]= "U" unless _masterKey.upcase.index(instring[x].upcase).nil? 	#set the result to a U (Correct Letter, Wrong Spot) if the letter in position X exists in both strings
			result[x] = "O" if instring[x].upcase == _masterKey[x].upcase	#set the result to a O if its in the correct place with the correct letter
		end			#There are probably more efficient ways to do this, But im not using them
		cor = result == "OOOOO"	#Set it as a correct guess, if the result string is all Correct Letter, Correct Spot
		puts result unless cor	#Prints the result if you didn't get a correct guess
		cor			#returns whether the result was correct or not
	end				
	

	def run				#The method that holds the gameplay
		correct = false		#Set the state to be not solved 
		guesses = 0		#Sets the initial value for guesses. Refer to DIFFICULTY if you think about changing this
		masterKey = WordGetter.run.chomp	#Gets a word, and removes the trailing backslash n
		until StringValidator.run(masterKey)  #Until the word we got will validate with our string Validator
			masterKey = WordGetter.run.chomp	#Get a new word, and remove the trailing backslash
		end						#When we find a suitable word to use
		until guesses == DIFFICULTY || correct		#While our game hasnt been won, or we havent run out of guesses
			ui = userInput(prompt: "Guess #{guesses}: ")	#Prompt the player for an input
			if StringValidator.run(ui)		#if the input validates through our String Validator
			guesses += 1				#use up a guess
			correct = compare(ui, masterKey)	#compare the strings, and set correct to the result of that function
			else					#if the guess doesnt validate
			puts "Input does not validate. Check it has 5 letters, and no numbers or duplicate letters"	#give the user some feed back, don't use a guess
			end
		end
		if correct					#if the correct guess has happened
			puts "Congratulations! You win mastermind" 	#Congratulate them for the win
			puts "It took you #{guesses} guesses!"		#Tell them how many guesses it took them
		else						#if they ran out of guesses
			puts "Oof Ya Dingus! You didn't get it!"	#show the loss state
		end
		ui = userInput(prompt: "Do you want to play again? [Y]/n") #ask them if they want to play again. Defaults to yes, unless they type "n"
		return false if ui.downcase == "n"		#return false if they type N or n
		true						#else, return true, Keeping the do loop working
	end
	
end
module StringValidator		#A module that validates a string by checking length, duplicates and alpha
	module_function		#Allows function members to be called outside the function
	def stringLenCheck(instring)	#Checks if the length is correct
		instring.length==5	#returns true if length = 5, else it will return false
	end
	def stringContentCheck(instring)	#Checks if the input string is an english character
		instring =~ /^[A-Za-z]+$/	#Using this regex	
	end
	def duplicateChecker(instring)	#Checks if there is a duplicate in the string
		instring.chars == instring.chars.uniq	#if the characters in the string are the same as the characters in the string with duplicates removed
	end
	def run(_instring) #The function called to execute the checks
		instring = _instring.downcase #change the string to lowercase
		stringLenCheck(instring) && stringContentCheck(instring) && duplicateChecker(instring) #returns true if all the string checks pass
							#Use && instead of and, as and has a different use, even if it could work here
	end
end
module WordGetter 	#A module that gets a random word from a set file
	module_function	#Allows the functions members to be called outside the function
	def run		#default method to call
		File.readlines("mastermindWordList.txt").sample	#Reads a random line (using sample) from the specified file (Low memory usage, High Disk access)
	end
end
puts "Welcome to Mastermind! Written by lunarised (James McKenzie)"	#Prints a help page
puts "The objective is to guess the word generated in a set number of gos!"
puts "X = Incorrect Guess, O = Correct Place Correct Guess, U = Correct Guess, Wrong Place"
puts "Words are 5 alpha characters long, with no duplicate letters!"
puts "Have Fun!"
loop do
	break unless Game.run #While the player hasnt chosen to exit the game (See above)
end

