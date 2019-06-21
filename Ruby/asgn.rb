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
		instring.length==5
	end
	def stringContentCheck(instring)
		succ = true
		characters = instring.split('')
		chars.each{ |c|
			unless letter?(c)	
				succ = false;
			end
		}
	succ	
	end
	def letter?(letter)
  		letter =~ /[A-Za-z]/
	end
end

Game.run
