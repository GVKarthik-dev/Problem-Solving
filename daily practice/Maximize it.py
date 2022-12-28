# import numpy as np
#
# k,m = map(int,input().split())
# a,temp = [],0
# for i in range(k):
#     li = list(map(int,input().split()))
#     temp = ((np.array(li))**2)%m
#     a.append(max(temp))
#
# print(sum(a)%m)



from itertools import product

n,m = map(int,input().split())
li1 = []
for i in range(n):
    l= list(map(int,input().split()))
    li1.append(l)

result = list(map(lambda x:sum(i*i for i in x)%m,product(*li1)))

print(max(result))