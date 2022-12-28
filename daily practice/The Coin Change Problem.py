#!/bin/python3

import math
import os
import random
import re
import sys

#
# Complete the 'getWays' function below.
#
# The function is expected to return a LONG_INTEGER.
# The function accepts following parameters:
#  1. INTEGER n
#  2. LONG_INTEGER_ARRAY c
#

def getWays(n, c):
    # Write your code here
    c.sort()
    count = 0
    for i in c:
        t = n%i
        if (t == 0 or t%i == 0)  and n>=i  :
            count+=1

        else:
            break

    return count, c

if __name__ == '__main__':
    # fptr = open(os.environ['OUTPUT_PATH'], 'w')
    #
    # first_multiple_input = input().rstrip().split()
    #
    # n = int(first_multiple_input[0])
    #
    # m = int(first_multiple_input[1])
    #
    # c = list(map(int, input().rstrip().split()))


    a = [[],[4,1,2,3],[10,2,5,3,6],[3,8,3,1,2],[2,1,2]]

    n,*c = a[4]


    # Print the number of ways of making change for 'n' units using coins having the values given by 'c'

    ways = getWays(n, c)




    print(ways)

    # fptr.write(str(ways) + '\n')
    #
    # fptr.close()

