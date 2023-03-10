from collections import defaultdict

def line_gen(n):
    output = ""
    for _ in range(n):
        output = output + "="
    output = output + "|"
    return output

inputString = input("Please provide a string to be barcharted: ").lower()
d = defaultdict()

ALPHABET = 'abcdefghijklmnopqrstuvwxyz'

for word in inputString:
    for letter in word:
        if letter in d:
            d[letter] = d[letter] + 1
        else:
            d[letter] = 1

for letter in ALPHABET:
    if letter in d:
        print(F"{letter}|{line_gen(d[letter])}")
