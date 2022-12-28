#!/bin/python3

import math
import os
import random
import re
import sys


#
# Complete the 'findDigits' function below.
#
# The function is expected to return an INTEGER.
# The function accepts INTEGER n as parameter.
#

def findDigits(n):
    # Write your code here
    s = 0
    for i in str(n):
        if i != '0':
            if n % int(i) == 0:
                s += 1
    return s


if __name__ == '__main__':
    # fptr = open(os.environ['OUTPUT_PATH'], 'w')

    t = int(input().strip())

    for t_itr in range(t):
        n = int(input().strip())

        result = findDigits(n)

        print(result)

    #     fptr.write(str(result) + '\n')
    #
    # fptr.close()
