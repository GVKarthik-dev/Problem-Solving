def optimalCandies(n,arr):
    total = [1]*n
    for i in range(n-1):
        if arr[i+1]> arr[i]:
            total[i+1] +=1
    for i in range(n-1,0,-1):
        if arr[i-1]>arr[i] and total[i-1]<=total[i]:
            total[i-1] += 1
    return(sum(total))
n = int(input())
arr = [int(input()) for _ in range(n)]
print(optimalCandies(n,arr))



#    2  4  2  6  1  7  8  9  2  1
#    1  1  1  1  1  1  1  1  1  1
#    1  2  1  2  1  2  2  2  1  1
#          1  2   1  2  2  2  2  1