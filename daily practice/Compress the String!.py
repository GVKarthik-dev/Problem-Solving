import itertools as it

n = input()


groups = []
uniquekeys = []
data = sorted(n)
for k, g in it.groupby(n):
    groups.append(list(g))
    uniquekeys.append(k)

print(groups,uniquekeys,sep='\n')