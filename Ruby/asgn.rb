#!/bin/ruby
class Game
	DIFFICULTY = 10
	def checkResults(user, mastermind)

	end


	def self.userInput(prompt: "")	#Takes an optional prompt which will be stored in prompt. In this case
					#I will use as a prompt
		puts prompt if prompt 	#Prints out the Prompt
		gets			#Reads User Input
	end				#Returns the result of gets
	def self.run
		guesses = 0
		until guesses == DIFFICULTY	
			userInput(prompt: "What is my name")
			guesses += 1
		end
	end
end
class StringValidator
	def stringLenCheck(instring)
		if instring.length != 5
			return 0
		end
	end
	def stringContentCheck(instring)
	
	end
	
end

Game.run
