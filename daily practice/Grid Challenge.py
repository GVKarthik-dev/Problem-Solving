#!/bin/python3

import math
import os
import random
import re
import sys

#
# Complete the 'gridChallenge' function below.
#
# The function is expected to return a STRING.
# The function accepts STRING_ARRAY grid as parameter.
#
def test(arr):
    result = ['NO', 'NO', 'NO', 'NO', 'YES', 'YES', 'YES', 'NO', 'YES', 'NO', 'NO', 'NO', 'NO', 'NO', 'YES', 'YES', 'NO', 'NO', 'NO', 'YES', 'NO', 'NO', 'NO', 'YES', 'YES', 'NO', 'YES', 'NO', 'YES', 'YES', 'NO', 'NO', 'NO', 'YES', 'NO', 'YES', 'NO', 'NO', 'YES', 'NO', 'YES', 'YES', 'NO', 'NO', 'YES', 'NO', 'NO', 'NO', 'NO', 'NO', 'NO', 'YES', 'YES', 'NO', 'YES', 'NO', 'NO', 'NO', 'YES', 'NO', 'YES', 'YES', 'NO', 'YES', 'YES', 'NO', 'YES', 'YES', 'NO', 'YES', 'YES', 'YES', 'NO', 'YES', 'YES', 'NO', 'NO', 'YES', 'YES', 'NO', 'YES', 'YES', 'NO', 'YES', 'NO', 'YES', 'NO', 'YES', 'NO', 'NO', 'NO', 'YES', 'YES', 'NO', 'NO', 'NO', 'YES', 'YES', 'NO', 'YES']
    for i in range(len(arr)):
        if result[i] != arr[i]:
            print(i)

def gridChallenge(n,grid):
    # Write your code here
    for j in range(n-1):
        for k in range(n-1):
            if grid[j][k] > grid[j+1][k]:
                return 'NO'
    return 'YES'

if __name__ == '__main__':
    t = int(input().strip())
    output = []

    for t_itr in range(t):
        n = int(input().strip())
        grid = []

        for _ in range(n):
            grid_item = list(input().strip())
            grid_item.sort()
            grid.append(grid_item)

        result = gridChallenge(n,grid)
        output.append(result)

    test(output)