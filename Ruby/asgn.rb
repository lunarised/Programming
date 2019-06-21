#!/bin/ruby
module Game
	module_function
	DIFFICULTY = 10
	def checkResults(user, mastermind)

	end


	def userInput(prompt: "")	#Takes an optional prompt which will be stored in prompt. In this case
					#I will use as a prompt
		puts prompt if prompt 	#Prints out the Prompt
		gets.chomp			#Reads User Input
	end				#Returns the result of gets
	def run
		guesses = 0
		until guesses == DIFFICULTY	
			ui = userInput(prompt: "What is my name")
			if StringValidator.run(ui)
			guesses += 1
			
			else
				puts "invalid input"
			end
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
		dsucc = true
		dsucc = false if instring.each_char.find{|c| instring.count(c)>1}
	dsucc
	end
	def run(instring)
		succ = false		
		if stringLenCheck(instring) and stringContentCheck(instring) and duplicateChecker(instring)
			succ =  true
		end
	succ
	end
end

Game.run
