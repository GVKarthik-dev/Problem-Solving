import numpy as np

# arr = [5,4,4,2,2,8]
arr = [1,2,3,4,3,3,2,1]



def cutTheSticks(arr):
    # Write your code here
    result = []
    arr = np.array(arr)
    while any(arr):
        arr = arr[arr != 0]
        result.append(len(arr))
        arr -= min(arr)
    return result


print(cutTheSticks(arr))


