
def nonDivisibleSubset(k, s):
    result0,result1 = set(),set()
    for i in range(len(s)):
        for j in range(i):
            if (s[i]+s[j])%k != 0:
                result0.add(s[i])
                result1.add(s[j])
    return result0,result1



k,s = 4,[19,10,12,10,24,25,22]

# k,s = 3,[1,7,2,4]

pr = nonDivisibleSubset(k,s)

print(pr)