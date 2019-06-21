#!/bin/ruby
class Game
	DIFFICULTY = 10
	def checkResults(user, mastermind)

	end


	def userInput(prompt: "")	#Takes an optional prompt which will be stored in prompt. In this case
					#I will use as a prompt
		puts prompt if prompt 	#Prints out the Prompt
		gets			#Reads User Input
	end				#Returns the result of gets
	guesses = 0
	until guesses == DIFFICULTY
	
		userInput(prompt: "What is my name")
		guesses++
	end

end
class StringCheck


end

Game
