#!/bin/python3

import math
import os
import random
import re
import sys

#
# Complete the 'pickingNumbers' function below.
#
# The function is expected to return an INTEGER.
# The function accepts INTEGER_ARRAY a as parameter.
#

def pickingNumbers(a,n):
    # Write your code here
    a.sort()
    ans = 0
    for i in range(n):
        for j in range(n):
            if i == j :
                continue
            elif abs(a[i] - a[j]) <= 1:
                ans = max(ans,j-i + 1)
    return ans

if __name__ == '__main__':
    fptr = open(os.environ['OUTPUT_PATH'], 'w')

    n = int(input().strip())

    a = list(map(int, input().rstrip().split()))

    result = pickingNumbers(a,n)

    fptr.write(str(result) + '\n')

    fptr.close()
