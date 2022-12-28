#!/bin/python3

import math
import os
import random
import re
import sys
import datetime
import time
#
# Complete the 'timeConversion' function below.
#
# The function is expected to return a STRING.
# The function accepts STRING s as parameter.
#

def timeConversion(s):
    # Write your code here
    in_time = datetime.datetime.strptime(s,"%I:%M:%S%p")
    a = datetime.datetime.strftime(in_time,"%H:%M:%S")
    return a

if __name__ == '__main__':
    # fptr = open(os.environ['OUTPUT_PATH'], 'w')

    s = input()

    result = timeConversion(s)

    print(result)
    # fptr.write(result + '\n')
    #
    # fptr.close()
