#!/bin/python3
class Sentence:
	def __iter__(self):
		return self
	def __init__(self, input):
		self.word_list = input.split()
		self.current_index = 0
	
	def __next__(self):
		
		if (self.current_index >= len(self.word_list)):
			self.current_index = 0
			raise StopIteration
		current_word = self.word_list[self.current_index]
		self.current_index += 1
		return current_word
sentence = Sentence('Hello My Name is Annie')
for s in sentence:
	print(s)

