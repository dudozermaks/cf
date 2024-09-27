from random import choice
from string import ascii_lowercase 

print("1")
s = ''.join(choice(ascii_lowercase) for i in range(200000))

print(len(s))
print(s)
