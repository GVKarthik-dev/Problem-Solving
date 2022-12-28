sticks = [1,2,3,4,5,10]

result = []

for i in range(-1,len(sticks)-2):

    result.append(sticks[i]+sticks[i+1]+sticks[i+2])

print(result)


