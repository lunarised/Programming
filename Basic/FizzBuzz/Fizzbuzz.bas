Dim i as Integer
for i = 1 to 100
	IF i Mod 3 = 0 THEN
		IF i Mod 5 = 0 THEN
			PRINT "FizzBuzz"
		ELSE
			PRINT "Fizz"
		END IF
	ELSEIF i Mod 5 = 0 THEN
		PRINT "Buzz"
	ELSE
		PRINT i
	ENDIF
	
NEXT i

END
