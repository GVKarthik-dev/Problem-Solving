#!/bin/python3

import math
import os
import random
import re
import sys

#
# Complete the 'countingValleys' function below.
#
# The function is expected to return an INTEGER.
# The function accepts following parameters:
#  1. INTEGER steps
#  2. STRING path
#

def countingValleys(steps, path):
    # Write your code here
    count = 0
    path = list(path)
    for i in range(len(path)-1):
        if path[i] > path[i+1]:
            count += 1
        elif path[i] < path[i+1]:
            count -=1
    return count

if __name__ == '__main__':
    # fptr = open(os.environ['OUTPUT_PATH'], 'w')

    steps = int(input().strip())

    path = input()

    # result = countingValleys(steps, path)

    print(list(path).count('D'))

    # print(result)

    # fptr.write(str(result) + '\n')
    #
    # fptr.close()
