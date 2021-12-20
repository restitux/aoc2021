#!/usr/bin/env python3

# 10, 21, 33, 46
# 11, 23, 36, 50
# 12, 23, 39, 54
# 12, 23, 39, 54

x = 0
y = 0
x_v = 7
y_v = -1


for i in range(0, 1000):
    print(f"{x}, {y}")
    x += x_v
    y += y_v
    if x_v > 0:
        x_v -= 1
    y_v -= 1
