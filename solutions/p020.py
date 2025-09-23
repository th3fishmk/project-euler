# Problem 20
# Factorial Digit Sum
import math


factorial = list(str(math.factorial(100)))
sum = 0
for i in factorial:
    sum += int(i)

print(sum)
