#!/usr/bin/env python3
"""Python baseline implementations for basic functions."""

def identity(n: int) -> int:
    return n

def add_one(n: int) -> int:
    return n + 1

def double(n: int) -> int:
    return n * 2

def square(n: int) -> int:
    return n * n

def max_two(a: int, b: int) -> int:
    return a if a > b else b

def min_two(a: int, b: int) -> int:
    return a if a < b else b

def abs_val(n: int) -> int:
    return -n if n < 0 else n

def sign(n: int) -> int:
    if n < 0:
        return -1
    elif n > 0:
        return 1
    else:
        return 0

def is_even(n: int) -> bool:
    return n % 2 == 0

def is_positive(n: int) -> bool:
    return n > 0


if __name__ == "__main__":
    # Test cases
    assert identity(42) == 42
    assert add_one(41) == 42
    assert double(21) == 42
    assert square(5) == 25
    assert max_two(3, 7) == 7
    assert min_two(3, 7) == 3
    assert abs_val(-5) == 5
    assert sign(-5) == -1
    assert is_even(4) == True
    assert is_positive(-3) == False
    print("All basic tests passed!")
