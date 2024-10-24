from fibo_rust import fibo
import time

before = time.time()
result = fibo(40)
after = time.time()
print(f"Result in Python with Rust after {after-before:.2f} seconds is: {result}.")
