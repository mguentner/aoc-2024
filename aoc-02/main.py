#!/usr/bin/env python

import copy

example = """
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"""

def is_safe(numbers):
    last = 0
    last_diff = 0
    for idx, n in enumerate(numbers):
        if idx > 0:
            diff = last - n
            if abs(diff) == 0 or abs(diff) > 3:
                return False
            if (diff < 0 and last_diff > 0) or (diff > 0 and last_diff < 0):
                return False
            last_diff = diff
        last = n
    return True

def is_safe_tolerate(numbers):
    if is_safe(numbers):
        return True
    else:
        for x in range(len(numbers)):
            copied = copy.deepcopy(numbers)
            del copied[x]
            if is_safe(copied):
                return True
        return False

def count_safe(input, safe_fn):
    safe = 0
    for line in input.splitlines():
        if len(line) == 0:
            continue
        numbers = [ int(x) for x in line.split(" ") ]
        if safe_fn(numbers):
            safe = safe + 1
    return safe

print(count_safe(example, is_safe))
with open("data.txt") as data:
    print(count_safe(data.read(), is_safe))

print(count_safe(example, is_safe_tolerate))
with open("data.txt") as data:
    print(count_safe(data.read(), is_safe_tolerate))