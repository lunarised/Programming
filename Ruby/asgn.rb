#!/bin/ruby
module Game
	module_function
	DIFFICULTY = 1
	
	def checkResults(user, mastermind)

	end


	def userInput(prompt: "")	#Takes an optional prompt which will be stored in prompt. In this case
					#I will use as a prompt
		puts prompt if prompt 	#Prints out the Prompt
		gets.chomp			#Reads User Input
	end				#Returns the result of gets
	def run
		correct = true
		guesses = 0
		until guesses == DIFFICULTY	
			ui = userInput(prompt: "What is my name")
			if StringValidator.run(ui)
			guesses += 1
			puts "Valid Guess"
			else
			puts "Bad Guess"
			end
		end
		if correct
		puts "Congratulations! you win mastermind"
		else
		puts "Oof Ya Dingus! You didn't get it!"
		end
	end
end
module StringValidator
	module_function
	def stringLenCheck(instring)
		instring.length==5
	end
	def stringContentCheck(instring)
		instring =~ /^[A-Za-z]+$/	
	end
	def duplicateChecker(instring)
		instring.chars == instring.chars.uniq
	end
	def run(_instring)
		instring = _instring.downcase
		stringLenCheck(instring) && stringContentCheck(instring) && duplicateChecker(instring)
	end
end
module WordGetter
	def run
	
	end
end
Game.run
