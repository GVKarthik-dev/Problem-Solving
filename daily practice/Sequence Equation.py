a = [4,3,5,1,2]

for i in range(1,1+len(a)):
    print(a.index(a.index(i)+1)+1)