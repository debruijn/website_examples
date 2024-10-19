import time

def fibo(n):
    if n == 1:
        return 1
    if n == 2:
        return 1
    return fibo(n-2) + fibo(n-1)

before = time.time()
result = fibo(40)
after = time.time()
print(f"Result in Python after {after-before:.2f} seconds is: {result}.")
