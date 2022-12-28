#!/bin/python3

import math
import os
import random
import re
import sys

#
# Complete the 'gradingStudents' function below.
#
# The function is expected to return an INTEGER_ARRAY.
# The function accepts INTEGER_ARRAY grades as parameter.
#

def check(n):
    if ((1 + n // 5) * 5) - n < 3:
        return ((1 + n // 5) * 5)
    else:
        return (n)

def gradingStudents(grades):
    # Write your code here
    a = []
    for i in grades:
        if i >= 38:
            a.append(check(i))
        else:
            a.append(i)
    return a

if __name__ == '__main__':
    # fptr = open(os.environ['OUTPUT_PATH'], 'w')

    grades_count = int(input().strip())

    grades = []

    for _ in range(grades_count):
        grades_item = int(input().strip())
        grades.append(grades_item)

    result = gradingStudents(grades)

    print(result)
    # fptr.write('\n'.join(map(str, result)))
    # fptr.write('\n')
    #
    # fptr.close()



