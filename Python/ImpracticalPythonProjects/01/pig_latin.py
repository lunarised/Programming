"""Transforms a word into pig latin"""

while True:
    inputtedWord = input("Please input a word to be transformed into PigLatin: ")
    if ['a', 'e', 'i', 'o', 'u'].__contains__(inputtedWord[0].lower()):
        processedWord = inputtedWord + "way"
    else:
        processedWord = inputtedWord[1:] + inputtedWord[0] + 'ay'
    print(f"{inputtedWord} evaluates to {processedWord.lower()} in pig latin")

    try_again = input("Do you want to generate another name? Y/n    ")
    if try_again.lower() == "n":
        break
