cube = lambda x: x**3# complete the lambda function

def fibonacci(n):
    # return a list of fibonacci numbers
    fib = []
    for i in range(n):
        if i<=1:
            fib.append(i)
        elif i>1:
            k = fib[-1]+fib[-2]
            fib.append(k)
    return fib




if __name__ == '__main__':
    n = int(input())
    print(list(map(cube, fibonacci(n))))