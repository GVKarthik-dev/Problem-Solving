n= 5
arr = [1,-3,71,68,17]
arr.sort()
result = []
for i in range(-1,n-1):
    result.append(abs(arr[i]-arr[i+1]))

print(min(result))