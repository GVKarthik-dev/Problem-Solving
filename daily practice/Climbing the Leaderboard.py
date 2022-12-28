
rank = [100,100,50,40,40,20,10]

player = [5,25,50,120]
i,n=0,len(player)
sc = list(sorted(set(rank)))
ra = []

for i in player:
    while (n>i and i>=sc[i]):
        i+=1
    ra.append(n-i+1)

print(sorted(set(rank)))
